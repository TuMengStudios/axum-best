use std::time::Duration;

use axum::Router;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_request_id::RequestIdLayer;
use tracing::Level;

use crate::core::state::AppState;
use crate::handlers::foo;
use crate::handlers::health;
use crate::handlers::user as userHandler;
use crate::transport::middleware::inject_request_id;
use crate::transport::middleware::make_request_span;

async fn not_implemented() -> crate::core::Result<u8> {
    Err(crate::errors::ErrNotImplemented.clone())
}

pub fn app_routers(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(make_request_span)
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
        .on_eos(trace::DefaultOnEos::new().level(Level::INFO))
        .on_body_chunk(trace::DefaultOnBodyChunk::new());

    let cors_layer = CorsLayer::new().allow_credentials(true);

    let layer = ServiceBuilder::new()
        .layer(RequestDecompressionLayer::new())
        .layer(CompressionLayer::new())
        .layer(trace_layer);
    //
    Router::new()
        .route("/user/{id}", get(userHandler::user_by_id))
        .route("/user/wx/login", post(userHandler::wechat_login))
        .route("/user/email", post(userHandler::bind_email))
        .route("/user/email/pre", post(userHandler::pre_bind_email))
        .route("/user/random", get(userHandler::random_user))
        .route("/foo", get(foo::foo))
        .route("/health", get(health::health))
        .fallback(not_implemented)
        .layer(layer)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(cors_layer)
        // RequestIdLayer must stay outermost: it inserts the RequestId extension that
        // `inject_request_id` reads to tag logs and response headers.
        .layer(middleware::from_fn(inject_request_id))
        .layer(RequestIdLayer)
        .with_state(state)
}
