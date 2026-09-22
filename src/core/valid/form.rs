use axum::Form;
use axum::extract::FromRequest;
use axum::extract::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

use super::error::bad_request;
use super::validate;
use crate::core::rest::AppError;

/// Form extractor that validates the deserialized payload.
pub struct ValidForm<T>(pub T);

impl<S, T> FromRequest<S> for ValidForm<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let extracted = Form::<T>::from_request(req, state).await;
        let Form(value) = extracted.map_err(|e| bad_request(e.body_text()))?;
        let value = validate(value)?;
        Ok(ValidForm(value))
    }
}
