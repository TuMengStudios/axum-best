use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use derivative::Derivative;
use serde::Serialize;
use std::sync::Arc;
use tracing::error;

/// Unified handler/service result type: `Ok(AppResult<T>)` / `Err(AppError)`.
pub type Result<T> = core::result::Result<AppResult<T>, AppError>;

/// ok!(a) equal Ok(AppResult(a))
#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        Ok($crate::core::rest::AppResult($expr))
    };
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct AppError {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    status: StatusCode,
    err_no: i64,
    err_msg: String,
    // The original error and context are only used for server-side logging.
    #[serde(skip)]
    err: Option<Arc<dyn std::error::Error + Send + Sync>>,
    #[serde(skip)]
    detail: Option<String>,
}

impl AppError {
    pub fn new(status: StatusCode, err_no: i64, err_msg: impl Into<String>) -> Self {
        AppError {
            status,
            err_no,
            err_msg: err_msg.into(),
            err: None,
            detail: None,
        }
    }
    pub fn with_cause(
        &self,
        err: impl std::error::Error + Send + Sync + 'static,
        detail: impl Into<String>,
    ) -> Self {
        AppError {
            status: self.status,
            err_no: self.err_no,
            err_msg: self.err_msg.clone(),
            err: Some(Arc::new(err)),
            detail: Some(detail.into()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Cause and detail provide server-side diagnostics and are never sent to clients.
        if self.err.is_some() || self.detail.is_some() {
            error!(
                status = %self.status,
                err_no = self.err_no,
                detail = ?self.detail,
                cause = ?self.err,
                "app error"
            );
        }
        // Reuse the unified response structure; internal cause/detail are not part of the client protocol.
        let body = InnerAppResult::<()> {
            err_no: self.err_no,
            err_msg: self.err_msg,
            data: None,
        };

        (self.status, Json(body)).into_response()
    }
}

#[derive(Serialize)]
struct InnerAppResult<T: Serialize> {
    err_no: i64,
    err_msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> IntoResponse for InnerAppResult<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub struct AppResult<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for AppResult<T> {
    fn into_response(self) -> axum::response::Response {
        let res = InnerAppResult {
            err_no: 10000,
            err_msg: "success".to_string(),
            data: Some(self.0),
        };
        res.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::{AppError, AppResult, InnerAppResult};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_app_error() {
        let res = AppError::new(StatusCode::OK, 1, "error");
        assert_eq!(res.err_msg.as_str(), "error");
        assert_eq!(res.err_no, 1);
        assert_eq!(res.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_inner_app_result() {
        let res = InnerAppResult::<u8> {
            err_msg: "success".to_string(),
            err_no: 1,
            data: None,
        };
        assert_eq!(res.data, None);
        assert_eq!(res.err_no, 1);
        assert_eq!(res.err_msg, "success");
    }

    #[tokio::test]
    async fn test_app_result() {
        let r = AppResult(0);
        assert_eq!(r.0, 0);
    }

    #[test]
    fn test_with_cause() {
        let base = AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50200, "Server Internal Error");
        let err = base.with_cause(sqlx::Error::PoolClosed, "query user by id");

        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.err_no, 50200);
        assert_eq!(err.err_msg, "Server Internal Error");
        assert_eq!(err.detail.as_deref(), Some("query user by id"));
        assert_eq!(
            err.err.as_ref().unwrap().to_string(),
            "attempted to acquire a connection on a closed pool"
        );

        // Predefined `errors::ErrDbXxx` values are static and must not be moved.
        let cloned = crate::errors::ErrDbIo.with_cause(sqlx::Error::PoolClosed, "ping");
        assert_eq!(cloned.status, crate::errors::ErrDbIo.status);
        assert_eq!(cloned.err_no, crate::errors::ErrDbIo.err_no);
        assert!(cloned.err.is_some());
    }

    #[test]
    fn test_database_conversion_keeps_io_cause() {
        let err = crate::data::db_error::covert_error(sqlx::Error::Io(std::io::Error::other(
            "connection reset",
        )));

        assert_eq!(err.err_no, 50210);
        assert!(err.detail.as_deref().unwrap().contains("database I/O"));
        assert!(
            err.err
                .as_ref()
                .unwrap()
                .to_string()
                .contains("connection reset")
        );
        assert!(err.detail.as_deref().unwrap().contains("connection reset"));
    }

    #[test]
    fn test_database_conversion_keeps_protocol_cause() {
        let err = crate::data::db_error::covert_error(sqlx::Error::Protocol(
            "unexpected packet".to_string(),
        ));

        assert_eq!(err.err_no, 50212);
        assert!(err.detail.as_deref().unwrap().contains("database protocol"));
        assert!(
            err.err
                .as_ref()
                .unwrap()
                .to_string()
                .contains("unexpected packet")
        );
        assert!(err.detail.as_deref().unwrap().contains("unexpected packet"));
    }

    #[test]
    fn test_database_conversion_keeps_column_context_and_cause() {
        let err = crate::data::db_error::covert_error(sqlx::Error::ColumnNotFound(
            "created_at".to_string(),
        ));

        assert_eq!(err.err_no, 50216);
        assert_eq!(err.detail.as_deref(), Some("database column created_at lookup"));
        assert!(err.err.as_ref().unwrap().to_string().contains("created_at"));
    }

    #[test]
    fn test_error_with_cause_is_logged() {
        let logs = capture_logs(|| {
            let _ =
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, 50200, "Server Internal Error")
                    .with_cause(sqlx::Error::PoolClosed, "query user by id")
                    .into_response();
        });

        assert!(logs.contains("app error"), "cause should be logged: {logs}");
        assert!(logs.contains("query user by id"), "detail missing: {logs}");
        assert!(logs.contains("50200"), "err_no missing: {logs}");
    }

    #[test]
    fn test_error_without_cause_is_not_logged() {
        let logs = capture_logs(|| {
            let _ = crate::errors::ErrDbRowNotFound.clone().into_response();
        });

        assert!(logs.is_empty(), "plain errors must stay silent: {logs}");
    }

    #[tokio::test]
    async fn test_error_response_shape() {
        let res =
            AppError::new(StatusCode::BAD_REQUEST, 14000, "Bad Request Params").into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // The error response body does not contain a data field.
        assert_eq!(body, serde_json::json!({ "err_no": 14000, "err_msg": "Bad Request Params" }));
    }

    #[tokio::test]
    async fn test_error_response_does_not_expose_cause_or_detail() {
        let res = crate::errors::ErrDbGeneric
            .with_cause(
                sqlx::Error::Configuration("password=secret host=db.internal".into()),
                "execute query against users table",
            )
            .into_response();

        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            serde_json::json!({
                "err_no": 50208,
                "err_msg": "Server Internal Error"
            })
        );
        let body_text = body.to_string();
        assert!(!body_text.contains("password=secret"));
        assert!(!body_text.contains("db.internal"));
        assert!(!body_text.contains("users table"));
    }

    fn capture_logs(f: impl FnOnce()) -> String {
        let buffer = Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let writer = buffer.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(move || LogWriter(writer.clone()))
            .finish();

        tracing::subscriber::with_default(subscriber, f);

        String::from_utf8(buffer.lock().unwrap().clone()).unwrap()
    }

    struct LogWriter(Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for LogWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
