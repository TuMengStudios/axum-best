use async_trait::async_trait;
use sqlx::MySqlPool;
use sqlx::QueryBuilder;

use super::mysql::covert_error;
use crate::core::rest::AppError;
use crate::models::oauth::OAuthAccount;
use crate::models::user::UserInfo;
use crate::repos::user::UserRepo;
use crate::repos::user::UserUpdate;

/// User repository implementation backed by MySQL/SQLx
pub struct MySqlUserRepo {
    pool: MySqlPool,
}

impl MySqlUserRepo {
    pub fn new(pool: MySqlPool) -> MySqlUserRepo {
        MySqlUserRepo { pool }
    }
}

#[async_trait]
impl UserRepo for MySqlUserRepo {
    /// Creates a user
    async fn create(&self, user: &mut UserInfo) -> Result<(), AppError> {
        user.id = sqlx::query_as!(UserInfo,
            r#"INSERT INTO user_info (nick_name, avatar, signature, age, phone, salt, password, created_at, updated_at, deleted_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            user.nick_name,
            user.avatar,
            user.signature,
            user.age,
            user.phone,
            user.salt,
            user.password,
            user.created_at,
            user.updated_at,
            user.deleted_at
        )
        .execute(&self.pool)
        .await
        .map_err(covert_error)?
        .last_insert_id() as i64;

        Ok(())
    }

