use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use validator::Validate;

use crate::models::user::UserInfo;

/// WeChat mini program login request
#[derive(Deserialize, Debug, Validate, utoipa::ToSchema)]
pub struct WxMiniLoginRequest {
    /// Login code obtained from WeChat login
    /// After using wx.login().then(res=>{res.code})
    ///
    /// `axios.post("/wx/login",{code:code})`
    #[validate(length(min = 1, max = 50))]
    pub code: String,
}

/// WeChat mini program login response
#[derive(Serialize, SmartDefault, utoipa::ToSchema)]
pub struct WxMiniLoginResponse {
    /// Bearer JWT to authenticate subsequent requests
    pub token: String,
    /// Display name stored for the user
    pub nick_name: String,
    /// Avatar URL stored for the user
    pub avatar: String,
}

/// Email login request structure
#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
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
#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct BindEmailRequest {
    /// Email address to bind
    #[validate(email)]
    pub email: String,

    /// Validation code for email binding
    #[validate(length(min = 1, max = 10))]
    pub valid_code: String,
}

/// Response structure for email binding operation
#[derive(Debug, Serialize, SmartDefault, utoipa::ToSchema)]
pub struct BindEmailResponse {
    // Response placeholder for email binding
}

/// Pre-bind email request for sending validation code
#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct PreBindEmailRequest {
    /// Email address for pre-binding validation
    #[validate(email)]
    pub email: String,
}

/// Pre-bind email response after sending validation code
#[derive(Debug, Serialize, SmartDefault, utoipa::ToSchema)]
pub struct PreBindEmailResponse {
    // TODO: Add response fields for pre-bind email
}

/// Pagination structure for list requests
#[derive(Debug, Validate, Deserialize, utoipa::ToSchema)]
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
#[derive(Deserialize, Debug, Validate, utoipa::ToSchema)]
pub struct ByUserIdRequest {
    /// User ID to search for
    #[validate(range(min = 1))]
    pub id: i64,
}

/// Response structure for `GET /user/{id}`.
///
/// Excludes sensitive fields (`salt`, `password`, `phone`) so the API never
/// returns password material or contact PII to clients.
#[derive(Serialize, Debug, utoipa::ToSchema)]
pub struct ByUserIdResponse {
    /// Unique identifier for the user
    pub id: i64,
    /// Display name of the user
    pub nick_name: String,
    /// URL or path to user's profile picture
    pub avatar: String,
    /// User's personal signature or bio
    pub signature: String,
    /// User's age
    pub age: u8,
    /// Timestamp when the user was created (Unix timestamp)
    pub created_at: i64,
    /// Timestamp when the user was last updated (Unix timestamp)
    pub updated_at: i64,
    /// Timestamp when the user was deleted (Unix timestamp, 0 if not deleted)
    pub deleted_at: i64,
    /// Account status, see [`UserInfo::STATUS_NORMAL`] / [`UserInfo::STATUS_DISABLED`]
    pub status: i8,
}

impl From<UserInfo> for ByUserIdResponse {
    fn from(user: UserInfo) -> Self {
        Self {
            id: user.id,
            nick_name: user.nick_name,
            avatar: user.avatar,
            signature: user.signature,
            age: user.age,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            status: user.status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ByUserIdResponse;
    use crate::models::user::UserInfo;

    #[test]
    fn by_user_id_response_omits_sensitive_fields() {
        let user = UserInfo {
            id: 42,
            phone: "13800138000".to_string(),
            salt: "super-secret-salt".to_string(),
            password: "super-secret-hash".to_string(),
            ..UserInfo::default()
        };

        let resp: ByUserIdResponse = user.into();
        let json = serde_json::to_value(&resp).expect("serialize ByUserIdResponse");

        assert!(json.get("salt").is_none(), "salt must not appear: {json}");
        assert!(json.get("password").is_none(), "password hash must not appear: {json}");
        assert!(json.get("phone").is_none(), "phone must not appear: {json}");
        assert_eq!(json.get("id"), Some(&serde_json::json!(42)));
    }
}

/// Request structure for getting random user
#[derive(Deserialize, Debug, utoipa::IntoParams, utoipa::ToSchema)]
pub struct RandomUserRequest {
    // No parameters needed for random user request
}

pub type RandomUserResponse = UserInfo;
