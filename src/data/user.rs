use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;
use sea_orm::sea_query::Expr;

use super::db_error::covert_error;
use crate::core::rest::AppError;
use crate::models::OAuthAccount;
use crate::models::UserInfo;
use crate::models::entity::oauth_account as oauth_entity;
use crate::models::entity::user_info as user_entity;
use crate::repos::user::UserRepo;
use crate::repos::user::UserUpdate;

/// User repository implementation backed by MySQL/SeaORM
pub struct MySqlUserRepo {
    db: DatabaseConnection,
}

impl MySqlUserRepo {
    pub fn new(db: DatabaseConnection) -> MySqlUserRepo {
        MySqlUserRepo { db }
    }
}

/// Maps a domain user into an insertable active model (primary key left to MySQL).
fn active_model_for_create(user: &UserInfo) -> user_entity::ActiveModel {
    user_entity::ActiveModel {
        id: NotSet,
        nick_name: Set(user.nick_name.clone()),
        avatar: Set(user.avatar.clone()),
        signature: Set(user.signature.clone()),
        age: Set(user.age),
        phone: Set(user.phone.clone()),
        salt: Set(user.salt.clone()),
        password: Set(user.password.clone()),
        created_at: Set(user.created_at),
        updated_at: Set(user.updated_at),
        deleted_at: Set(user.deleted_at),
        status: Set(user.status),
    }
}

/// Maps a domain user into an updatable active model.
///
/// `created_at` / `deleted_at` are left untouched and `updated_at` is left for
/// the entity `before_save` hook, so the database always gets the current time.
fn active_model_for_update(user: &UserInfo) -> user_entity::ActiveModel {
    user_entity::ActiveModel {
        id: Set(user.id),
        nick_name: Set(user.nick_name.clone()),
        avatar: Set(user.avatar.clone()),
        signature: Set(user.signature.clone()),
        age: Set(user.age),
        phone: Set(user.phone.clone()),
        salt: Set(user.salt.clone()),
        password: Set(user.password.clone()),
        created_at: NotSet,
        updated_at: NotSet,
        deleted_at: NotSet,
        status: Set(user.status),
    }
}

/// Maps a domain oauth account into an insertable active model.
fn active_model_for_oauth(account: &OAuthAccount) -> oauth_entity::ActiveModel {
    oauth_entity::ActiveModel {
        id: NotSet,
        user_id: Set(account.user_id),
        provider: Set(account.provider.clone()),
        provider_app_id: Set(account.provider_app_id.clone()),
        sub_id: Set(account.sub_id.clone()),
        created_at: Set(account.created_at),
        updated_at: Set(account.updated_at),
    }
}

fn row_not_found() -> AppError {
    covert_error(sea_orm::DbErr::RecordNotFound("user row missing".to_string()))
}

#[async_trait]
impl UserRepo for MySqlUserRepo {
    /// Creates a user
    async fn create(&self, user: &mut UserInfo) -> Result<(), AppError> {
        let model = active_model_for_create(user)
            .insert(&self.db)
            .await
            .map_err(covert_error)?;
        user.id = model.id;
        Ok(())
    }

