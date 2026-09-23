use crate::core::rest::AppError;
use crate::errors;
use tracing::debug;

/// Converts SQLx errors into stable application errors.
///
/// The application error code identifies the technical category, while the
/// original SQLx error is retained for server-side diagnostics only.
pub(crate) fn covert_error(err: sqlx::Error) -> AppError {
    let (app_error, detail) = match &err {
        sqlx::Error::Configuration(configuration) => {
            (errors::ErrDbConfiguration.clone(), format!("database configuration: {configuration}"))
        }
        sqlx::Error::InvalidArgument(argument) => {
            (errors::ErrDbInvalidArgument.clone(), format!("database invalid argument: {argument}"))
        }
        sqlx::Error::Database(database_error) => {
            let app_error = database_error
                .code()
                .map(|code| map_database_error_code(code.as_ref()))
                .unwrap_or_else(|| errors::ErrDbUnknown.clone());
            (app_error, format!("database operation: {database_error}"))
        }
        sqlx::Error::Io(io_error) => (errors::ErrDbIo.clone(), format!("database I/O: {io_error}")),
        sqlx::Error::Tls(tls_error) => {
            (errors::ErrDbTls.clone(), format!("database TLS: {tls_error}"))
        }
        sqlx::Error::Protocol(protocol_error) => {
            (errors::ErrDbProtocol.clone(), format!("database protocol: {protocol_error}"))
        }
        sqlx::Error::RowNotFound => {
            (errors::ErrDbRowNotFound.clone(), "database row lookup".to_string())
        }
        sqlx::Error::TypeNotFound { type_name } => {
            (errors::ErrDbTypeNotFound.clone(), format!("database type lookup: {type_name}"))
        }
        sqlx::Error::ColumnIndexOutOfBounds { index, len } => (
            errors::ErrDbColumnIndexOutOfBounds.clone(),
            format!("database column index lookup: index={index}, len={len}"),
        ),
        sqlx::Error::ColumnNotFound(column) => {
            (errors::ErrDbColumnNotFound.clone(), format!("database column {column} lookup"))
        }
        sqlx::Error::ColumnDecode { index, source } => (
            errors::ErrDbColumnDecode.clone(),
            format!("database column decode: index={index}, source={source}"),
        ),
        sqlx::Error::Encode(encode_error) => {
            (errors::ErrDbEncode.clone(), format!("database value encode: {encode_error}"))
        }
        sqlx::Error::Decode(decode_error) => {
            (errors::ErrDbDecode.clone(), format!("database value decode: {decode_error}"))
        }
        sqlx::Error::AnyDriverError(driver_error) => {
            (errors::ErrDbDriver.clone(), format!("database driver: {driver_error}"))
        }
        sqlx::Error::PoolTimedOut => (
            errors::ErrDbPoolTimeout.clone(),
            "database connection pool acquire timed out".to_string(),
        ),
        sqlx::Error::PoolClosed => {
            (errors::ErrDbPoolClosed.clone(), "database connection pool is closed".to_string())
        }
        sqlx::Error::WorkerCrashed => {
            (errors::ErrDbWorkerCrashed.clone(), "database worker crashed".to_string())
        }
        sqlx::Error::Migrate(migration_error) => {
            (errors::ErrDbMigration.clone(), format!("database migration: {migration_error}"))
        }
        sqlx::Error::InvalidSavePointStatement => (
            errors::ErrDbInvalidSavePoint.clone(),
            "database savepoint statement is invalid".to_string(),
        ),
        sqlx::Error::BeginFailed => {
            (errors::ErrDbBeginFailed.clone(), "database transaction begin failed".to_string())
        }
        other_error => (
            errors::ErrDbUnknownError.clone(),
            format!("unclassified database error: {other_error}"),
        ),
    };

    // Always attach the complete SQLx error. Only the stable application code
    // and generic message are serialized; the cause is logged server-side.
    app_error.with_cause(err, detail)
}

/// Maps database-specific error codes to stable application error codes.
///
/// PostgreSQL uses SQLSTATE values, while MySQL drivers commonly expose the
/// numeric server error code. Neither value is returned to the client.
fn map_database_error_code(code: &str) -> AppError {
    match code {
        // PostgreSQL SQLSTATE and MySQL duplicate/constraint codes.
        "23000" | "23505" | "1062" | "1022" => errors::ErrDbDataConflict.clone(),
        // PostgreSQL string-data-right-truncation and MySQL data-too-long.
        "22001" | "1406" => errors::ErrDbDataLengthExceeded.clone(),
        // PostgreSQL numeric-value-out-of-range and MySQL out-of-range values.
        "22003" | "1264" | "1690" => errors::ErrDbNumericRange.clone(),
        // PostgreSQL not-null violation and MySQL cannot-be-null.
        "23502" | "1048" => errors::ErrDbRequiredField.clone(),
        // PostgreSQL foreign-key violation and MySQL foreign-key errors.
        "23503" | "1451" | "1452" => errors::ErrDbForeignKeyConstraint.clone(),
        // PostgreSQL undefined-table and MySQL table-not-found.
        "42P01" | "42S02" | "1146" => errors::ErrDbTableNotFound.clone(),
        unknown_code => {
            debug!(database_code = unknown_code, "unclassified database error code");
            errors::ErrDbGeneric.clone()
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
        let app_error = covert_error(sqlx::Error::RowNotFound);
        assert_eq!(app_error.into_response().status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_covert_error_pool_timeout() {
        let app_error = covert_error(sqlx::Error::PoolTimedOut);
        assert_eq!(app_error.into_response().status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_covert_error_configuration() {
        let app_error = covert_error(sqlx::Error::Configuration("test config error".into()));
        assert_eq!(app_error.into_response().status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_database_error_codes_support_postgresql_and_mysql() {
        assert_error_code("23505", 50202).await;
        assert_error_code("1062", 50202).await;
        assert_error_code("23502", 50205).await;
        assert_error_code("1048", 50205).await;
        assert_error_code("23503", 50206).await;
        assert_error_code("1452", 50206).await;
        assert_error_code("42P01", 50207).await;
        assert_error_code("1146", 50207).await;
        assert_error_code("unknown", 50208).await;
    }

    async fn assert_error_code(database_code: &str, expected: i64) {
        let response = map_database_error_code(database_code).into_response();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["err_no"], expected);
    }
}
