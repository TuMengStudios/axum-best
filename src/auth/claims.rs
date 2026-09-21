use axum::extract::FromRequestParts;
use axum::http::HeaderValue;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::errors::ErrUnauthorized;

/// Claims carried by authenticated requests.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub user_id: i64,
    pub exp: i64,
}

/// Request extractor for the authenticated user's JWT claims.
///
/// The auth middleware validates the header and inserts claims into request
/// extensions. Handlers can then declare `Claims` directly as a parameter.
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(claims) = parts.extensions.get::<Claims>() {
            return Ok(claims.clone());
        }

        let mut response = ErrUnauthorized.clone().into_response();
        response
            .headers_mut()
            .insert(axum::http::header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        Err(response)
    }
}
