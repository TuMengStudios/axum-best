use std::sync::LazyLock;
use std::time::Duration;

use anyhow::Context;
use derivative::Derivative;
use regex::Regex;
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use tracing::info;

/// Matches the userinfo section of a DSN: `scheme://user:password@`
static DSN_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<prefix>[a-zA-Z][a-zA-Z0-9+.-]*://[^:/?#@]*):[^/?#]*@")
        .expect("dsn masking regex is valid")
});

/// MySQL database configuration
///
/// This struct holds all configuration parameters needed to establish
/// and manage a MySQL database connection pool.
#[derive(Derivative, Deserialize, Clone)]
#[derivative(Debug)]
pub struct MysqlConf {
    /// Database connection string (DSN)
    ///
    /// Format: mysql://username:password@host:port/database
    /// This field is ignored in Debug implementation for security reasons
    #[derivative(Debug = "ignore")]
    pub dsn: String,

    /// Maximum number of connections in the pool
    pub max_connections: u32,

    /// Log level for slow query logging
    ///
    /// Valid values: "trace", "debug", "info", "warn", "error"
    pub slow_level: String,

    /// Maximum lifetime of a connection in seconds
    ///
    /// Connections older than this will be closed when returned to the pool
    pub lifetime_sec: u64,

    /// Maximum idle time for connections in seconds
    ///
    /// Connections idle longer than this will be closed
    pub idle_sec: u64,

    /// Connection acquisition timeout in seconds
    ///
    /// Maximum time to wait when acquiring a connection from the pool
    pub acquire_timeout_sec: u64,

    /// Log level for connection timeout events
    ///
    /// Valid values: "trace", "debug", "info", "warn", "error"
    pub timeout_level: String,

    /// Threshold in milliseconds for slow query detection
    ///
    /// Queries taking longer than this threshold will be logged as slow queries
    pub slow_threshold_mills: u64,
}

impl MysqlConf {
    /// Converts the slow_level string to a log::LevelFilter
    ///
    /// Returns the corresponding log level filter for slow query logging.
    /// If the provided level is invalid, defaults to LevelFilter::Info.
    fn get_slow_level(&self) -> log::LevelFilter {
        match self.slow_level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            other => {
                println!("Invalid slow_level '{}', using default 'info'", other);
                log::LevelFilter::Info
            }
        }
    }

    /// Converts the timeout_level string to a log::LevelFilter
    ///
    /// Returns the corresponding log level filter for connection timeout events.
    /// If the provided level is invalid, defaults to LevelFilter::Warn.
    fn get_timeout_level(&self) -> log::LevelFilter {
        match self.timeout_level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            other => {
                println!("Invalid timeout_level '{}', using default 'warn'", other);
                log::LevelFilter::Warn
            }
        }
    }

    /// Converts lifetime_sec to Duration
    ///
    /// Returns the maximum connection lifetime as a Duration object.
    fn get_lifetime(&self) -> Duration {
        Duration::from_secs(self.lifetime_sec)
    }

    /// Converts idle_sec to Duration
    ///
    /// Returns the maximum idle timeout as a Duration object.
    fn get_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_sec)
    }

    /// Converts acquire_timeout_sec to Duration
    ///
    /// Returns the maximum time to wait for acquiring a database connection
    /// from the pool as a Duration object.
    ///
    /// This timeout prevents indefinite blocking when all connections are in use
    /// and no new connections can be created. If the timeout is reached,
    /// the connection acquisition will fail with a timeout error.
    ///
    /// # Returns
    /// - `Duration` representing the connection acquisition timeout
    fn get_acquire_timeout_sec(&self) -> Duration {
        Duration::from_secs(self.acquire_timeout_sec)
    }
    /// Converts slow_threshold_mills to Duration
    ///
    /// Returns the slow query threshold as a Duration object.
    fn get_slow_threshold(&self) -> Duration {
        Duration::from_millis(self.slow_threshold_mills)
    }

    /// Returns the DSN with the password replaced by `***`
    ///
    /// Keeps scheme, username, host, port, database and query parameters
    /// readable for diagnostics while ensuring the password cannot leak
    /// into logs or error messages.
    fn masked_dsn(&self) -> String {
        DSN_PASSWORD
            .replace(&self.dsn, "${prefix}:***@")
            .into_owned()
    }
}

