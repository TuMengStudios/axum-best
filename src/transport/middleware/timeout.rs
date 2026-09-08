//! Request timeout middleware with per-path exemptions
//!
//! Enforces a time budget on request handling by reusing
//! [`tower_http::timeout::TimeoutLayer`], which answers a bare
//! `408 Request Timeout` (empty body) when the budget elapses. Requests whose
//! path matches one of the configured exempt prefixes skip the timeout
//! entirely, for long-running endpoints such as SSE streams, file uploads or
//! reports. Matching is segment-aware: `/stream` exempts `/stream` and
//! `/stream/1`, but not `/streaming`.

use std::sync::Arc;
use std::time::Duration;

use axum::extract::Request;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::Response;
use tower::Layer;
use tower::ServiceExt;
use tower_http::timeout::TimeoutLayer;

/// Configuration for the timeout middleware, used as the
/// [`axum::middleware::from_fn_with_state`] state
#[derive(Clone)]
pub struct TimeoutConfig {
    duration: Duration,
    exempt_prefixes: Arc<[String]>,
}

impl TimeoutConfig {
    pub fn new(duration: Duration) -> TimeoutConfig {
        TimeoutConfig {
            duration,
            exempt_prefixes: Arc::from([]),
        }
    }

    /// Registers path prefixes that skip the timeout
    pub fn with_exempt_prefixes<I, P>(mut self, prefixes: I) -> TimeoutConfig
    where
        I: IntoIterator<Item = P>,
        P: Into<String>,
    {
        self.exempt_prefixes = prefixes
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into();
        self
    }
}

/// The middleware itself; plug in via
/// [`axum::middleware::from_fn_with_state`](axum::middleware::from_fn_with_state)
pub async fn middleware(State(config): State<TimeoutConfig>, req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();

    if is_exempt(&path, &config.exempt_prefixes) {
        return next.run(req).await;
    }

    // Reuse tower-http's timeout service: on elapsed it answers a bare 408
    // itself. `Next` is infallible, so the error side is unreachable.
    match TimeoutLayer::new(config.duration)
        .layer(next)
        .oneshot(req)
        .await
    {
        Ok(resp) => resp,
        Err(never) => match never {},
    }
}

/// Segment-aware prefix match: `/stream` exempts `/stream` and `/stream/1`
/// but not `/streaming`. A bare `/` exempts everything.
fn is_exempt(path: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| {
        let prefix = prefix.trim_end_matches('/');
        path == prefix || path.starts_with(&format!("{prefix}/"))
    })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::Router;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::http::StatusCode;
    use axum::middleware;
    use axum::routing::get;
    use tower::ServiceExt;

    use super::TimeoutConfig;
    use super::middleware;

    async fn slow_handler() -> &'static str {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "ok"
    }

    fn request(path: &str) -> HttpRequest<Body> {
        HttpRequest::builder()
            .uri(path)
            .body(Body::empty())
            .unwrap()
    }

    fn app(config: TimeoutConfig) -> Router {
        Router::new()
            .route("/slow", get(slow_handler))
            .route("/exempt/slow", get(slow_handler))
            .route("/slower", get(slow_handler))
            .layer(middleware::from_fn_with_state(config, middleware))
    }

    #[tokio::test]
    async fn path_over_budget_times_out_with_408() {
        let config = TimeoutConfig::new(Duration::from_millis(20));
        let resp = app(config).oneshot(request("/slow")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
    }

    #[tokio::test]
    async fn exempt_prefix_bypasses_the_timeout() {
        let config =
            TimeoutConfig::new(Duration::from_millis(20)).with_exempt_prefixes(["/exempt"]);
        let resp = app(config).oneshot(request("/exempt/slow")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn prefix_match_does_not_leak_into_sibling_paths() {
        // "/slow" must not accidentally exempt "/slower"
        let config = TimeoutConfig::new(Duration::from_millis(20)).with_exempt_prefixes(["/slow"]);
        let resp = app(config).oneshot(request("/slower")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
    }

    #[tokio::test]
    async fn fast_path_passes_through() {
        let config = TimeoutConfig::new(Duration::from_secs(5));
        let app = Router::new()
            .route("/fast", get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(config, middleware));
        let resp = app.oneshot(request("/fast")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
