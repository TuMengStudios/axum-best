use std::time::Duration;

use axum::Router;
use axum::middleware;
use axum::routing::get;

use crate::core::state::AppState;
use crate::handlers::foo;
use crate::transport::middleware::auth;
use crate::transport::middleware::rate_limit::RateLimitLayer;

/// Demo `/foo` route behind the auth middleware.
pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/foo",
            get(foo::foo).layer(RateLimitLayer::with_login_quota(Duration::from_secs(5), 2, 2)),
        )
        // Keep auth outside the route quota so the quota can read Claims.
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::auth))
}
