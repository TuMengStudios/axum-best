use std::time::Duration;

use axum::Router;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_request_id::RequestId;
use tower_request_id::RequestIdLayer;
use tracing::Instrument;
use tracing::Level;

use crate::core::state::AppState;
use crate::handlers::foo;
use crate::handlers::health;
use crate::handlers::user as userHandler;

async fn not_implemented() -> crate::core::Result<u8> {
    Err(crate::errors::ErrNotImplemented.clone())
}

pub fn app_routers(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
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
    let app = Router::new()
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
        .layer(middleware::from_fn(inject_request_id))
        .layer(RequestIdLayer)
        .with_state(state);

    app
}

/// Middleware function that injects request ID into the response headers and tracing spans
///
/// This middleware:
/// - Extracts the request ID from the request extensions (if available)
/// - Creates a tracing span with the request ID for better observability
/// - Adds the request ID to the response headers as "Request-Id"
/// - Falls back to "unknown" if no request ID is found
///
/// # Arguments
/// * `req` - The incoming HTTP request
/// * `next` - The next middleware/handler in the chain
///
/// # Returns
/// The HTTP response with request ID header added
async fn inject_request_id(req: Request, next: Next) -> Response {
    let request_id = req
        .extensions()
        .get::<RequestId>()
        .map(ToString::to_string)
        .unwrap_or_else(|| "unknown".into());

    let parent = tracing::error_span!("http_request", request_id = &request_id);

    let mut resp = next.run(req).instrument(parent).await;
    let resp_header = resp.headers_mut();
    resp_header.insert("Request-Id", HeaderValue::from_str(&request_id).unwrap());
    resp
}
