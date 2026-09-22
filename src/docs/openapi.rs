use utoipa::OpenApi;

use crate::models::user::UserInfo;
use crate::types::foo::{FooItem, FooRequest, FooResponse};
use crate::types::user::{
    BindEmailRequest, BindEmailResponse, PreBindEmailRequest, PreBindEmailResponse,
};
use crate::types::user::{RandomUserRequest, WxMiniLoginRequest, WxMiniLoginResponse};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum Best API",
        version = env!("CARGO_PKG_VERSION"),
        description = "HTTP API for the Axum Best service. Successful responses use a unified response structure. Authenticated endpoints require a Bearer JWT."
    ),
    paths(
        crate::handlers::health::health,
        crate::handlers::user::wechat_login,
        crate::handlers::user::bind_email,
        crate::handlers::user::pre_bind_email,
        crate::handlers::user::user_by_id,
        crate::handlers::user::random_user,
        crate::handlers::foo::foo
    ),
    components(schemas(
        BindEmailRequest,
        BindEmailResponse,
        FooItem,
        FooRequest,
        FooResponse,
        PreBindEmailRequest,
        PreBindEmailResponse,
        RandomUserRequest,
        UserInfo,
        WxMiniLoginRequest,
        WxMiniLoginResponse
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Service health endpoints"),
        (name = "User", description = "User account endpoints"),
        (name = "Demo", description = "Example endpoints")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}
