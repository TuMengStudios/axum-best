use axum::extract::Query;
use axum::extract::State;
use tracing::debug;
use tracing::info;

use crate::core::Result;
use crate::core::state::AppState;
use crate::core::valid::ValidJson;
use crate::core::valid::ValidPath;
use crate::models::user::UserInfo;
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
#[utoipa::path(
    post,
    path = "/user/email",
    tag = "User",
    summary = "Bind an email address",
    description = "Binds an email address to the currently authenticated user. The verification code in the request must be valid.",
    security(("bearerAuth" = [])),
    request_body = BindEmailRequest,
    responses(
        (status = 200, description = "Email bound successfully", body = BindEmailResponse),
        (status = 400, description = "Request validation failed"),
        (status = 401, description = "Missing or invalid authentication")
    )
)]
pub async fn bind_email(
    State(state): State<AppState>,
    ValidJson(req): ValidJson<BindEmailRequest>,
) -> Result<BindEmailResponse> {
    info!("user email: {req:?}");
    state.user_service.bind_email(req).await
}

/// Handles WeChat mini-program login
///
/// # Arguments
/// * `state` - Application state containing shared resources
/// * `req` - WeChat mini-program login request containing authorization code
///
/// # Returns
/// * `Result<WxMiniLoginResponse>` - Login response with user information
#[utoipa::path(
    post,
    path = "/user/wx/login",
    tag = "User",
    summary = "Log in with WeChat",
    description = "Logs in or creates a user with a WeChat mini-program login code and returns basic user information.",
    request_body = WxMiniLoginRequest,
    responses(
        (status = 200, description = "Login successful", body = WxMiniLoginResponse),
        (status = 400, description = "Invalid WeChat login code or request parameters")
    )
)]
pub async fn wechat_login(
    State(state): State<AppState>,
    ValidJson(req): ValidJson<WxMiniLoginRequest>,
) -> Result<WxMiniLoginResponse> {
    debug!("code {}", req.code);
    state.user_service.wx_login(req).await
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    tag = "User",
    summary = "Get a user by ID",
    description = "Returns detailed user information for the specified user ID. This endpoint requires a Bearer JWT.",
    params(("id" = i64, Path, description = "User ID; must be greater than 0")),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "User found", body = UserInfo),
        (status = 400, description = "Invalid user ID format or value"),
        (status = 401, description = "Missing or invalid authentication"),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_by_id(
    State(state): State<AppState>,
    ValidPath(req): ValidPath<ByUserIdRequest>,
) -> Result<UserInfo> {
    info!("user by id {:?}", req);
    state.user_service.by_id(req).await
}

/// Pre-validates email for binding operation
///
/// # Arguments
/// * `req` - Pre-binding email request containing email to validate
///
/// # Returns
/// * `Result<PreBindEmailResponse>` - Pre-validation result
#[utoipa::path(
    post,
    path = "/user/email/pre",
    tag = "User",
    summary = "Send an email verification code",
    description = "Sends an email verification code to the specified address. The current user must be authenticated.",
    security(("bearerAuth" = [])),
    request_body = PreBindEmailRequest,
    responses(
        (status = 200, description = "Verification code sent", body = PreBindEmailResponse),
        (status = 400, description = "Invalid email address"),
        (status = 401, description = "Missing or invalid authentication")
    )
)]
pub async fn pre_bind_email(
    State(state): State<AppState>,
    ValidJson(req): ValidJson<PreBindEmailRequest>,
) -> Result<PreBindEmailResponse> {
    info!("user {}", req.email);
    state.user_service.pre_bind_email(req).await
}

#[utoipa::path(
    get,
    path = "/user/random",
    tag = "User",
    summary = "Generate a random user",
    description = "Generates and returns a random user example. The current user must be authenticated.",
    security(("bearerAuth" = [])),
    params(RandomUserRequest),
    responses(
        (status = 200, description = "Random user generated successfully", body = UserInfo),
        (status = 401, description = "Missing or invalid authentication")
    )
)]
pub async fn random_user(
    State(state): State<AppState>,
    Query(req): Query<RandomUserRequest>,
) -> Result<RandomUserResponse> {
    info!("create random user");
    state.user_service.random_user(req).await
}
