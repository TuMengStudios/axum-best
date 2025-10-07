use tracing::debug;

use crate::core::Result;
use crate::ok;

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
