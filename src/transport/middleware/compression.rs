//! Response compression middleware with per-path exclusions
//!
//! Reuses [`tower_http::compression::CompressionLayer`] per request: responses
//! for excluded prefixes are served verbatim (no `Content-Encoding`), everything
//! else is compressed according to the request's `Accept-Encoding`. Excluding
//! is useful for endpoints that stream (SSE) or already emit compressed
//! content. Matching is segment-aware: `/stream` excludes `/stream` and
//! `/stream/1`, but not `/streaming`.

use std::sync::Arc;

use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::Response;
use tower::Layer;
use tower::ServiceExt;
use tower_http::compression::CompressionLayer;

/// Configuration for the compression middleware, used as the
/// [`axum::middleware::from_fn_with_state`] state
#[derive(Clone, Default)]
pub struct CompressionConfig {
    excluded_prefixes: Arc<[String]>,
}

impl CompressionConfig {
    pub fn new() -> CompressionConfig {
        CompressionConfig::default()
    }

    /// Registers path prefixes whose responses skip compression
    pub fn with_excluded_prefixes<I, P>(mut self, prefixes: I) -> CompressionConfig
    where
        I: IntoIterator<Item = P>,
        P: Into<String>,
    {
        self.excluded_prefixes = prefixes
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into();
        self
    }
}

/// The middleware itself; plug in via
/// [`axum::middleware::from_fn_with_state`]
pub async fn middleware(
    State(config): State<CompressionConfig>,
    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_owned();

    if super::prefix::is_excluded(&path, &config.excluded_prefixes) {
        return next.run(req).await;
    }

    // Reuse tower-http's compression: it negotiates the encoding from the
    // request's `Accept-Encoding` and leaves already-compressed responses
    // untouched. `Next` is infallible, so the error side is unreachable.
    match CompressionLayer::new().layer(next).oneshot(req).await {
        Ok(resp) => resp.map(Body::new),
        Err(never) => match never {},
    }
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::middleware;
    use axum::routing::get;
    use tower::ServiceExt;

    use super::CompressionConfig;
    use super::middleware;

    async fn handler() -> &'static str {
        "compression middleware test body"
    }

    fn request(path: &str) -> HttpRequest<Body> {
        HttpRequest::builder()
            .uri(path)
            .header("accept-encoding", "gzip")
            .body(Body::empty())
            .unwrap()
    }

    fn app(config: CompressionConfig) -> Router {
        Router::new()
            .route("/data", get(handler))
            .route("/excluded/data", get(handler))
            .layer(middleware::from_fn_with_state(config, middleware))
    }

    #[tokio::test]
    async fn response_is_compressed_when_accepted() {
        let resp = app(CompressionConfig::new())
            .oneshot(request("/data"))
            .await
            .unwrap();
        assert_eq!(resp.headers().get("content-encoding").unwrap(), "gzip");
    }

    #[tokio::test]
    async fn excluded_prefix_serves_the_response_uncompressed() {
        let config = CompressionConfig::new().with_excluded_prefixes(["/excluded"]);
        let resp = app(config)
            .oneshot(request("/excluded/data"))
            .await
            .unwrap();
        assert!(resp.headers().get("content-encoding").is_none());
    }

    #[tokio::test]
    async fn no_accept_encoding_means_no_compression() {
        let req = HttpRequest::builder()
            .uri("/data")
            .body(Body::empty())
            .unwrap();
        let resp = app(CompressionConfig::new()).oneshot(req).await.unwrap();
        assert!(resp.headers().get("content-encoding").is_none());
    }
}
