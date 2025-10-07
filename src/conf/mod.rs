use derivative::Derivative;
use serde::Deserialize;

use crate::core::state::WeChatConf;
use crate::data::cache::RedisConf;
use crate::data::mysql::MysqlConf;
use crate::logx::LogConfig;
use crate::transport::http::HttpConf;

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

    pub wechat: WeChatConf,
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
