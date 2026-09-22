use tracing::debug;

use crate::core::Result;
use crate::ok;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    summary = "Check service health",
    description = "Checks whether the HTTP service is running normally. This endpoint does not require authentication and returns `ok` on success.",
    responses((status = 200, description = "Service is healthy", body = String))
)]
/// Health check endpoint handler
///
/// This function provides a simple health check endpoint that returns "ok"
/// to indicate the service is running and healthy.
///
/// # Returns
/// - `Result<&'static str>`: A successful result containing the string "ok"
pub async fn health() -> Result<&'static str> {
    debug!("health");
    ok!("ok")
}
