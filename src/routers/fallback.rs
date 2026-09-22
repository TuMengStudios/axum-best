use axum::extract::Request;
use tracing::warn;

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
