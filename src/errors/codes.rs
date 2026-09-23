use axum::http::StatusCode;
use lazy_static::lazy_static;

use crate::core::rest::AppError;

// Application error definitions
lazy_static! {
    /// Success response - request completed successfully
    pub static ref ErrOk: AppError = AppError::new(StatusCode::OK, 10000, "Success");

    /// Bad request - invalid request parameters
    pub static ref ErrBadRequest: AppError =
        AppError::new(StatusCode::BAD_REQUEST, 14000, "Bad Request Params");

    /// Too many requests - the request exceeded the configured rate limit
    pub static ref ErrTooManyRequests: AppError =
        AppError::new(StatusCode::TOO_MANY_REQUESTS, 42900, "Too Many Requests");

    /// Request timeout - the request exceeded its time budget (returned by
    /// the timeout middleware as a bare 408; also usable by handlers that
    /// enforce their own budgets)
    pub static ref ErrRequestTimeout: AppError =
        AppError::new(StatusCode::REQUEST_TIMEOUT, 40800, "Request Timeout");

    /// Method not allowed - the route exists but does not support the request method
    pub static ref ErrMethodNotAllowed: AppError =
        AppError::new(StatusCode::METHOD_NOT_ALLOWED, 40500, "Method Not Allowed");
}

lazy_static! {
    /// Redis client error - internal server error for Redis operations
    pub static ref ErrRedisClient: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50100, "Server Internal Error");
}

lazy_static! {
    /// Invalid user ID - unauthorized access attempt
    pub static ref ErrInvalidUserId: AppError = AppError::new(StatusCode::UNAUTHORIZED, 20000, "");

    /// Missing or invalid JWT credentials
    pub static ref ErrUnauthorized: AppError =
        AppError::new(StatusCode::UNAUTHORIZED, 20401, "Unauthorized");

    /// JWT token generation failed
    pub static ref ErrJwtTokenCreation: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 20402, "Server Internal Error");

    /// User account is abnormal (soft-deleted or disabled)
    pub static ref ErrUserAbnormal: AppError =
        AppError::new(StatusCode::UNAUTHORIZED, 20403, "User Abnormal");
}

lazy_static! {
    /// Not found - the requested route does not exist
    pub static ref ErrNotFound: AppError =
        AppError::new(StatusCode::NOT_FOUND, 40400, "Not Found");

    /// Not implemented - requested feature is not implemented
    pub static ref ErrNotImplemented: AppError =
        AppError::new(StatusCode::NOT_IMPLEMENTED, 50000, "Not Implemented");
}

