//! Cross-origin resource sharing middleware configuration.

use tower_http::cors::CorsLayer;

/// Build the application's CORS layer.
pub fn layer() -> CorsLayer {
    CorsLayer::new().allow_credentials(true)
}
