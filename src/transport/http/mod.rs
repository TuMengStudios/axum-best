use serde::Deserialize;
use smart_default::SmartDefault;
use tokio::net::TcpListener;

/// HTTP server configuration
///
/// This struct defines the configuration settings for the HTTP server,
/// including the listening address and port.
#[derive(Debug, Deserialize, SmartDefault, Clone)]
pub struct HttpConf {
    /// The IP address or hostname to listen on
    ///
    /// Defaults to "0.0.0.0" to listen on all available network interfaces
    #[default("0.0.0.0")]
    pub listen: String,

    /// The port number to listen on
    ///
    /// Defaults to 8080, which is a common development port
    #[default(8080)]
    pub port: u16,

    /// Request time budget enforced by the timeout middleware, in seconds
    ///
    /// Defaults to 30 seconds. The budget covers request handling including
    /// response compression, but not CORS bookkeeping.
    #[default(30)]
    pub timeout_secs: u64,

    /// Path prefixes excluded from the request timeout middleware
    ///
    /// Segment-aware prefix match: "/stream" excludes "/stream" and "/stream/1"
    /// but not "/streaming". Useful for SSE streams, file uploads or reports.
    /// Defaults to an empty list (no exclusions).
    #[serde(default)]
    pub timeout_excluded_paths: Vec<String>,

    /// Path prefixes whose responses skip the compression middleware
    ///
    /// Segment-aware prefix match, same semantics as `timeout_excluded_paths`.
    /// Useful for endpoints that stream (SSE) or already emit compressed
    /// content. Defaults to an empty list (compress everything compressible).
    #[serde(default)]
    pub compression_excluded_paths: Vec<String>,

    /// Origins allowed to make cross-origin requests.
    ///
    /// Set to `['*']` to allow any origin. When using an explicit list,
    /// credentialed requests are allowed; wildcard mode intentionally does
    /// not allow credentials because browsers reject that combination.
    #[serde(default)]
    pub cors_allowed_origins: Vec<String>,
}

impl HttpConf {
    /// Constructs the full address string in the format "host:port"
    ///
    /// # Returns
    /// A string containing the formatted address (e.g., "0.0.0.0:8080")
    fn address(&self) -> String {
        format!("{}:{}", self.listen, self.port)
    }

    /// Creates and binds a TCP listener using the configured address and port
    ///
    /// # Returns
    /// - `Ok(TcpListener)` if the listener was successfully created and bound
    /// - `Err(anyhow::Error)` if binding failed
    pub async fn build_listener(&self) -> anyhow::Result<TcpListener> {
        let address = self.address();
        println!("try to listen {address}");
        let listener = match TcpListener::bind(&address).await {
            Ok(listener) => listener,
            Err(err) => return Err(anyhow::anyhow!("listen {address} failed: {err}")),
        };

        let display_addr = if self.listen == "0.0.0.0" {
            local_ip_address::local_ip()
                .map(|ip| format!("{ip}:{}", self.port))
                .unwrap_or(address)
        } else {
            address
        };
        println!("Starting HTTP server on http://{display_addr}");
        println!("listen {display_addr} success");

        Ok(listener)
    }
}
