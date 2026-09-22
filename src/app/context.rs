use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use sqlx::MySqlPool;
use tracing::error;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::conf::AppConf;
use crate::core::state::AppState;
use crate::core::worker_dispatcher::WorkerDispatcher;
use crate::data::kv::RedisKvStore;
use crate::data::user::MySqlUserRepo;
use crate::data::wechat::WechatApiRepo;
use crate::observability::{self, OpenTelemetryGuard};
use crate::routers;
use crate::services::foo::FooService;
use crate::services::user::UserService;

/// Application context: the single assembly point of the whole app
///
/// Owns everything long-lived: the shared config (`Arc<AppConf>`), the HTTP
/// server (`Router`, which owns the fully wired `AppState` after assembly)
/// and a MySQL pool handle for deterministic shutdown. `new` assembles the
/// dependency graph, `start` binds the listener and serves requests.
///
/// There is deliberately no `app_state` field. The assembled `AppState` is
/// moved into the router, and every consumer receives it as a parameter at
/// its own entry point: handlers extract it via `State<AppState>`, and
/// background tasks take a clone at spawn time. A stored copy would have no
/// consumer, and would give the redis pool a second owner, weakening the
/// drop-order guarantee documented on `work_guard`. If code ever genuinely
/// needs to reach services after assembly, add a purpose-built handle for
/// that specific need rather than a general accessor.
///
/// The one exception is `worker`, that purpose-built handle for background
/// work: the shared `WorkerDispatcher` is owned here so tasks can be submitted
/// across the whole lifecycle — startup paths in this context, handlers via
/// the clone inside `AppState`, and future shutdown work — all backed by one
/// pool.
///
/// The bb8 redis pool has no close API and lives solely inside its
/// repository (part of the router's state); dropping the context tears it
/// down.
pub struct AppContext {
    cfg: Arc<AppConf>,
    router: Router,
    db_pool: MySqlPool,
    /// Shared fire-and-forget task pool; a clone lives inside `AppState`.
    /// Kept on the context so startup and shutdown paths can submit tasks too.
    #[allow(dead_code)]
    worker: WorkerDispatcher,
    /// Keeps the OTLP exporter alive until the application shuts down.
    /// Declared before `work_guard` so exporter shutdown can still write diagnostics.
    #[allow(dead_code)]
    otel_guard: OpenTelemetryGuard,
    /// Guard for the non-blocking log worker; must be kept alive for the
    /// lifetime of the server so log flushing on drop happens correctly.
    /// Declared last so OpenTelemetry flushes before the log worker stops.
    #[allow(dead_code)]
    work_guard: WorkerGuard,
}

impl AppContext {
    /// Assembles logging, connection pools, repositories, services and the
    /// HTTP router into one context
    pub async fn new(cfg: AppConf) -> anyhow::Result<AppContext> {
        let otel_guard = observability::init_open_telemetry(env!("CARGO_PKG_NAME"), &cfg.otel)?;
        let guard = cfg.log.init_log(otel_guard.tracer())?;
        let cfg = Arc::new(cfg);

        // build the shared background task pool; it only depends on cfg, so
        // it is ready before any connection pool
        let worker = WorkerDispatcher::new(&cfg.worker);

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

        // Assemble dependencies: data-layer implementations -> inject into services -> build AppState -> build the HTTP server
        let user_service = UserService::new(
            Arc::new(MySqlUserRepo::new(db_conn.clone())),
            Arc::new(RedisKvStore::new(redis_client.clone())),
            Arc::new(WechatApiRepo::new(&cfg.wechat)),
        );
        let app_state = AppState::new(cfg.clone(), worker.clone(), user_service, FooService);
        let router = routers::app_routers(app_state);

        let app = AppContext {
            cfg,
            router,
            db_pool: db_conn,
            worker,
            otel_guard,
            work_guard: guard,
        };
        Ok(app)
    }

    /// Binds the HTTP listener and serves requests until the server exits
    ///
    /// Graceful shutdown: stops accepting new connections and drains
    /// in-flight ones when Ctrl+C (all platforms) or SIGTERM (Unix, what
    /// `docker stop` sends) arrives. After `axum::serve` returns, the pools
    /// are closed and dropping the context flushes logs last
    /// (field drop order).
    ///
    /// # Returns
    /// - `Ok(())` if the server starts and runs to completion
    /// - `Err` if the listener cannot be built or serving fails
    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = self.cfg.http.build_listener().await?;
        info!("Server started successfully");
        println!("Server started successfully");

        axum::serve(
            listener,
            self.router
                .clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

        self.close_pools().await;
        info!("Server stopped");
        Ok(())
    }

    /// Closes connection pools once the HTTP server has drained
    ///
    /// sqlx supports deterministic async close. The bb8 redis pool has no
    /// close API; it is torn down by dropping the context (its only handle
    /// lives inside the repository in `app_state`).
    async fn close_pools(&self) {
        self.db_pool.close().await;
        info!("connection pools closed");
    }
}

/// Resolves when the process is asked to shut down: Ctrl+C on all platforms,
/// plus SIGTERM on Unix (`docker stop`, systemd, k8s).
///
/// Signal-handler installation failures degrade to a never-resolving future
/// (logged as errors) instead of panicking, matching the no-unwrap/expect
/// convention of this codebase.
async fn shutdown_signal() {
    let ctrl_c = async {
        if tokio::signal::ctrl_c().await.is_err() {
            error!("failed to install Ctrl+C handler");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(err) => {
                error!("failed to install SIGTERM handler {}", err);
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("shutdown signal received, draining in-flight requests"),
        _ = terminate => info!("shutdown signal received, draining in-flight requests"),
    }
}
