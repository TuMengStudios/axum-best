use axum::Json;
use axum::extract::FromRequest;
use axum::extract::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error::bad_request;
use super::validate;
use crate::core::rest::AppError;

/// JSON extractor that validates the deserialized payload.
pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = Json::<T>::from_request(req, state).await;
        let Json(value) = extracted.map_err(|e| bad_request(e.body_text()))?;
        let value = validate(value)?;
        Ok(ValidJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::ValidJson;
    use crate::core::valid::error::ERR_NO_BAD_REQUEST;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::routing::post;
    use serde::Deserialize;
    use tower::ServiceExt;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    struct Paginator {
        #[validate(range(min = 1, max = 50))]
        page_size: usize,
        #[validate(range(min = 1))]
        page_no: usize,
    }

    #[tokio::test]
    async fn test_valid_json_rejects_invalid_payload() {
        async fn handler(ValidJson(_req): ValidJson<Paginator>) {}

        let app = Router::new().route("/", post(handler));
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"page_size":0,"page_no":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["err_no"], ERR_NO_BAD_REQUEST);
        assert!(body["err_msg"].as_str().unwrap().contains("page_size"));
    }

    #[tokio::test]
    async fn test_valid_json_accepts_valid_payload() {
        async fn handler(ValidJson(req): ValidJson<Paginator>) -> String {
            req.page_size.to_string()
        }

        let app = Router::new().route("/", post(handler));
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"page_size":10,"page_no":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
