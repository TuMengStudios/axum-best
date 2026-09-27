//! Consumer-friendly re-exports of the entity models.
//!
//! External code can `use axum_best::models::entity::prelude::*` (or the
//! re-exports from [`crate::models`]) to get the domain model together with
//! its `Entity` type under stable names without depending on SeaORM's
//! mandatory `Model` struct name. `Column` and `ActiveModel` are intentionally
//! not re-exported; import them from the entity module when building filters
//! or writes.

pub use super::oauth_account::Entity as OAuthAccountEntity;
pub use super::oauth_account::Model as OAuthAccount;

pub use super::user_info::Entity as UserInfoEntity;
pub use super::user_info::Model as UserInfo;
