use async_trait::async_trait;

use crate::core::rest::AppError;
use crate::models::oauth::OAuthAccount;
use crate::models::user::UserInfo;

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
    UpdatedAt(i64),
}

/// 用户仓储接口：只定义数据访问契约，具体实现由 data 层提供
///
/// service 层仅依赖本接口（`Arc<dyn UserRepo>`），不感知底层存储，
/// 单测时可注入 mock 实现。
#[async_trait]
pub trait UserRepo: Send + Sync {
    /// 创建用户
    async fn create(&self, user: &mut UserInfo) -> Result<(), AppError>;

    /// 更新用户信息
    async fn update(&self, user: &UserInfo) -> Result<(), AppError>;

    /// 根据ID获取用户
    async fn get_by_id(&self, id: i64) -> Result<UserInfo, AppError>;

    /// 根据手机号获取用户
    async fn get_by_phone(&self, phone: &str) -> Result<UserInfo, AppError>;

    /// 根据第三方身份获取用户
    async fn get_by_oauth(
        &self,
        provider: &str,
        provider_app_id: &str,
        sub_id: &str,
    ) -> Result<Option<UserInfo>, AppError>;

    /// 创建用户的第三方身份关联
    async fn create_oauth_account(&self, account: &mut OAuthAccount) -> Result<(), AppError>;

    /// 在同一个事务中创建用户及其第三方身份关联
    async fn create_user_with_oauth(
        &self,
        user: &mut UserInfo,
        account: &mut OAuthAccount,
    ) -> Result<(), AppError>;

    /// 软删除用户（设置deleted_at时间戳）
    async fn delete(&self, id: i64, deleted_at: i64) -> Result<(), AppError>;

    /// 硬删除用户（从数据库中完全删除）
    async fn hard_delete(&self, id: i64) -> Result<(), AppError>;

    /// 获取用户列表（分页查询）
    async fn list(&self, page: u32, page_size: u32) -> Result<Vec<UserInfo>, AppError>;

    /// 获取用户总数
    async fn count(&self) -> Result<i64, AppError>;

    /// 根据昵称搜索用户
    async fn search_by_nickname(
        &self,
        nickname: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserInfo>, AppError>;

    /// 更新用户部分信息（动态构建更新语句）
    async fn update_partial(&self, id: i64, updates: &[UserUpdate]) -> Result<(), AppError>;
}
