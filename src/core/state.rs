use std::sync::Arc;

use crate::conf::AppConf;
use crate::services::foo::FooService;
use crate::services::user::UserService;

/// 应用状态：持有所有 service 与应用配置
///
/// 依赖方向：handlers -> services -> repos(trait) <- data(实现)。
/// 连接池等基础设施由 data 层的仓储实现持有，不再出现在 AppState 上。
/// 配置以 `Arc<AppConf>` 共享：AppState 每个请求都会被克隆，
/// Arc 保证克隆只增加引用计数，不复制整份配置。
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
