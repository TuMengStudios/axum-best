//! HTTP request span middleware for OpenTelemetry.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use opentelemetry::global;
use opentelemetry::propagation::{Extractor, TextMapPropagator};
use opentelemetry::trace::TraceContextExt;
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Response header carrying the current OpenTelemetry trace id.
pub const TRACE_ID_HEADER: &str = "x-trace-id";

struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|key| key.as_str()).collect()
    }
}

/// Creates a span with standard HTTP request attributes and records the response status.
pub async fn middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let parent_context = global::get_text_map_propagator(|propagator: &dyn TextMapPropagator| {
        propagator.extract(&HeaderExtractor(req.headers()))
    });
    let span = tracing::info_span!(
        "http.server",
        http.request.method = %method,
        url.path = %uri.path(),
        url.query = ?uri.query(),
        http.request.scheme = ?uri.scheme_str(),
        http.response.status_code = tracing::field::Empty,
        trace_id = tracing::field::Empty,
        span_id = tracing::field::Empty,
    );
    span.set_parent(parent_context);
    let context = span.context();
    let context_span = context.span();
    let span_context = context_span.span_context();
    let trace_id = span_context.is_valid().then(|| {
        let trace_id = span_context.trace_id().to_string();
        span.record("trace_id", trace_id.as_str());
        span.record("span_id", span_context.span_id().to_string());
        trace_id
    });

    let mut response = next.run(req).instrument(span.clone()).await;
    span.record("http.response.status_code", response.status().as_u16());
    if let Some(trace_id) = trace_id.as_deref() {
        insert_trace_id_header(&mut response, trace_id);
    }
    response
}

fn insert_trace_id_header(response: &mut Response, trace_id: &str) {
    match HeaderValue::from_str(trace_id) {
        Ok(value) => {
            response.headers_mut().insert(TRACE_ID_HEADER, value);
        }
        Err(error) => {
            tracing::warn!(%error, "failed to encode trace id response header");
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::Request, middleware, response::Response, routing::get};
    use tower::ServiceExt;

    use super::{TRACE_ID_HEADER, insert_trace_id_header, middleware};

    #[tokio::test]
    async fn records_request_without_changing_response() {
        let app = Router::new()
            .route("/health", get(|| async { "ok" }))
            .layer(middleware::from_fn(middleware));
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .expect("request should build");
        let response = app.oneshot(request).await.expect("request should complete");
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn preserves_response_body() {
        let app = Router::new()
            .route("/health", get(|| async { "ok" }))
            .layer(middleware::from_fn(middleware));
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .expect("request should build");
        let response = app.oneshot(request).await.expect("request should complete");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should be readable");
        assert_eq!(bytes.as_ref(), b"ok");
    }

    #[tokio::test]
    async fn adds_trace_id_to_response_header() {
        let mut response = Response::new(Body::from("ok"));
        insert_trace_id_header(&mut response, "0000000000000000000000000000002a");

        assert_eq!(response.headers()[TRACE_ID_HEADER], "0000000000000000000000000000002a");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should be readable");
        assert_eq!(bytes.as_ref(), b"ok");
    }

    #[test]
    fn skips_invalid_trace_id_header_value() {
        let mut response = Response::new(Body::empty());
        insert_trace_id_header(&mut response, "invalid\ntrace-id");

        assert!(response.headers().get(TRACE_ID_HEADER).is_none());
    }
}
