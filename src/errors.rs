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
}

lazy_static! {
    /// Redis client error - internal server error for Redis operations
    pub static ref ErrRedisClient: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50100, "Server Internal Error");
}

lazy_static! {
    /// Invalid user ID - unauthorized access attempt
    pub static ref ErrInvalidUserId: AppError = AppError::new(StatusCode::UNAUTHORIZED, 20000, "");
}

lazy_static! {
    /// Not implemented - requested feature is not implemented
    pub static ref ErrNotImplemented: AppError =
        AppError::new(StatusCode::NOT_IMPLEMENTED, 50000, "Not Implemented");
}

// Database specific errors
lazy_static! {
    /// Database invalid argument error - internal server error for invalid database arguments
    pub static ref ErrDbInvalidArgument: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50200, "Server Internal Error");

    /// Database configuration error - internal server error for database configuration issues
    pub static ref ErrDbConfiguration: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50201, "Server Internal Error");

    /// Database data conflict error - conflict error for data constraint violations
    pub static ref ErrDbDataConflict: AppError =
        AppError::new(StatusCode::CONFLICT, 50202, "Server Internal Error");

    /// Database data length exceeded error - bad request for data exceeding length limits
    pub static ref ErrDbDataLengthExceeded: AppError =
        AppError::new(StatusCode::BAD_REQUEST, 50203, "Server Internal Error");

    /// Database numeric range error - bad request for numeric value out of range
    pub static ref ErrDbNumericRange: AppError =
        AppError::new(StatusCode::BAD_REQUEST, 50204, "Server Internal Error");

    /// Database required field error - bad request for missing required fields
    pub static ref ErrDbRequiredField: AppError =
        AppError::new(StatusCode::BAD_REQUEST, 50205, "Server Internal Error");

    /// Database foreign key constraint error - bad request for foreign key violations
    pub static ref ErrDbForeignKeyConstraint: AppError =
        AppError::new(StatusCode::BAD_REQUEST, 50206, "Server Internal Error");

    /// Database table not found error - internal server error for missing database tables
    pub static ref ErrDbTableNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50207, "Server Internal Error");

    /// Database generic error - internal server error for general database failures
    pub static ref ErrDbGeneric: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50208, "Server Internal Error");

    /// Database unknown error - internal server error for unknown database issues
    pub static ref ErrDbUnknown: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50209, "Server Internal Error");

    /// Database I/O error - internal server error for database I/O operations
    pub static ref ErrDbIo: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50210, "Server Internal Error");

    /// Database TLS error - internal server error for TLS connection issues
    pub static ref ErrDbTls: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50211, "Server Internal Error");

    /// Database protocol error - internal server error for protocol violations
    pub static ref ErrDbProtocol: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50212, "Server Internal Error");

    /// Database row not found error - not found error for missing database records
    pub static ref ErrDbRowNotFound: AppError =
        AppError::new(StatusCode::NOT_FOUND, 50213, "Record Not Found");

    /// Database type not found error - internal server error for missing database types
    pub static ref ErrDbTypeNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50214, "Server Internal Error");

    /// Database column index out of bounds error - internal server error for invalid column indices
    pub static ref ErrDbColumnIndexOutOfBounds: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50215, "Server Internal Error");

    /// Database column not found error - internal server error for missing columns
    pub static ref ErrDbColumnNotFound: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50216, "Server Internal Error");

    /// Database column decode error - internal server error for column data decoding failures
    pub static ref ErrDbColumnDecode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50217, "Server Internal Error");

    /// Database encode error - internal server error for data encoding failures
    pub static ref ErrDbEncode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50218, "Server Internal Error");

    /// Database decode error - internal server error for data decoding failures
    pub static ref ErrDbDecode: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50219, "Server Internal Error");

    /// Database driver error - internal server error for database driver issues
    pub static ref ErrDbDriver: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50220, "Server Internal Error");

    /// Database pool timeout error - service unavailable for connection pool timeouts
    pub static ref ErrDbPoolTimeout: AppError =
        AppError::new(StatusCode::SERVICE_UNAVAILABLE, 50221, "Server Internal Error");

    /// Database pool closed error - service unavailable for closed connection pools
    pub static ref ErrDbPoolClosed: AppError =
        AppError::new(StatusCode::SERVICE_UNAVAILABLE, 50222, "Server Internal Error");

    /// Database worker crashed error - internal server error for worker thread crashes
    pub static ref ErrDbWorkerCrashed: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50223, "Server Internal Error");

    /// Database migration error - internal server error for migration failures
    pub static ref ErrDbMigration: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50224, "Server Internal Error");

    /// Database invalid save point error - internal server error for invalid save points
    pub static ref ErrDbInvalidSavePoint: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50225, "Server Internal Error");

    /// Database begin failed error - internal server error for transaction start failures
    pub static ref ErrDbBeginFailed: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50226, "Server Internal Error");

    /// Database unknown error - internal server error for unknown database errors
    pub static ref ErrDbUnknownError: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50227, "Server Internal Error");
}

lazy_static! {
    pub static ref ErrWechatLogin: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50500, "Server Internal Error");
    pub static ref ErrUnmarshalJSON: AppError =
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50501, "Server Internal Error");
}
