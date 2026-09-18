use async_trait::async_trait;

use crate::core::rest::AppError;

/// 微信小程序接口契约：根据登录 code 换取用户 OpenID。
#[async_trait]
pub trait WechatRepo: Send + Sync {
    async fn open_id(&self, code: &str) -> Result<String, AppError>;
}
