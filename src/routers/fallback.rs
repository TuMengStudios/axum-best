use axum::extract::Request;
use tracing::warn;

/// Fallback handler for any unmatched route: logs the request and returns the
/// shared `ErrNotImplemented` (err_no `50000`) so the unified error envelope is
/// preserved.
pub(super) async fn not_implemented(req: Request) -> crate::core::Result<()> {
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query();
    warn!(
        method = %method,
        path = %path,
        query = query.unwrap_or(""),
        "fallback hit: route not implemented"
    );
    Err(crate::errors::ErrNotImplemented.clone())
}