    /// Updates user information
    async fn update(&self, user: &UserInfo) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE user_info SET
               nick_name = ?, avatar = ?, signature = ?, age = ?, phone = ?,
               salt = ?, password = ?, updated_at = ?
               WHERE id = ?"#,
            user.nick_name,
            user.avatar,
            user.signature,
            user.age,
            user.phone,
            user.salt,
            user.password,
            user.updated_at,
            user.id
        )
        .execute(&self.pool)
        .await
        .map_err(covert_error)?;

        Ok(())
    }

    /// Gets a user by ID
    async fn get_by_id(&self, id: i64) -> Result<UserInfo, AppError> {
        let user = sqlx::query_as!(
            UserInfo,
            r#"SELECT id, nick_name, avatar, signature, age, phone, salt, password,
                      created_at, updated_at, deleted_at
               FROM user_info WHERE id = ?"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(covert_error)?;
        Ok(user)
    }

    /// Gets a user by phone number
    async fn get_by_phone(&self, phone: &str) -> Result<UserInfo, AppError> {
        let user = sqlx::query_as!(
            UserInfo,
            r#"SELECT id, nick_name, avatar, signature, age, phone, salt, password,
                      created_at, updated_at, deleted_at
               FROM user_info WHERE phone = ?"#,
            phone
        )
        .fetch_one(&self.pool)
        .await
        .map_err(covert_error)?;
        Ok(user)
    }

    /// Gets a user by third-party identity
    async fn get_by_oauth(
        &self,
        provider: &str,
        provider_app_id: &str,
        sub_id: &str,
    ) -> Result<Option<UserInfo>, AppError> {
        sqlx::query_as::<_, UserInfo>(
            r#"SELECT u.id, u.nick_name, u.avatar, u.signature, u.age, u.phone,
                      u.salt, u.password, u.created_at, u.updated_at, u.deleted_at
               FROM user_info u
               INNER JOIN user_oauth_account a ON a.user_id = u.id
               WHERE a.provider = ? AND a.provider_app_id = ?
                 AND a.sub_id = ? AND u.deleted_at = 0"#,
        )
        .bind(provider)
        .bind(provider_app_id)
        .bind(sub_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(covert_error)
    }

    async fn create_oauth_account(&self, account: &mut OAuthAccount) -> Result<(), AppError> {
        account.id = sqlx::query(
            r#"INSERT INTO user_oauth_account
               (user_id, provider, provider_app_id, sub_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(account.user_id)
        .bind(&account.provider)
        .bind(&account.provider_app_id)
        .bind(&account.sub_id)
        .bind(account.created_at)
        .bind(account.updated_at)
        .execute(&self.pool)
        .await
        .map_err(covert_error)?
        .last_insert_id() as i64;
        Ok(())
    }

    async fn create_user_with_oauth(
        &self,
        user: &mut UserInfo,
        account: &mut OAuthAccount,
    ) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(covert_error)?;

        user.id = sqlx::query(
            r#"INSERT INTO user_info
               (nick_name, avatar, signature, age, phone, salt, password,
                created_at, updated_at, deleted_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&user.nick_name)
        .bind(&user.avatar)
        .bind(&user.signature)
        .bind(user.age)
        .bind(&user.phone)
        .bind(&user.salt)
        .bind(&user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.deleted_at)
        .execute(&mut *tx)
        .await
        .map_err(covert_error)?
        .last_insert_id() as i64;

        account.user_id = user.id;
        account.id = sqlx::query(
            r#"INSERT INTO user_oauth_account
               (user_id, provider, provider_app_id, sub_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(account.user_id)
        .bind(&account.provider)
        .bind(&account.provider_app_id)
        .bind(&account.sub_id)
        .bind(account.created_at)
        .bind(account.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(covert_error)?
        .last_insert_id() as i64;

        tx.commit().await.map_err(covert_error)?;
        Ok(())
    }

    /// Soft-deletes a user (sets the deleted_at timestamp)
    async fn delete(&self, id: i64, deleted_at: i64) -> Result<(), AppError> {
        sqlx::query!(r#"UPDATE user_info SET deleted_at = ? WHERE id = ?"#, deleted_at, id)
            .execute(&self.pool)
            .await
            .map_err(covert_error)?;

        Ok(())
    }

    /// Hard-deletes a user (removes the row from the database)
    async fn hard_delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query!(r#"DELETE FROM user_info WHERE id = ?"#, id)
            .execute(&self.pool)
            .await
            .map_err(covert_error)?;

        Ok(())
    }

    /// Lists users (paginated query)
    async fn list(&self, page: u32, page_size: u32) -> Result<Vec<UserInfo>, AppError> {
        let offset = (page - 1) * page_size;
        let users = sqlx::query_as!(
            UserInfo,
            r#"SELECT id, nick_name, avatar, signature, age, phone, salt, password,
                      created_at, updated_at, deleted_at
               FROM user_info WHERE deleted_at = 0 ORDER BY id DESC LIMIT ? OFFSET ?"#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(covert_error)?;

        Ok(users)
    }

    /// Counts users
    async fn count(&self) -> Result<i64, AppError> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM user_info WHERE deleted_at = 0"#)
            .fetch_one(&self.pool)
            .await
            .map_err(covert_error)?;

        Ok(count)
    }

    /// Searches users by nickname
    async fn search_by_nickname(
        &self,
        nickname: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserInfo>, AppError> {
        let offset = (page - 1) * page_size;
        let search_pattern = format!("%{}%", nickname);

        let users = sqlx::query_as!(
            UserInfo,
            r#"SELECT id, nick_name, avatar, signature, age, phone, salt, password,
                      created_at, updated_at, deleted_at
               FROM user_info
               WHERE nick_name LIKE ? AND deleted_at = 0
               ORDER BY id DESC LIMIT ? OFFSET ?"#,
            search_pattern,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(covert_error)?;

        Ok(users)
    }

    /// Updates part of a user's information (builds the update statement dynamically with QueryBuilder)
    async fn update_partial(&self, id: i64, updates: &[UserUpdate]) -> Result<(), AppError> {
        if updates.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new("UPDATE user_info SET ");

        for (i, update) in updates.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }

            match update {
                UserUpdate::NickName(value) => query_builder.push("nick_name = ").push_bind(value),
                UserUpdate::Avatar(value) => query_builder.push("avatar = ").push_bind(value),
                UserUpdate::Signature(value) => query_builder.push("signature = ").push_bind(value),
                UserUpdate::Age(value) => query_builder.push("age = ").push_bind(value),
                UserUpdate::Phone(value) => query_builder.push("phone = ").push_bind(value),
                UserUpdate::Salt(value) => query_builder.push("salt = ").push_bind(value),
                UserUpdate::Password(value) => query_builder.push("password = ").push_bind(value),
                UserUpdate::UpdatedAt(value) => {
                    query_builder.push("updated_at = ").push_bind(value)
                }
            };
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        query_builder
            .build()
            .execute(&self.pool)
            .await
            .map_err(covert_error)?;

        Ok(())
    }
}
