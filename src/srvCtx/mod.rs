use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::conf::AppConf;
use crate::core::state::AppState;
use crate::routers;

/// Server context that holds application configuration and state
///
/// This struct encapsulates the core components needed to run the HTTP server,
/// including configuration settings and application state (database connections, cache, etc.)
pub struct ServeContext {
    /// Application configuration instance containing all settings
    cfg: AppConf,
    /// Application state instance containing database connections and other shared resources
    app_state: AppState,
    work_guard: WorkerGuard,
}

impl ServeContext {
    pub async fn new(cfg: AppConf) -> anyhow::Result<ServeContext> {
        let guard = cfg.log.init_log()?;

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
        let wechat = cfg.wechat.clone();
        let res = ServeContext {
            work_guard: guard,
            cfg,
            app_state: AppState::new(db_conn.clone(), redis_client.clone(), wechat),
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
