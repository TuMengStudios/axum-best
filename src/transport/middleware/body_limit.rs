//! Request body size limit middleware.

use tower_http::limit::RequestBodyLimitLayer;

/// Builds the request body limit layer for the configured maximum size.
pub fn body_limit(max_bytes: usize) -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(max_bytes)
}
