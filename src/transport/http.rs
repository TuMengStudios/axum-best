use serde::Deserialize;
use smart_default::SmartDefault;
use tokio::net::TcpListener;
use tracing::info;

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
}

impl HttpConf {
    /// Constructs the full address string in the format "host:port"
    ///
    /// # Returns
    /// A string containing the formatted address (e.g., "0.0.0.0:8080")
    pub fn address(&self) -> String {
        format!("{}:{}", self.listen, self.port)
    }
}

impl HttpConf {
    /// Creates and binds a TCP listener using the configured address and port
    ///
    /// # Returns
    /// - `Ok(TcpListener)` if the listener was successfully created and bound
    /// - `Err(anyhow::Error)` if binding failed
    pub async fn build_listener(&self) -> anyhow::Result<TcpListener> {
        println!("try to listen {:?}", self.address());
        let listener = TcpListener::bind(self.address()).await?;
        if self.address().starts_with("0.0.0.0") {
            match local_ip_address::local_ip() {
                Ok(ip) => {
                    // Start HTTP server
                    println!("Starting HTTP server on http://{}:{}", ip, self.port);
                    info!("Starting HTTP server on http://{}:{}", ip, self.port);
                }
                Err(_err) => {
                    // Start HTTP server
                    println!("Starting HTTP server on http://{}", self.address(),);
                    info!("Starting HTTP server on http://{}", self.address(),);
                }
            }
        } else {
            // Start HTTP server
            println!("Starting HTTP server on http://{}", self.address(),);
            tracing::info!("Starting HTTP server on http://{}", self.address(),);
        }

        println!("listen {} success", self.address());
        Ok(listener)
    }
}
