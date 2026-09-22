use std::time::Duration;

use axum::Router;
use axum::routing::get;

use crate::core::state::AppState;
use crate::handlers::health;
use crate::transport::middleware::rate_limit::RateLimitLayer;

/// Public health check route.
pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/health",
        get(health::health).layer(RateLimitLayer::with_quota(Duration::from_secs(10), 2, 2)),
    )
}
