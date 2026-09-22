//! routers module
//!
//! Construct web application router, one submodule per route domain:
//!
//! - [`user`]: `/user/*` routes, auth-guarded except the WeChat login
//! - [`foo`]: demo `/foo` route behind the auth middleware
//! - [`health`]: public health check
//! - [`metrics`]: optional Prometheus metrics endpoint
//! - [`layers`]: the global middleware stack applied to the merged router

mod foo;
mod health;
mod layers;
mod metrics;
mod user;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::state::AppState;

async fn not_implemented() -> crate::core::Result<u8> {
    Err(crate::errors::ErrNotImplemented.clone())
}

/// Builds the application router: merge the per-domain route modules, apply
/// the global middleware stack, then the optional metrics endpoint.
pub fn app_routers(state: AppState) -> Router {
    let swagger_enabled = state.cfg.swagger.enabled;
    let router = Router::new()
        .merge(user::routes(&state))
        .merge(foo::routes(&state))
        .merge(health::routes())
        .fallback(not_implemented);

    let router = if swagger_enabled {
        router.merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", crate::docs::openapi::ApiDoc::openapi()),
        )
    } else {
        router
    };

    let router = layers::apply(router, &state);
    metrics::apply(router, &state).with_state(state)
}