    /// Updates user information
    ///
    /// `updated_at` is refreshed by the entity `before_save` hook, ignoring the
    /// value carried by `user`.
    async fn update(&self, user: &UserInfo) -> Result<(), AppError> {
        active_model_for_update(user)
            .update_without_returning(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(())
    }

    /// Gets a user by ID
    async fn get_by_id(&self, id: i64) -> Result<UserInfo, AppError> {
        user_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(covert_error)?
            .ok_or_else(row_not_found)
    }

    /// Gets a user by phone number
    async fn get_by_phone(&self, phone: &str) -> Result<UserInfo, AppError> {
        user_entity::Entity::find()
            .filter(user_entity::Column::Phone.eq(phone))
            .one(&self.db)
            .await
            .map_err(covert_error)?
            .ok_or_else(row_not_found)
    }

    /// Gets a user by third-party identity
    async fn get_by_oauth(
        &self,
        provider: &str,
        provider_app_id: &str,
        sub_id: &str,
    ) -> Result<Option<UserInfo>, AppError> {
        let account = oauth_entity::Entity::find()
            .filter(oauth_entity::Column::Provider.eq(provider))
            .filter(oauth_entity::Column::ProviderAppId.eq(provider_app_id))
            .filter(oauth_entity::Column::SubId.eq(sub_id))
            .one(&self.db)
            .await
            .map_err(covert_error)?;

        let Some(account) = account else {
            return Ok(None);
        };

        let user = user_entity::Entity::find_by_id(account.user_id)
            .filter(user_entity::Column::DeletedAt.eq(0))
            .one(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(user)
    }

    async fn create_oauth_account(&self, account: &mut OAuthAccount) -> Result<(), AppError> {
        let model = active_model_for_oauth(account)
            .insert(&self.db)
            .await
            .map_err(covert_error)?;
        account.id = model.id;
        Ok(())
    }

    async fn create_user_with_oauth(
        &self,
        user: &mut UserInfo,
        account: &mut OAuthAccount,
    ) -> Result<(), AppError> {
        let tx = self.db.begin().await.map_err(covert_error)?;

        let model = active_model_for_create(user)
            .insert(&tx)
            .await
            .map_err(covert_error)?;
        user.id = model.id;

        account.user_id = user.id;
        let model = active_model_for_oauth(account)
            .insert(&tx)
            .await
            .map_err(covert_error)?;
        account.id = model.id;

        tx.commit().await.map_err(covert_error)?;
        Ok(())
    }

    /// Soft-deletes a user (sets the deleted_at timestamp)
    async fn delete(&self, id: i64, deleted_at: i64) -> Result<(), AppError> {
        user_entity::Entity::update_many()
            .col_expr(user_entity::Column::DeletedAt, Expr::value(deleted_at))
            .filter(user_entity::Column::Id.eq(id))
            .exec(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(())
    }

    /// Hard-deletes a user (removes the row from the database)
    async fn hard_delete(&self, id: i64) -> Result<(), AppError> {
        user_entity::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(())
    }

    /// Lists users (paginated query)
    async fn list(&self, page: u32, page_size: u32) -> Result<Vec<UserInfo>, AppError> {
        let offset = u64::from(page - 1) * u64::from(page_size);
        let users = user_entity::Entity::find()
            .filter(user_entity::Column::DeletedAt.eq(0))
            .order_by_desc(user_entity::Column::Id)
            .offset(offset)
            .limit(u64::from(page_size))
            .all(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(users)
    }

    /// Counts users
    async fn count(&self) -> Result<i64, AppError> {
        let count = user_entity::Entity::find()
            .filter(user_entity::Column::DeletedAt.eq(0))
            .count(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(count as i64)
    }

    /// Searches users by nickname
    async fn search_by_nickname(
        &self,
        nickname: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserInfo>, AppError> {
        let offset = u64::from(page - 1) * u64::from(page_size);
        let users = user_entity::Entity::find()
            .filter(user_entity::Column::NickName.contains(nickname))
            .filter(user_entity::Column::DeletedAt.eq(0))
            .order_by_desc(user_entity::Column::Id)
            .offset(offset)
            .limit(u64::from(page_size))
            .all(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(users)
    }

    /// Updates part of a user's information (only the provided columns are written)
    ///
    /// `updated_at` is refreshed by the entity `before_save` hook on every
    /// partial update, even when the caller does not request it.
    async fn update_partial(&self, id: i64, updates: &[UserUpdate]) -> Result<(), AppError> {
        if updates.is_empty() {
            return Ok(());
        }

        let mut model = user_entity::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        for update_item in updates {
            match update_item {
                UserUpdate::NickName(value) => model.nick_name = Set(value.clone()),
                UserUpdate::Avatar(value) => model.avatar = Set(value.clone()),
                UserUpdate::Signature(value) => model.signature = Set(value.clone()),
                UserUpdate::Age(value) => model.age = Set(*value),
                UserUpdate::Phone(value) => model.phone = Set(value.clone()),
                UserUpdate::Salt(value) => model.salt = Set(value.clone()),
                UserUpdate::Password(value) => model.password = Set(value.clone()),
                UserUpdate::Status(value) => model.status = Set(*value),
                UserUpdate::UpdatedAt(_) => {}
            }
        }

        model
            .update_without_returning(&self.db)
            .await
            .map_err(covert_error)?;

        Ok(())
    }
}
