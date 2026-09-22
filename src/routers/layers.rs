use std::time::Duration;

use axum::Router;
use axum::middleware;
use tower::ServiceBuilder;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::core::state::AppState;
use crate::transport::middleware::body_limit;
use crate::transport::middleware::compression;
use crate::transport::middleware::cors;
use crate::transport::middleware::otel;
use crate::transport::middleware::timeout;

/// Applies the global middleware stack.
///
/// Order, outermost to innermost (each `.layer()` call wraps everything
/// above it, so the last call is the outermost):
///   CORS
///   timeout               time budget (covers compression work)
///   compression           negotiates from Accept-Encoding
///   OpenTelemetry         adds the trace id response header before compression
///   request decompression (from `layer`'s ServiceBuilder)
///   tracing
///   routes
pub fn apply(router: Router<AppState>, state: &AppState) -> Router<AppState> {
    let trace_layer = TraceLayer::new_for_http()
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
        .on_eos(trace::DefaultOnEos::new().level(Level::INFO))
        .on_body_chunk(trace::DefaultOnBodyChunk::new());

    let layer = ServiceBuilder::new()
        .layer(RequestDecompressionLayer::new())
        .layer(trace_layer);

    router
        .layer(layer)
        .layer(middleware::from_fn(otel::middleware))
        .layer(middleware::from_fn_with_state(
            compression::CompressionConfig::new()
                .with_excluded_prefixes(state.cfg.http.compression_excluded_paths.iter().cloned()),
            compression::middleware,
        ))
        .layer(middleware::from_fn_with_state(
            timeout::TimeoutConfig::new(Duration::from_secs(state.cfg.http.timeout_secs))
                .with_excluded_prefixes(state.cfg.http.timeout_excluded_paths.iter().cloned()),
            timeout::middleware,
        ))
        .layer(body_limit::body_limit(state.cfg.http.request_body_limit_bytes))
        .layer(cors::layer(&state.cfg.http.cors_allowed_origins))
}
