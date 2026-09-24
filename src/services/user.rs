use std::sync::Arc;

use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing::info;

use crate::auth::JwtConfig;
use crate::core::Result;
use crate::core::rest::AppError;
use crate::data::kv::RedisKvStore;
use crate::errors::ErrUserAbnormal;
use crate::models::oauth::OAuthAccount;
use crate::models::user::UserInfo;
use crate::ok;
use crate::repos::user::UserRepo;
use crate::repos::wechat::WechatRepo;
use crate::types::user::BindEmailRequest;
use crate::types::user::BindEmailResponse;
use crate::types::user::ByUserIdRequest;
use crate::types::user::ByUserIdResponse;
use crate::types::user::PreBindEmailRequest;
use crate::types::user::PreBindEmailResponse;
use crate::types::user::RandomUserRequest;
use crate::types::user::RandomUserResponse;
use crate::types::user::WxMiniLoginRequest;
use crate::types::user::WxMiniLoginResponse;
use crate::utils;

#[derive(Deserialize, FromRedisValue, Serialize, ToRedisArgs)]
struct BindEmailCode {
    email: String,
    valid_code: String,
}

/// User service for handling user-related operations
///
/// Depends on repository abstractions for user data and the Redis data implementation for KV writes, and is
/// unaware of the underlying storage; mocks can be injected in unit tests, while the
/// concrete implementations are injected by the assembly layer (app::AppContext).
#[derive(Clone)]
pub struct UserService {
    jwt: Arc<JwtConfig>,
    repo: Arc<dyn UserRepo>,
    kv: Arc<RedisKvStore>,
    wechat: Arc<dyn WechatRepo>,
}

impl UserService {
    /// Creates a service with injected repository implementations
    pub fn new(
        jwt: Arc<JwtConfig>,
        repo: Arc<dyn UserRepo>,
        kv: Arc<RedisKvStore>,
        wechat: Arc<dyn WechatRepo>,
    ) -> UserService {
        UserService {
            jwt,
            repo,
            kv,
            wechat,
        }
    }

    /// Loads a user for authentication and rejects soft-deleted or disabled accounts.
    pub async fn active_user(&self, user_id: i64) -> std::result::Result<UserInfo, AppError> {
        let user = self.repo.get_by_id(user_id).await?;
        if user.deleted_at != 0 || user.status != UserInfo::STATUS_NORMAL {
            return Err(ErrUserAbnormal.clone());
        }
        Ok(user)
    }

    async fn get_or_create_wechat_user(
        &self,
        app_id: &str,
        open_id: &str,
    ) -> std::result::Result<UserInfo, AppError> {
        const PROVIDER: &str = "wechat";
        if let Some(user) = self.repo.get_by_oauth(PROVIDER, app_id, open_id).await? {
            return Ok(user);
        }

        let mut user = UserInfo::new_external();
        let mut account = OAuthAccount::new(0, PROVIDER, app_id, open_id);
        self.repo
            .create_user_with_oauth(&mut user, &mut account)
            .await?;
        info!(user_id = user.id, "created WeChat user");
        Ok(user)
    }

    /// Pre-binds an email address by generating and storing a validation code
    ///
    /// # Arguments
    /// * `req` - PreBindEmailRequest containing the email address
    ///
    /// # Returns
    /// * `Result<PreBindEmailResponse>` - Response indicating success
    pub async fn pre_bind_email(
        &self,
        user_id: i64,
        req: PreBindEmailRequest,
    ) -> Result<PreBindEmailResponse> {
        const VALID_CODE_TTL_SECS: u64 = 300;
        let key = format!("bind_email:{user_id}");
        let valid_code = utils::gen_valid_code(5);
        let value = BindEmailCode {
            email: req.email,
            valid_code,
        };
        self.kv.set_ex(&key, value, VALID_CODE_TTL_SECS).await?;
        ok!(PreBindEmailResponse::default())
    }

    /// Handles WeChat mini-program login
    ///
    /// Exchanges the WeChat code for a session, locates or creates the user, and
    /// issues a Bearer JWT so subsequent authenticated requests succeed.
    ///
    /// # Arguments
    /// * `req` - WxMiniLoginRequest containing WeChat login code
    ///
    /// # Returns
    /// * `Result<WxMiniLoginResponse>` - Login response with user authentication info
    pub async fn wx_login(&self, req: WxMiniLoginRequest) -> Result<WxMiniLoginResponse> {
        debug!("WeChat login request received");
        let session = self.wechat.exchange_login_code(&req.code).await?;
        let user = self
            .get_or_create_wechat_user(&session.app_id, &session.open_id)
            .await?;
        let token = self.jwt.generate_token(user.id)?;
        info!(user_id = user.id, "WeChat login succeeded");
        let resp = WxMiniLoginResponse {
            token,
            nick_name: user.nick_name,
            avatar: user.avatar,
        };
        ok!(resp)
    }

    /// Binds an email address to a user account
    ///
    /// # Arguments
    /// * `req` - BindEmailRequest containing email address
    ///
    /// # Returns
    /// * `Result<BindEmailResponse>` - Response indicating successful binding
    pub async fn bind_email(
        &self,
        user_id: i64,
        req: BindEmailRequest,
    ) -> Result<BindEmailResponse> {
        info!("bind email {}", req.email);
        let code_key = format!("bind_email:{user_id}");
        let code: BindEmailCode = self.kv.get(&code_key).await?;
        if code.email != req.email || code.valid_code != req.valid_code {
            return Err(crate::errors::ErrBadRequest.clone());
        }
        self.kv.del(&code_key).await?;

        let key = format!("user_{}", req.email);
        self.kv.set(&key, "").await?;
        ok!(BindEmailResponse::default())
    }

    /// Retrieves user information by user ID
    ///
    /// # Arguments
    /// * `req` - ByUserIdRequest containing user ID
    ///
    /// # Returns
    /// * `Result<ByUserIdResponse>` - User information response (without `salt`/`password`)
    pub async fn by_id(&self, req: ByUserIdRequest) -> Result<ByUserIdResponse> {
        let user = self.repo.get_by_id(req.id).await?;
        info!("by id {}", req.id);
        ok!(user.into())
    }

    /// Generates a random user
    ///
    /// # Arguments
    /// * `req` - RandomUserRequest (currently unused)
    ///
    /// # Returns
    /// * `Result<RandomUserResponse>` - Response containing the randomly generated user
    pub async fn random_user(&self, _req: RandomUserRequest) -> Result<RandomUserResponse> {
        let mut user = UserInfo::random();
        info!("create random user {user:?}");
        self.repo.create(&mut user).await?;
        ok!(user)
    }
}
