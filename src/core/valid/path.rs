use axum::extract::FromRequestParts;
use axum::extract::Path;
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error::bad_request;
use super::validate;
use crate::core::rest::AppError;

/// Path-parameters extractor that validates the deserialized payload.
pub struct ValidPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = Path::<T>::from_request_parts(parts, state).await;
        let Path(value) = extracted.map_err(|e| bad_request(e.body_text()))?;
        let value = validate(value)?;
        Ok(ValidPath(value))
    }
}

#[cfg(test)]
mod tests {
    use super::ValidPath;
    use crate::core::valid::error::ERR_NO_BAD_REQUEST;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::routing::get;
    use serde::Deserialize;
    use tower::ServiceExt;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    struct IdRequest {
        #[validate(range(min = 1))]
        id: i64,
    }

    #[tokio::test]
    async fn test_valid_path_rejects_invalid_payload() {
        async fn handler(ValidPath(_req): ValidPath<IdRequest>) {}

        let app = Router::new().route("/{id}", get(handler));
        let response = app
            .oneshot(Request::builder().uri("/0").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["err_no"], ERR_NO_BAD_REQUEST);
        assert!(body["err_msg"].as_str().unwrap().contains("id"));
    }
}
