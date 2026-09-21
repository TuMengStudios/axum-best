use std::time::Duration;

use axum::Router;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_request_id::RequestIdLayer;
use tracing::Level;

use crate::core::state::AppState;
use crate::handlers::foo;
use crate::handlers::health;
use crate::handlers::user as userHandler;
use crate::transport::middleware::auth;
use crate::transport::middleware::compression;
use crate::transport::middleware::cors;
use crate::transport::middleware::otel;
use crate::transport::middleware::request_id::inject_request_id;
use crate::transport::middleware::request_id::make_request_span;
use crate::transport::middleware::timeout;

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

    let layer = ServiceBuilder::new()
        .layer(RequestDecompressionLayer::new())
        .layer(trace_layer);
    //
    // Middleware stack, outermost to innermost (each `.layer()` call wraps
    // everything above it, so the last call is the outermost):
    //   RequestIdLayer        inserts the RequestId extension
    //   inject_request_id     tags every log line with the request id
    //   CORS
    //   timeout               time budget (covers compression work)
    //   compression           negotiates from Accept-Encoding
    //   OpenTelemetry         adds the trace id response header before compression
    //   request decompression (from `layer`'s ServiceBuilder)
    //   tracing
    //   routes
    let protected_routes = Router::new()
        .route("/user/{id}", get(userHandler::user_by_id))
        .route("/user/email", post(userHandler::bind_email))
        .route("/user/email/pre", post(userHandler::pre_bind_email))
        .route("/user/random", get(userHandler::random_user))
        .route("/foo", get(foo::foo))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::auth));

    Router::new()
        .merge(protected_routes)
        .route("/user/wx/login", post(userHandler::wechat_login))
        .route("/health", get(health::health))
        .fallback(not_implemented)
        .layer(layer)
        .layer(middleware::from_fn(otel::middleware))
        .layer(middleware::from_fn_with_state(
            compression::CompressionConfig::new().with_excluded_prefixes(
                state.cfg.http.compression_excluded_paths.iter().cloned(),
            ),
            compression::middleware,
        ))
        .layer(middleware::from_fn_with_state(
            timeout::TimeoutConfig::new(Duration::from_secs(state.cfg.http.timeout_secs))
                .with_excluded_prefixes(state.cfg.http.timeout_excluded_paths.iter().cloned()),
            timeout::middleware,
        ))
        .layer(cors::layer())
        // RequestIdLayer must stay outermost: it inserts the RequestId extension that
        // `inject_request_id` reads to tag logs and response headers.
        .layer(middleware::from_fn(inject_request_id))
        .layer(RequestIdLayer)
        .with_state(state)
}
