use derivative::Derivative;
use serde::Deserialize;

use crate::auth::JwtConfig;
use crate::core::worker_dispatcher::WorkerConf;
use crate::data::cache::RedisConf;
use crate::data::mysql::MysqlConf;
use crate::data::wechat::WeChatConf;
use crate::logx::LogConfig;
use crate::observability::OpenTelemetryConfig;
use crate::transport::HttpConf;

/// Prometheus metrics endpoint and its protection settings.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MetricsConf {
    /// Metrics endpoint path. The route is disabled when this is absent or empty.
    #[serde(default)]
    pub path: Option<String>,
}

/// Swagger UI and OpenAPI document exposure settings.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SwaggerConf {
    /// Exposes Swagger UI and the OpenAPI document when enabled.
    #[serde(default)]
    pub enabled: bool,
}

impl MetricsConf {
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref().filter(|path| !path.is_empty())
    }
}

/// Application configuration structure
///
/// This struct represents the complete configuration for the Axum Best application.
/// It aggregates all configuration sections including logging, HTTP server, and database settings.
/// The configuration is typically loaded from a TOML file and can be deserialized from it.
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct AppConf {
    /// Logging configuration section
    ///
    /// Contains settings for log levels, rotation, file output, and formatting.
    /// Controls how application logs are generated and stored.
    pub log: LogConfig,

    /// OpenTelemetry configuration. Without an endpoint, traces remain in local logs.
    #[serde(default)]
    pub otel: OpenTelemetryConfig,

    /// Prometheus metrics configuration. The route is disabled by default.
    #[serde(default)]
    pub metrics: MetricsConf,

    /// Swagger configuration. Disabled by default, including production deployments.
    #[serde(default)]
    pub swagger: SwaggerConf,

    /// Background task pool configuration. Absent sections use the defaults.
    #[serde(default)]
    pub worker: WorkerConf,

    /// HTTP server configuration section
    ///
    /// Defines server settings such as listen address, port number, and other HTTP-related
    /// options. Controls how the web server accepts and handles incoming requests.
    pub http: HttpConf,

    /// MySQL database configuration section
    ///
    /// Contains database connection parameters, connection pool settings, and query performance
    /// monitoring. Manages the connection to the MySQL database and connection pool behavior.
    pub mysql: MysqlConf,

    /// redis config
    pub redis: RedisConf,

    /// JWT authentication configuration
    #[derivative(Debug = "ignore")]
    pub jwt: JwtConfig,

    /// WeChat mini-program configuration
    pub(crate) wechat: WeChatConf,
}

impl AppConf {
    /// Loads application configuration from a TOML file
    ///
    /// Reads the configuration file from the specified path and deserializes it
    /// into an `AppConf` instance. The file should be in TOML format and contain
    /// all required configuration sections.
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file (relative or absolute)
    ///
    /// # Returns
    /// - `Ok(AppConf)` on successful configuration loading and parsing
    /// - `Err(anyhow::Error)` if file cannot be read or TOML parsing fails
    ///
    /// # Errors
    /// - Returns error if the configuration file cannot be opened or read
    /// - Returns error if the TOML content cannot be parsed into AppConf
    pub fn from_path(path: &str) -> anyhow::Result<AppConf> {
        let content = std::fs::read_to_string(path)
            .map_err(|err| anyhow::anyhow!("open file {} error {:?}", path, err))?;

        toml::from_str(&content).map_err(|err| anyhow::anyhow!("parser file error {:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::{MetricsConf, SwaggerConf};
    use crate::observability::OpenTelemetryConfig;

    #[test]
    fn default_metrics_config_does_not_expose_a_route() {
        let config = MetricsConf::default();

        assert!(config.path().is_none());
    }

    #[test]
    fn metrics_settings_are_configurable() {
        let config: MetricsConf = toml::from_str("path = \"/internal/metrics\"")
            .expect("metrics configuration should parse");

        assert_eq!(config.path(), Some("/internal/metrics"));
    }

    #[test]
    fn swagger_is_disabled_by_default() {
        assert!(!SwaggerConf::default().enabled);
    }

    #[test]
    fn swagger_settings_are_configurable() {
        let config: SwaggerConf =
            toml::from_str("enabled = true").expect("swagger configuration should parse");

        assert!(config.enabled);
    }

    #[test]
    fn default_otel_config_keeps_export_disabled() {
        assert!(OpenTelemetryConfig::default().endpoint.is_none());
    }
}
