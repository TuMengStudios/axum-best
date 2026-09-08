use std::sync::Arc;

use serde::Deserialize;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::core::Result;
use crate::errors;
use crate::models::user::UserInfo;
use crate::ok;
use crate::repos::kv::KvStore;
use crate::repos::user::UserRepo;
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

#[derive(Debug, Deserialize)]
/// Response structure for WeChat login API
struct InnerWechatLoginResponse {
    /// WeChat user's unique identifier
    pub openid: String,
}

/// User service for handling user-related operations
///
/// 只依赖仓储接口（`Arc<dyn UserRepo>` / `Arc<dyn KvStore>`），
/// 不感知底层存储实现，单测时可注入 mock；具体实现由组装层（app::AppContext）注入。
#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepo>,
    kv: Arc<dyn KvStore>,
}

impl UserService {
    /// Creates a service with injected repository implementations
    pub fn new(repo: Arc<dyn UserRepo>, kv: Arc<dyn KvStore>) -> UserService {
        UserService { repo, kv }
    }

    /// Pre-binds an email address by generating and storing a validation code
    ///
    /// # Arguments
    /// * `req` - PreBindEmailRequest containing the email address
    ///
    /// # Returns
    /// * `Result<PreBindEmailResponse>` - Response indicating success
    pub async fn pre_bind_email(&self, req: PreBindEmailRequest) -> Result<PreBindEmailResponse> {
        let key = format!("bind_email_{}", req.email);
        let valid_code = utils::gen_valid_code(5);
        info!("valid_code {}", valid_code);
        self.kv.set(&key, &valid_code).await?;
        ok!(PreBindEmailResponse::default())
    }

    /// Handles WeChat mini-program login
    ///
    /// # Arguments
    /// * `req` - WxMiniLoginRequest containing WeChat login code
    ///
    /// # Returns
    /// * `Result<WxMiniLoginResponse>` - Login response with user authentication info
    pub async fn wx_login(&self, req: WxMiniLoginRequest) -> Result<WxMiniLoginResponse> {
        debug!("wx login {}", req.code);
        // Not Implemented Yet
        let resp = ureq::get("https://exmaple.com/foo/baz")
            .call()
            .map_err(|err| {
                error!("call wechat api error {}", err);
                errors::ErrWechatLogin.clone()
            })?
            .body_mut()
            .read_json::<InnerWechatLoginResponse>()
            .map_err(|err| {
                error!("unmarshal wechat response {}", err);
                errors::ErrUnmarshalJSON.clone()
            })?;

        let open_id = resp.openid.clone();
        let user = self.repo.get_by_wx_open_id(&open_id).await?;
        info!("user info {:?}", user);
        let resp = WxMiniLoginResponse {
            auth: user.nick_name,
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
    pub async fn bind_email(&self, req: BindEmailRequest) -> Result<BindEmailResponse> {
        info!("bind email {}", req.email);
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
    /// * `Result<ByUserIdResponse>` - User information response
    pub async fn by_id(&self, req: ByUserIdRequest) -> Result<ByUserIdResponse> {
        let user = self.repo.get_by_id(req.id).await?;
        info!("by id {} user {user:?}", req.id);
        ok!(user)
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
