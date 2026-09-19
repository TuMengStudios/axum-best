use async_trait::async_trait;

use crate::core::rest::AppError;

/// 微信 `jscode2session` 返回的会话信息。
///
/// `session_key` 只供服务端后续校验/解密使用，禁止返回给客户端或写入日志。
#[derive(Debug, Clone)]
pub struct WechatSession {
    pub app_id: String,
    pub open_id: String,
    pub session_key: String,
    pub union_id: Option<String>,
}

/// 微信小程序接口契约：使用登录 code 换取会话信息。
#[async_trait]
pub trait WechatRepo: Send + Sync {
    /// 调用 `jscode2session`，使用登录 code 换取用户 OpenID。
    async fn exchange_login_code(&self, code: &str) -> Result<WechatSession, AppError>;
}
