use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use derivative::Derivative;
use serde::Serialize;

#[allow(unused)]
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct AppError {
    #[derivative(Debug = "ignore")]
    status: StatusCode,
    err_no: i64,
    err_msg: String,
}

impl AppError {
    pub fn new(status: StatusCode, err_no: i64, err_msg: &str) -> Self {
        let res = AppError {
            status,
            err_no,
            err_msg: err_msg.to_string(),
        };
        res
    }
}

#[tokio::test]
async fn test_app_error() {
    let res = AppError::new(StatusCode::OK, 1, "error");
    assert_eq!(res.err_msg.as_str(), "error");
    assert_eq!(res.err_no, 1);
    assert_eq!(res.status, StatusCode::OK);
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let res = InnerAppResult::<u8> {
            err_msg: self.err_msg.clone(),
            err_no: self.err_no,
            data: None,
        };

        (self.status, Json(res)).into_response()
    }
}

#[derive(Serialize)]
struct InnerAppResult<T: Serialize> {
    err_no: i64,
    err_msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
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

#[tokio::test]
async fn test_app_result() {
    //
    let r = AppResult(0);
    assert_eq!(r.0, 0);
}
