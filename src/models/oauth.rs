use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

/// An external authentication identity linked to a local user.
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct OAuthAccount {
    pub id: i64,
    pub user_id: i64,
    pub provider: String,
    pub provider_app_id: String,
    /// Provider-specific user identifier normalized to one field.
    /// Examples: WeChat `openid`, OIDC `sub`, GitHub user id.
    pub sub_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl OAuthAccount {
    pub fn new(
        user_id: i64,
        provider: impl Into<String>,
        provider_app_id: impl Into<String>,
        sub_id: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            user_id,
            provider: provider.into(),
            provider_app_id: provider_app_id.into(),
            sub_id: sub_id.into(),
            created_at: now,
            updated_at: now,
        }
    }
}
