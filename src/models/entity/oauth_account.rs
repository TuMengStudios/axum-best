use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// `oauth_account` table entity and domain model.
///
/// SeaORM requires the model struct to be named `Model`; it is re-exported as
/// `OAuthAccount` from `models`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "oauth_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Required relation to `user_info.id`; no database default.
    pub user_id: i64,
    /// Required identity provider; no database default.
    pub provider: String,
    #[sea_orm(default_value = "")]
    pub provider_app_id: String,
    /// Required provider-side identifier; no database default.
    pub sub_id: String,
    #[sea_orm(default_value = 0)]
    pub created_at: i64,
    #[sea_orm(default_value = 0)]
    pub updated_at: i64,
}

impl Model {
    pub fn new(
        user_id: i64,
        provider: impl Into<String>,
        provider_app_id: impl Into<String>,
        sub_id: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Fills unset timestamps with the current Unix epoch milliseconds.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert && self.created_at.is_not_set() {
            self.created_at = Set(chrono::Utc::now().timestamp_millis());
        }
        self.updated_at = Set(chrono::Utc::now().timestamp_millis());
        Ok(self)
    }
}
