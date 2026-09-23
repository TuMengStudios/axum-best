use std::time::Duration;

use derivative::Derivative;
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use tracing::info;

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
            .await?;

        info!("MySQL connection pool initialized successfully");

        Ok(pool)
    }
}
