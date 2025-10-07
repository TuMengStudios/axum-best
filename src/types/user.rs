use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use validator::Validate;

use crate::models::user::UserInfo;

/// WeChat mini program login request
#[allow(unused)]
#[derive(Deserialize, Debug, Validate)]
pub struct WxMiniLoginRequest {
    /// Login code obtained from WeChat login
    /// After using wx.login().then(res=>{res.code})
    ///
    /// `axios.post("/wx/login",{code:code})`
    #[validate(length(min = 1, max = 50))]
    pub code: String,
}

/// WeChat mini program login response
#[allow(unused)]
#[derive(Serialize, SmartDefault)]
pub struct WxMiniLoginResponse {
    /// Authentication token for login
    pub auth: String,
}

/// Email login request structure
#[derive(Debug, Deserialize, Validate)]
pub struct EmailLoginRequest {
    /// User's email address
    #[validate(email)]
    pub email: String,

    /// User's password
    #[validate(length(min = 6, max = 20))]
    pub password: String,
}

pub type EmailLoginResponse = WxMiniLoginResponse;

/// Request structure for binding email to user account
#[derive(Debug, Deserialize, Validate)]
pub struct BindEmailRequest {
    /// Email address to bind
    #[validate(email)]
    pub email: String,

    /// Validation code for email binding
    #[validate(length(min = 1, max = 10))]
    pub valid_code: String,
}

/// Response structure for email binding operation
#[derive(Debug, Serialize, SmartDefault)]
pub struct BindEmailResponse {
    // Response placeholder for email binding
}

/// Pre-bind email request for sending validation code
#[derive(Debug, Deserialize, Validate)]
pub struct PreBindEmailRequest {
    /// Email address for pre-binding validation
    #[validate(email)]
    pub email: String,
}

/// Pre-bind email response after sending validation code
#[derive(Debug, Serialize, SmartDefault)]
pub struct PreBindEmailResponse {
    // TODO: Add response fields for pre-bind email
}

/// Pagination structure for list requests
#[derive(Debug, Validate, Deserialize)]
pub struct Paginator {
    /// Number of items per page (1-50)
    #[validate(range(min = 1, max = 50))]
    pub page_size: usize,
    /// Current page number (starts from 1)
    #[validate(range(min = 1))]
    pub page_no: usize,
}

pub type UsersListRequest = Paginator;
pub type BooksListRequest = Paginator;

/// Request structure for getting user by ID
#[derive(Deserialize, Debug)]
pub struct ByUserIdRequest {
    /// User ID to search for
    pub id: i64,
}

pub type ByUserIdResponse = UserInfo;

/// Request structure for getting random user
#[derive(Deserialize, Debug)]
pub struct RandomUserRequest {
    // No parameters needed for random user request
}

pub type RandomUserResponse = UserInfo;
