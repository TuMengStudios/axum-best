use async_trait::async_trait;

use crate::core::rest::AppError;
use crate::models::OAuthAccount;
use crate::models::UserInfo;

/// Type-safe description of a partial user update.
#[derive(Debug, Clone)]
pub enum UserUpdate {
    NickName(String),
    Avatar(String),
    Signature(String),
    Age(u8),
    Phone(String),
    Salt(String),
    Password(String),
    Status(i8),
    /// Retained for API compatibility; `updated_at` is always written by the
    /// data layer with the current timestamp.
    UpdatedAt(i64),
}

/// User repository trait: defines only the data-access contract; concrete implementations are provided by the data layer.
///
/// The service layer depends only on this trait (`Arc<dyn UserRepo>`) and is unaware of the
/// underlying storage, so mocks can be injected in unit tests.
#[async_trait]
pub trait UserRepo: Send + Sync {
    /// Creates a user
    async fn create(&self, user: &mut UserInfo) -> Result<(), AppError>;

    /// Updates user information
    async fn update(&self, user: &UserInfo) -> Result<(), AppError>;

    /// Gets a user by ID
    async fn get_by_id(&self, id: i64) -> Result<UserInfo, AppError>;

    /// Gets a user by phone number
    async fn get_by_phone(&self, phone: &str) -> Result<UserInfo, AppError>;

    /// Gets a user by third-party identity
    async fn get_by_oauth(
        &self,
        provider: &str,
        provider_app_id: &str,
        sub_id: &str,
    ) -> Result<Option<UserInfo>, AppError>;

    /// Creates a third-party identity link for a user
    async fn create_oauth_account(&self, account: &mut OAuthAccount) -> Result<(), AppError>;

    /// Creates a user and its third-party identity link in the same transaction
    async fn create_user_with_oauth(
        &self,
        user: &mut UserInfo,
        account: &mut OAuthAccount,
    ) -> Result<(), AppError>;

    /// Soft-deletes a user (sets the deleted_at timestamp)
    async fn delete(&self, id: i64, deleted_at: i64) -> Result<(), AppError>;

    /// Hard-deletes a user (removes the row from the database)
    async fn hard_delete(&self, id: i64) -> Result<(), AppError>;

    /// Lists users (paginated query)
    async fn list(&self, page: u32, page_size: u32) -> Result<Vec<UserInfo>, AppError>;

    /// Counts users
    async fn count(&self) -> Result<i64, AppError>;

    /// Searches users by nickname
    async fn search_by_nickname(
        &self,
        nickname: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserInfo>, AppError>;

    /// Updates part of a user's information (builds the update statement dynamically)
    async fn update_partial(&self, id: i64, updates: &[UserUpdate]) -> Result<(), AppError>;
}
