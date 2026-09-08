use std::sync::Arc;

use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::conf::AppConf;
use crate::core::state::AppState;
use crate::data::kv_store::RedisKvStore;
use crate::data::user_repo::MySqlUserRepo;
use crate::routers;
use crate::services::foo::FooService;
use crate::services::user::UserService;

/// Server context that holds application configuration and state
///
/// This struct encapsulates the core components needed to run the HTTP server,
/// including configuration settings and application state (services, etc.)
pub struct ServeContext {
    /// Application configuration instance containing all settings,
    /// shared with AppState via Arc so both sides see the same config
    cfg: Arc<AppConf>,
    /// Application state instance containing all services
    app_state: AppState,
    /// Guard for the non-blocking log worker; must be kept alive for the lifetime
    /// of the server so log flushing on drop happens correctly.
    #[allow(dead_code)]
    work_guard: WorkerGuard,
}

impl ServeContext {
    pub async fn new(cfg: AppConf) -> anyhow::Result<ServeContext> {
        let guard = cfg.log.init_log()?;
        let cfg = Arc::new(cfg);

        // build db connect pool
        let db_conn = cfg
            .mysql
            .init_conn()
            .await
            .map_err(|err| anyhow::anyhow!("build mysql connect pool error {}", err))?;

        // build cache client
        let redis_client = cfg
            .redis
            .init_pool()
            .await
            .map_err(|err| anyhow::anyhow!("build redis client error {}", err))?;

        // 组装依赖：data 层实现 -> 注入 service -> 装入 AppState
        let user_service = UserService::new(
            Arc::new(MySqlUserRepo::new(db_conn)),
            Arc::new(RedisKvStore::new(redis_client)),
        );

        let res = ServeContext {
            work_guard: guard,
            app_state: AppState::new(cfg.clone(), user_service, FooService),
            cfg,
        };
        Ok(res)
    }
}

impl ServeContext {
    /// Starts the HTTP server and begins serving requests
    ///
    /// This method performs the following steps:
    /// 1. Initializes the logging system
    /// 2. Creates the application router with the app state
    /// 3. Builds the HTTP listener
    /// 4. Starts serving HTTP requests
    ///
    /// # Returns
    /// - `Ok(())` if the server starts successfully
    /// - `Err` if any step fails
    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Create application router
        let app = routers::app_routers(self.app_state.clone());
        let listener = self.cfg.http.build_listener().await?;
        axum::serve::serve(listener, app).await?;
        println!("Server started successfully");
        info!("Server started successfully");
        Ok(())
    }
}