// Database technical errors. The numeric error code identifies the failure
// category for operators and clients; the message intentionally stays generic
// so database details are never exposed through the API response.
lazy_static! {
    /// 50200: SQLx rejected an invalid database argument.
    pub static ref ErrDbInvalidArgument: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50200, "Server Internal Error");

    /// 50201: Database configuration or connection setup failed.
    pub static ref ErrDbConfiguration: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50201, "Server Internal Error");

    /// 50202: The database rejected a write because of a data conflict.
    pub static ref ErrDbDataConflict: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50202, "Server Internal Error");

    /// 50203: Database rejected a value because its length exceeded a limit.
    pub static ref ErrDbDataLengthExceeded: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50203, "Server Internal Error");

    /// 50204: Database rejected a numeric value outside its supported range.
    pub static ref ErrDbNumericRange: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50204, "Server Internal Error");

    /// 50205: Database rejected a row because a required field was missing.
    pub static ref ErrDbRequiredField: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50205, "Server Internal Error");

    /// 50206: Database rejected a row because of a foreign-key constraint.
    pub static ref ErrDbForeignKeyConstraint: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50206, "Server Internal Error");

    /// 50207: A required database table was not found.
    pub static ref ErrDbTableNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50207, "Server Internal Error");

    /// 50208: An uncategorized database operation failed.
    pub static ref ErrDbGeneric: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50208, "Server Internal Error");

    /// 50209: Database returned an error without a SQLSTATE.
    pub static ref ErrDbUnknown: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50209, "Server Internal Error");

    /// 50210: Database I/O failed.
    pub static ref ErrDbIo: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50210, "Server Internal Error");

    /// 50211: Database TLS negotiation failed.
    pub static ref ErrDbTls: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50211, "Server Internal Error");

    /// 50212: Database protocol handling failed.
    pub static ref ErrDbProtocol: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50212, "Server Internal Error");

    /// 50213: An expected database row was not found.
    pub static ref ErrDbRowNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50213, "Server Internal Error");

    /// 50214: A required database type was not found.
    pub static ref ErrDbTypeNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50214, "Server Internal Error");

    /// 50215: A database column index was outside the available range.
    pub static ref ErrDbColumnIndexOutOfBounds: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50215, "Server Internal Error");

    /// 50216: A required database column was not found.
    pub static ref ErrDbColumnNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50216, "Server Internal Error");

    /// 50217: A database column value could not be decoded.
    pub static ref ErrDbColumnDecode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50217, "Server Internal Error");

    /// 50218: A value could not be encoded for the database.
    pub static ref ErrDbEncode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50218, "Server Internal Error");

    /// 50219: A database response value could not be decoded.
    pub static ref ErrDbDecode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50219, "Server Internal Error");

    /// 50220: The database driver failed.
    pub static ref ErrDbDriver: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50220, "Server Internal Error");

    /// 50221: Acquiring a database connection timed out.
    pub static ref ErrDbPoolTimeout: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50221, "Server Internal Error");

    /// 50222: The database connection pool was closed.
    pub static ref ErrDbPoolClosed: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50222, "Server Internal Error");

    /// 50223: The database worker crashed.
    pub static ref ErrDbWorkerCrashed: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50223, "Server Internal Error");

    /// 50224: A database migration failed.
    pub static ref ErrDbMigration: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50224, "Server Internal Error");

    /// 50225: A database savepoint operation was invalid.
    pub static ref ErrDbInvalidSavePoint: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50225, "Server Internal Error");

    /// 50226: A database transaction could not be started.
    pub static ref ErrDbBeginFailed: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50226, "Server Internal Error");

    /// 50227: An unclassified SQLx database error occurred.
    pub static ref ErrDbUnknownError: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50227, "Server Internal Error");
}

lazy_static! {
    pub static ref ErrWechatLogin: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50500, "Server Internal Error");
    pub static ref ErrUnmarshalJSON: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50501, "Server Internal Error");
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::response::IntoResponse;
    use serde_json::Value;
    use std::collections::HashSet;

    #[tokio::test]
    async fn database_errors_use_one_safe_message_and_distinct_codes() {
        let errors = [
            &*super::ErrDbInvalidArgument,
            &*super::ErrDbConfiguration,
            &*super::ErrDbDataConflict,
            &*super::ErrDbDataLengthExceeded,
            &*super::ErrDbNumericRange,
            &*super::ErrDbRequiredField,
            &*super::ErrDbForeignKeyConstraint,
            &*super::ErrDbTableNotFound,
            &*super::ErrDbGeneric,
            &*super::ErrDbUnknown,
            &*super::ErrDbIo,
            &*super::ErrDbTls,
            &*super::ErrDbProtocol,
            &*super::ErrDbTypeNotFound,
            &*super::ErrDbColumnIndexOutOfBounds,
            &*super::ErrDbColumnNotFound,
            &*super::ErrDbColumnDecode,
            &*super::ErrDbEncode,
            &*super::ErrDbDecode,
            &*super::ErrDbDriver,
            &*super::ErrDbPoolTimeout,
            &*super::ErrDbPoolClosed,
            &*super::ErrDbWorkerCrashed,
            &*super::ErrDbMigration,
            &*super::ErrDbInvalidSavePoint,
            &*super::ErrDbBeginFailed,
            &*super::ErrDbUnknownError,
        ];
        let mut messages = HashSet::new();
        let mut codes = HashSet::new();

        for error in errors {
            let response = error.clone().into_response();
            let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let body: Value = serde_json::from_slice(&body).unwrap();
            messages.insert(body["err_msg"].as_str().unwrap().to_owned());
            codes.insert(body["err_no"].as_i64().unwrap());
        }

        assert_eq!(messages.len(), 1);
        assert_eq!(messages.into_iter().next().unwrap(), "Server Internal Error");
        assert_eq!(codes.len(), errors.len());
    }
}
