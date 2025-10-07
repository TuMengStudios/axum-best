use axum::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum_valid::Valid;
use tracing::debug;
use tracing::info;

use crate::core::Result;
use crate::core::state::AppState;
use crate::models::user::UserInfo;
use crate::services::user::UserService;
use crate::types::user::BindEmailRequest;
use crate::types::user::BindEmailResponse;
use crate::types::user::ByUserIdRequest;
use crate::types::user::PreBindEmailRequest;
use crate::types::user::PreBindEmailResponse;
use crate::types::user::RandomUserRequest;
use crate::types::user::RandomUserResponse;
use crate::types::user::WxMiniLoginRequest;
use crate::types::user::WxMiniLoginResponse;

/// Binds an email address to a user account
///
/// # Arguments
/// * `state` - Application state containing shared resources
/// * `req` - Email binding request containing user email
///
/// # Returns
/// * `Result<BindEmailResponse>` - Binding operation result
pub async fn bind_email(
    State(state): State<AppState>,
    Valid(Json(req)): Valid<Json<BindEmailRequest>>,
) -> Result<BindEmailResponse> {
    info!("user email: {req:?}");
    UserService::bind_email(state, req).await
}

/// Handles WeChat mini-program login
///
/// # Arguments
/// * `state` - Application state containing shared resources
/// * `req` - WeChat mini-program login request containing authorization code
///
/// # Returns
/// * `Result<WxMiniLoginResponse>` - Login response with user information
pub async fn wechat_login(
    State(state): State<AppState>,
    Json(req): Json<WxMiniLoginRequest>,
) -> Result<WxMiniLoginResponse> {
    debug!("code {}", req.code);
    UserService::wx_login(state, req).await
}

pub async fn user_by_id(
    State(state): State<AppState>,
    Path(req): Path<ByUserIdRequest>,
) -> Result<UserInfo> {
    info!("user by id {:?}", req);
    UserService::by_id(state, req).await
}

/// Pre-validates email for binding operation
///
/// # Arguments
/// * `req` - Pre-binding email request containing email to validate
///
/// # Returns
/// * `Result<PreBindEmailResponse>` - Pre-validation result
pub async fn pre_bind_email(
    State(state): State<AppState>,
    Json(req): Json<PreBindEmailRequest>,
) -> Result<PreBindEmailResponse> {
    info!("user {}", req.email);
    UserService::pre_bind_email(state, req).await
}

pub async fn random_user(
    State(state): State<AppState>,
    Query(req): Query<RandomUserRequest>,
) -> Result<RandomUserResponse> {
    info!("create random user");
    UserService::random_user(state, req).await
}
