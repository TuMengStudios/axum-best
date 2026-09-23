use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use opentelemetry::trace::TraceContextExt as _;
use tracing::Span;
use tracing::warn;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use crate::core::rest::AppError;
use crate::errors::ErrMethodNotAllowed;

/// Fallback handler for a known route requested with an unsupported method.
pub(super) async fn method_not_allowed(req: Request) -> AppError {
    warn!(
        method = %req.method(),
        path = %req.uri().path(),
        query = req.uri().query().unwrap_or(""),
        "fallback hit: method not allowed"
    );

    ErrMethodNotAllowed.clone()
}

/// Fallback handler for any unmatched route: logs the request and renders a
/// self-contained HTML `404` page (HTTP status 404). The trace id attached
/// to the current span by the OpenTelemetry middleware is echoed into the
/// page after HTML-escaping; when no valid trace context is present, the
/// trace block is hidden.
pub(super) async fn not_found(req: Request) -> Response {
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query();
    warn!(
        method = %method,
        path = %path,
        query = query.unwrap_or(""),
        "fallback hit: route not found"
    );

    let trace_id = current_trace_id().filter(|id| !id.is_empty());

    let (trace_html, trace_display) = match trace_id {
        Some(value) => (escape_html(&value), "block"),
        None => (String::new(), "none"),
    };

    let html = NOT_FOUND_HTML
        .replace("__TRACE_ID__", &trace_html)
        .replace("__TRACE_DISPLAY__", trace_display);

    (StatusCode::NOT_FOUND, Html(html)).into_response()
}

/// Reads the OpenTelemetry trace id recorded on the current span by the
/// `otel::middleware` layer.
fn current_trace_id() -> Option<String> {
    let context = Span::current().context();
    let otel_span = context.span();
    let span_context = otel_span.span_context();
    span_context
        .is_valid()
        .then(|| span_context.trace_id().to_string())
}

/// Minimal HTML escaping for values injected into the page.
/// Built from individual characters to keep the replacement tokens free of
/// HTML entities in the source.
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => {
                out.push('&');
                out.push_str("amp;");
            }
            '<' => {
                out.push('&');
                out.push_str("lt;");
            }
            '>' => {
                out.push('&');
                out.push_str("gt;");
            }
            '"' => {
                out.push('&');
                out.push_str("quot;");
            }
            '\'' => {
                out.push('&');
                out.push_str("#39;");
            }
            _ => out.push(c),
        }
    }
    out
}

const NOT_FOUND_HTML: &str = include_str!("./assets/404.html");
