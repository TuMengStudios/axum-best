use async_trait::async_trait;

use crate::core::rest::AppError;

/// Session information returned by WeChat `jscode2session`.
///
/// `session_key` is only used for server-side verification/decryption and must never be returned to clients or written to logs.
#[derive(Debug, Clone)]
pub struct WechatSession {
    pub app_id: String,
    pub open_id: String,
    pub session_key: String,
    pub union_id: Option<String>,
}

/// WeChat mini-program API contract: exchanges a login code for session information.
#[async_trait]
pub trait WechatRepo: Send + Sync {
    /// Calls `jscode2session` to exchange the login code for the user's OpenID.
    async fn exchange_login_code(&self, code: &str) -> Result<WechatSession, AppError>;
}
