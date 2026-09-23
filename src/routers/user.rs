use std::time::Duration;

use axum::Router;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;

use crate::core::state::AppState;
use crate::handlers::user as userHandler;
use crate::transport::middleware::auth;
use crate::transport::middleware::rate_limit::RateLimitLayer;

/// All `/user` routes.
///
/// Everything except `/user/wx/login` sits behind the auth middleware
/// (`route_layer`, so the guard only wraps the routes registered on the
/// protected sub-router), each with its own login quota.
pub fn routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route(
            "/user/{id}",
            get(userHandler::user_by_id).layer(RateLimitLayer::with_login_quota(
                Duration::from_secs(5),
                2,
                2,
            )),
        )
        .route(
            "/user/email",
            post(userHandler::bind_email).layer(RateLimitLayer::with_login_quota(
                Duration::from_secs(10),
                3,
                3,
            )),
        )
        .route(
            "/user/email/pre",
            post(userHandler::pre_bind_email).layer(RateLimitLayer::with_login_quota(
                Duration::from_secs(10),
                2,
                2,
            )),
        )
        .route(
            "/user/random",
            get(userHandler::random_user).layer(RateLimitLayer::with_login_quota(
                Duration::from_secs(5),
                2,
                2,
            )),
        )
        // `route_layer` wraps the route layers, so auth runs before each
        // `with_login_quota` layer and supplies its Claims extension first.
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::auth));

    Router::new()
        .merge(protected)
        .route("/user/wx/login", post(userHandler::wechat_login))
}