impl MysqlConf {
    /// Initializes and returns a MySQL connection pool
    ///
    /// Creates a connection pool using the configuration parameters.
    /// Configures connection limits, timeouts, and logging levels.
    /// Performs a basic connection test to verify the pool is working.
    ///
    /// # Returns
    /// - `Ok(MySqlPool)` on successful pool creation
    /// - `Err(anyhow::Error)` if connection fails
    pub async fn init_conn(&self) -> anyhow::Result<MySqlPool> {
        info!("Initializing MySQL connection pool with config: {:?}", self);

        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .max_lifetime(self.get_lifetime())
            .idle_timeout(self.get_idle_timeout())
            .acquire_timeout(self.get_acquire_timeout_sec())
            .acquire_slow_level(self.get_slow_level())
            .acquire_slow_threshold(self.get_slow_threshold())
            .acquire_time_level(self.get_timeout_level())
            .connect(&self.dsn)
            .await
            .with_context(|| format!("connect mysql failed, dsn: {}", self.masked_dsn()))?;

        info!("MySQL connection pool initialized successfully");

        Ok(pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conf_with_dsn(dsn: &str) -> MysqlConf {
        MysqlConf {
            dsn: dsn.to_string(),
            max_connections: 5,
            slow_level: "info".to_string(),
            lifetime_sec: 30,
            idle_sec: 10,
            acquire_timeout_sec: 1,
            timeout_level: "warn".to_string(),
            slow_threshold_mills: 200,
        }
    }

    #[test]
    fn test_masked_dsn_hides_password() {
        let conf = conf_with_dsn("mysql://app:secret@127.0.0.1:3306/app_db");
        assert_eq!(conf.masked_dsn(), "mysql://app:***@127.0.0.1:3306/app_db");
    }

    #[test]
    fn test_masked_dsn_keeps_query_params() {
        let conf = conf_with_dsn("mysql://app:secret@127.0.0.1:3306/app_db?ssl-mode=required");
        assert_eq!(conf.masked_dsn(), "mysql://app:***@127.0.0.1:3306/app_db?ssl-mode=required");
    }

    #[test]
    fn test_masked_dsn_hides_at_sign_in_password() {
        let conf = conf_with_dsn("mysql://app:p@ss@127.0.0.1:3306/app_db");
        assert_eq!(conf.masked_dsn(), "mysql://app:***@127.0.0.1:3306/app_db");
    }

    #[test]
    fn test_masked_dsn_without_password_is_unchanged() {
        let conf = conf_with_dsn("mysql://app@127.0.0.1:3306/app_db");
        assert_eq!(conf.masked_dsn(), "mysql://app@127.0.0.1:3306/app_db");
    }

    #[test]
    fn test_masked_dsn_without_scheme_is_unchanged() {
        let conf = conf_with_dsn("app:secret@tcp(127.0.0.1:3306)/app_db");
        assert_eq!(conf.masked_dsn(), "app:secret@tcp(127.0.0.1:3306)/app_db");
    }

    #[tokio::test]
    async fn test_init_conn_error_contains_masked_dsn_and_cause() {
        let conf = conf_with_dsn("mysql://app:secret@127.0.0.1:3306/app_db?ssl-mode=invalid");
        let err = conf.init_conn().await.unwrap_err();
        let message = format!("{:#}", err);

        assert!(message.contains("dsn: mysql://app:***@127.0.0.1:3306/app_db?ssl-mode=invalid"));
        assert!(!message.contains("secret"));
        assert!(message.contains("unknown value"));
    }
}
