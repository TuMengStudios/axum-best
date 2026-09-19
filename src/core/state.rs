use std::sync::Arc;

use crate::conf::AppConf;
use crate::services::foo::FooService;
use crate::services::user::UserService;

/// Application state: owns all services and the application configuration.
///
/// Dependency direction: handlers -> services -> repos(trait) <- data(implementations).
/// Infrastructure such as connection pools is owned by the data-layer repository
/// implementations and no longer appears on AppState.
/// The configuration is shared via `Arc<AppConf>`: AppState is cloned on every request,
/// and the Arc ensures cloning only bumps the reference count instead of copying the config.
#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<AppConf>,
    pub user_service: UserService,
    pub foo_service: FooService,
}

impl AppState {
    pub fn new(cfg: Arc<AppConf>, user_service: UserService, foo_service: FooService) -> AppState {
        AppState {
            cfg,
            user_service,
            foo_service,
        }
    }
}
