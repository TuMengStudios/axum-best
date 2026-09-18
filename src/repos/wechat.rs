use async_trait::async_trait;

use crate::core::rest::AppError;

/// 微信小程序接口契约：根据登录 code 换取用户 OpenID。
#[async_trait]
pub trait WechatRepo: Send + Sync {
    /// 使用微信登录 code 换取用户 OpenID。
    async fn exchange_code_for_open_id(&self, code: &str) -> Result<String, AppError>;
}
