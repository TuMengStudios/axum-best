use std::time::Duration;

use derivative::Derivative;
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use tracing::error;
use tracing::info;

use crate::core::rest::AppError;
use crate::errors;
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
            #[rustfmt::skip]
            other @ _ => {
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

    /// Creates a default MySQL configuration
    ///
    /// Returns a MysqlConf instance with sensible default values
    /// suitable for development environments.
    pub fn default() -> Self {
        Self {
            dsn: "mysql://username:password@localhost:3306/database".to_string(),
            max_connections: 10,
            slow_level: "info".to_string(),
            lifetime_sec: 1800,      // 30 minutes
            idle_sec: 600,           // 10 minutes
            acquire_timeout_sec: 30, // 30 seconds
            timeout_level: "warn".to_string(),
            slow_threshold_mills: 2000, // 2 seconds
        }
    }
}

/// Database connection manager
///
/// Wraps a MySQL connection pool and provides convenient access methods.
/// This struct manages the lifecycle of database connections.
pub struct DbManager {
    pool: MySqlPool,
}

impl DbManager {
    /// Creates a new DbManager with the given connection pool
    ///
    /// # Arguments
    /// * `pool` - A MySQL connection pool to manage
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Returns a reference to the managed connection pool
    ///
    /// This method allows borrowing the pool without taking ownership.
    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    /// Consumes the DbManager and returns the underlying connection pool
    ///
    /// This method transfers ownership of the pool to the caller.
    pub fn into_pool(self) -> MySqlPool {
        self.pool
    }
}

pub fn covert_error(err: sqlx::Error) -> AppError {
    match err {
        sqlx::Error::Configuration(e) => {
            error!("db config error: {:?}", e);
            errors::ErrDbConfiguration.clone()
        }
        sqlx::Error::InvalidArgument(info) => {
            error!("db invalid argument: {}", info);
            errors::ErrDbInvalidArgument.clone()
        }
        sqlx::Error::Database(database_error) => {
            error!("database error: {}", database_error);
            // 根据数据库错误代码返回不同的预定义错误
            if let Some(code) = database_error.code() {
                match code.as_ref() {
                    "23000" | "23505" => {
                        error!("database data conflict: {}", database_error);
                        errors::ErrDbDataConflict.clone()
                    }
                    "22001" => {
                        error!("database data length exceeded: {}", database_error);
                        errors::ErrDbDataLengthExceeded.clone()
                    }
                    "22003" => {
                        error!("database numeric range error: {}", database_error);
                        errors::ErrDbNumericRange.clone()
                    }
                    "23502" => {
                        error!("database required field missing: {}", database_error);
                        errors::ErrDbRequiredField.clone()
                    }
                    "23503" => {
                        error!("database foreign key constraint: {}", database_error);
                        errors::ErrDbForeignKeyConstraint.clone()
                    }
                    "42S02" => {
                        error!("database table not found: {}", database_error);
                        errors::ErrDbTableNotFound.clone()
                    }
                    _ => {
                        error!(
                            "database generic error: code={}, message={}",
                            code,
                            database_error.message()
                        );
                        errors::ErrDbGeneric.clone()
                    }
                }
            } else {
                error!("database unknown error: {}", database_error);
                errors::ErrDbUnknown.clone()
            }
        }
        sqlx::Error::Io(e) => {
            error!("database IO error: {}", e);
            errors::ErrDbIo.clone()
        }
        sqlx::Error::Tls(e) => {
            error!("database TLS error: {}", e);
            errors::ErrDbTls.clone()
        }
        sqlx::Error::Protocol(e) => {
            error!("database protocol error: {}", e);
            errors::ErrDbProtocol.clone()
        }
        sqlx::Error::RowNotFound => {
            error!("database row not found");
            errors::ErrDbRowNotFound.clone()
        }
        sqlx::Error::TypeNotFound { type_name } => {
            error!("database type not found: {}", type_name);
            errors::ErrDbTypeNotFound.clone()
        }
        sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
            error!("database column index out of bounds: index={}, len={}", index, len);
            errors::ErrDbColumnIndexOutOfBounds.clone()
        }
        sqlx::Error::ColumnNotFound(column) => {
            error!("database column not found: {}", column);
            errors::ErrDbColumnNotFound.clone()
        }
        sqlx::Error::ColumnDecode { index, source } => {
            error!("database column decode error at index {}: {}", index, source);
            errors::ErrDbColumnDecode.clone()
        }
        sqlx::Error::Encode(e) => {
            error!("database encode error: {}", e);
            errors::ErrDbEncode.clone()
        }
        sqlx::Error::Decode(e) => {
            error!("database decode error: {}", e);
            errors::ErrDbDecode.clone()
        }
        sqlx::Error::AnyDriverError(e) => {
            error!("database driver error: {}", e);
            errors::ErrDbDriver.clone()
        }
        sqlx::Error::PoolTimedOut => {
            error!("database pool timeout");
            errors::ErrDbPoolTimeout.clone()
        }
        sqlx::Error::PoolClosed => {
            error!("database pool closed");
            errors::ErrDbPoolClosed.clone()
        }
        sqlx::Error::WorkerCrashed => {
            error!("database worker crashed");
            errors::ErrDbWorkerCrashed.clone()
        }
        sqlx::Error::Migrate(e) => {
            error!("database migration error: {}", e);
            errors::ErrDbMigration.clone()
        }
        sqlx::Error::InvalidSavePointStatement => {
            error!("database invalid savepoint statement");
            errors::ErrDbInvalidSavePoint.clone()
        }
        sqlx::Error::BeginFailed => {
            error!("database transaction begin failed");
            errors::ErrDbBeginFailed.clone()
        }
        _ => {
            error!("unknown database error: {}", err);
            errors::ErrDbUnknownError.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use super::*;

    #[test]
    fn test_covert_error_row_not_found() {
        let err = sqlx::Error::RowNotFound;
        let app_error = covert_error(err);

        // 通过 IntoResponse 转换来验证错误内容
        let response = app_error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_covert_error_pool_timeout() {
        let err = sqlx::Error::PoolTimedOut;
        let app_error = covert_error(err);

        let response = app_error.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_covert_error_configuration() {
        let err = sqlx::Error::Configuration("test config error".into());
        let app_error = covert_error(err);

        let response = app_error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
