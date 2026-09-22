use std::sync::Arc;

use crate::conf::AppConf;
use crate::core::worker_dispatcher::WorkerDispatcher;
use crate::services::foo::FooService;
use crate::services::user::UserService;

/// Application state: owns all services and the application configuration.
///
/// Dependency direction: handlers -> services -> repos(trait) <- data(implementations).
/// Infrastructure such as connection pools is owned by the data-layer repository
/// implementations and no longer appears on AppState. The exception is `worker`:
/// a cheap `Clone` handle over the shared background task pool, needed by
/// handlers and services to submit fire-and-forget work (the pool itself is
/// owned by `AppContext`).
/// The configuration is shared via `Arc<AppConf>`: AppState is cloned on every request,
/// and the Arc ensures cloning only bumps the reference count instead of copying the config.
#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<AppConf>,
    pub worker: WorkerDispatcher,
    pub user_service: UserService,
    pub foo_service: FooService,
}

impl AppState {
    pub fn new(
        cfg: Arc<AppConf>,
        worker: WorkerDispatcher,
        user_service: UserService,
        foo_service: FooService,
    ) -> AppState {
        AppState {
            cfg,
            worker,
            user_service,
            foo_service,
        }
    }
}
