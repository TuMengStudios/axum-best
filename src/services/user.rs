use redis::Commands;
use serde::Deserialize;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::core::Result;
use crate::core::state::AppState;
use crate::errors;
use crate::models::user::UserInfo;
use crate::ok;
use crate::repos;
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
pub struct UserService;

impl UserService {
    /// Pre-binds an email address by generating and storing a validation code
    ///
    /// # Arguments
    /// * `state` - Application state containing database connections
    /// * `req` - PreBindEmailRequest containing the email address
    ///
    /// # Returns
    /// * `Result<PreBindEmailResponse>` - Response indicating success
    pub async fn pre_bind_email(
        state: AppState,
        req: PreBindEmailRequest,
    ) -> Result<PreBindEmailResponse> {
        let key = format!("bind_email_{}", req.email);
        let valid_code = utils::gen_valid_code(5);
        info!("valid_code {}", valid_code);
        let _: () = state
            .get_redis_client()
            .unwrap()
            .set(&key, valid_code)
            .unwrap();
        ok!(PreBindEmailResponse::default())
    }

    /// Handles WeChat mini-program login
    ///
    /// # Arguments
    /// * `state` - Application state containing database connections
    /// * `req` - WxMiniLoginRequest containing WeChat login code
    ///
    /// # Returns
    /// * `Result<WxMiniLoginResponse>` - Login response with user authentication info
    pub async fn wx_login(state: AppState, req: WxMiniLoginRequest) -> Result<WxMiniLoginResponse> {
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
        let user = repos::user::get_by_wx_open_id(&state.get_conn(), &open_id).await?;
        info!("user info {:?}", user);
        let resp = WxMiniLoginResponse {
            auth: user.nick_name,
        };
        ok!(resp)
    }

    /// Binds an email address to a user account
    ///
    /// # Arguments
    /// * `state` - Application state containing database connections
    /// * `req` - BindEmailRequest containing email address
    ///
    /// # Returns
    /// * `Result<BindEmailResponse>` - Response indicating successful binding
    pub async fn bind_email(state: AppState, req: BindEmailRequest) -> Result<BindEmailResponse> {
        info!("bind email {}", req.email);
        let mut redis_conn = state.get_redis_client()?;
        let key = format!("user_{}", req.email);
        let _: () = redis_conn.set(&key, "").unwrap();
        ok!(BindEmailResponse::default())
    }

    /// Retrieves user information by user ID
    ///
    /// # Arguments
    /// * `state` - Application state containing database connections
    /// * `req` - ByUserIdRequest containing user ID
    ///
    /// # Returns
    /// * `Result<ByUserIdResponse>` - User information response
    pub async fn by_id(state: AppState, req: ByUserIdRequest) -> Result<ByUserIdResponse> {
        let user = repos::user::get_by_id(&state.get_conn(), req.id).await?;
        info!("by id {} user {user:?}", req.id);
        ok!(user)
    }

    /// Generates a random user
    ///
    /// # Arguments
    /// * `state` - Application state containing database connections
    /// * `req` - RandomUserRequest (currently unused)
    ///
    /// # Returns
    /// * `Result<RandomUserResponse>` - Response containing the randomly generated user
    pub async fn random_user(
        state: AppState,
        _req: RandomUserRequest,
    ) -> Result<RandomUserResponse> {
        let mut user = UserInfo::random();
        info!("create random user {user:?}");
        repos::user::create(&state.get_conn(), &mut user).await?;
        ok!(user)
    }
}
