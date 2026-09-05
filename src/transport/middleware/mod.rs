//! Middleware for the transport layer.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use tower_request_id::RequestId;
use tracing::Instrument;

/// Response header that carries the request id back to the caller
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware function that injects request ID into the response headers and tracing spans
///
/// This middleware:
/// - Extracts the request ID from the request extensions (inserted by the outermost
///   [`tower_request_id::RequestIdLayer`])
/// - Wraps the rest of the request handling in a tracing span carrying the request ID, so every log
///   emitted by downstream handlers and middleware includes it
/// - Adds the request ID to the response headers
/// - Falls back to "unknown" if no request ID is found
///
/// # Arguments
/// * `req` - The incoming HTTP request
/// * `next` - The next middleware/handler in the chain
///
/// # Returns
/// The HTTP response with request ID header added
pub async fn inject_request_id(req: Request, next: Next) -> Response {
    let request_id = request_id_of(&req);

    let span = tracing::info_span!("http_request", request_id = %request_id);

    let mut resp = next.run(req).instrument(span).await;
    match HeaderValue::from_str(&request_id) {
        Ok(header_value) => {
            resp.headers_mut().insert(REQUEST_ID_HEADER, header_value);
        }
        Err(err) => {
            tracing::error!(
                "failed to build {} header from '{}': {}",
                REQUEST_ID_HEADER,
                request_id,
                err
            );
        }
    }
    resp
}

/// [`tower_http::trace::TraceLayer`] span factory that carries the request id
///
/// Use it via `TraceLayer::make_span_with`. The `request` span wraps the whole request handling, so
/// the trace layer's own lifecycle logs (started/finished processing, body streaming, failures)
/// include the request id even though they run outside the `inject_request_id` future.
pub(crate) fn make_request_span<B>(request: &axum::http::Request<B>) -> tracing::Span {
    let request_id = request_id_of(request);
    tracing::info_span!(
        "request",
        method = %request.method(),
        uri = %request.uri(),
        version = ?request.version(),
        request_id = %request_id,
    )
}

/// Reads the request id inserted into the request extensions by
/// [`tower_request_id::RequestIdLayer`], falling back to "unknown" when it is missing
fn request_id_of<B>(req: &axum::http::Request<B>) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            tracing::warn!("request id extension missing, falling back to 'unknown'");
            "unknown".to_string()
        })
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::sync::Arc;
    use std::sync::Mutex;

    use axum::Router;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::http::StatusCode;
    use axum::middleware;
    use axum::routing::get;
    use tower::ServiceExt;
    use tower_http::trace::DefaultOnRequest;
    use tower_http::trace::DefaultOnResponse;
    use tower_http::trace::TraceLayer;
    use tracing::Level;

    use super::REQUEST_ID_HEADER;
    use super::inject_request_id;
    use super::make_request_span;

    async fn handler() -> &'static str {
        "ok"
    }

    fn app(with_request_id_layer: bool) -> Router {
        let router = Router::new().route("/", get(handler));
        let router = router.layer(middleware::from_fn(inject_request_id));
        if with_request_id_layer {
            router.layer(tower_request_id::RequestIdLayer)
        } else {
            router
        }
    }

    fn request() -> HttpRequest<Body> {
        HttpRequest::builder().uri("/").body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn response_header_contains_request_id() {
        let resp = app(true).oneshot(request()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let value = resp
            .headers()
            .get(REQUEST_ID_HEADER)
            .expect("request id header must be present")
            .to_str()
            .unwrap();
        assert!(!value.is_empty());
        assert_ne!(value, "unknown");
    }

    #[tokio::test]
    async fn response_header_falls_back_without_request_id_layer() {
        let resp = app(false).oneshot(request()).await.unwrap();
        assert_eq!(resp.headers().get(REQUEST_ID_HEADER).unwrap(), "unknown");
    }

    #[test]
    fn full_request_logs_carry_request_id() {
        let (logs, resp) = capture_logs(|| {
            let app = Router::new()
                .route(
                    "/",
                    get(|| async {
                        tracing::info!("handling request");
                        "ok"
                    }),
                )
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(make_request_span)
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(middleware::from_fn(inject_request_id))
                .layer(tower_request_id::RequestIdLayer);

            app.oneshot(request())
        });
        let resp = resp.unwrap();
        let request_id = resp
            .headers()
            .get(REQUEST_ID_HEADER)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        assert!(
            logs.contains("started processing request"),
            "trace on_request log missing: {logs}"
        );
        assert!(
            logs.contains("finished processing request"),
            "trace on_response log missing: {logs}"
        );
        assert!(logs.contains("handling request"), "handler log missing: {logs}");

        // every log line emitted while serving the request must carry the same request id
        let lines: Vec<&str> = logs.lines().collect();
        assert!(!lines.is_empty(), "no logs captured: {logs}");
        for line in &lines {
            assert!(
                line.contains(&request_id),
                "log line without request_id '{request_id}': {line}"
            );
        }
    }

    /// Runs `f` on a current-thread runtime inside a JSON subscriber writing into an in-memory
    /// buffer, returning the captured log lines and the future output
    fn capture_logs<T, F, Fut>(f: F) -> (String, T)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
        let writer_buffer = buffer.clone();
        let subscriber = tracing_subscriber::fmt()
            .json()
            .with_ansi(false)
            .with_writer(move || SharedWriter(writer_buffer.clone()))
            .finish();

        let mut output = None;
        tracing::subscriber::with_default(subscriber, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            output = Some(rt.block_on(f()));
        });

        let logs = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
        (logs, output.unwrap())
    }

    /// In-memory writer shared between the test and the tracing subscriber
    #[derive(Clone)]
    struct SharedWriter(Arc<Mutex<Vec<u8>>>);

    impl std::io::Write for SharedWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
