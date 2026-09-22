//! Validation extractors that reject with the unified `AppError` format:
//! `{"err_no": 14000, "err_msg": "Bad Request Params: ..."}`.

mod error;
mod form;
mod json;
mod path;
mod query;

use validator::Validate;

use crate::core::rest::AppError;

pub use form::ValidForm;
pub use json::ValidJson;
pub use path::ValidPath;
pub use query::ValidQuery;

fn validate<T: Validate>(value: T) -> Result<T, AppError> {
    value
        .validate()
        .map_err(|errors| error::validation_rejection(&errors))?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::error::ERR_NO_BAD_REQUEST;
    use super::validate;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    struct Paginator {
        #[validate(range(min = 1, max = 50))]
        page_size: usize,
        #[validate(range(min = 1))]
        page_no: usize,
    }

    #[tokio::test]
    async fn test_validate_rejection_uses_unified_body() {
        let value = Paginator {
            page_size: 0,
            page_no: 0,
        };
        let response = validate(value).unwrap_err().into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["err_no"], ERR_NO_BAD_REQUEST);

        let msg = body["err_msg"].as_str().unwrap();
        assert!(msg.contains("page_size"), "message: {msg}");
        assert!(msg.contains("page_no"), "message: {msg}");
    }
}
