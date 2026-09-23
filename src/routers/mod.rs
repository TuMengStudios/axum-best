//! routers module
//!
//! Construct web application router, one submodule per route domain:
//!
//! - [`user`]: `/user/*` routes, auth-guarded except the WeChat login
//! - [`foo`]: demo `/foo` route behind the auth middleware
//! - [`health`]: public health check
//! - [`metrics`]: optional Prometheus metrics endpoint
//! - [`openapi`]: optional Swagger UI and OpenAPI document
//! - [`layers`]: the global middleware stack applied to the merged router

mod fallback;
mod foo;
mod health;
mod layers;
mod metrics;
mod openapi;
mod user;

use axum::Router;

use crate::core::state::AppState;

/// Builds the application router: merge the per-domain route modules, apply
/// the global middleware stack, then the optional metrics endpoint.
pub fn app_routers(state: AppState) -> Router {
    let router = Router::new()
        .merge(user::routes(&state))
        .merge(foo::routes(&state))
        .merge(health::routes())
        .method_not_allowed_fallback(fallback::method_not_allowed)
        .fallback(fallback::not_found);

    let router = openapi::apply(router, &state);

    let router = layers::apply(router, &state);
    metrics::apply(router, &state).with_state(state)
}
