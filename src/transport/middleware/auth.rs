//! JWT bearer authentication middleware.

use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::auth::Claims;
use crate::core::rest::AppError;
use crate::core::state::AppState;
use crate::errors::ErrUnauthorized;

/// Extracts and validates an `Authorization: Bearer <jwt>` header.
///
/// Valid claims are inserted into request extensions so handlers can extract
/// [`Claims`] directly as a handler parameter.
///
/// The full [`AppState`] is injected so this middleware can perform additional
/// user checks (for example, disabled or deleted accounts) after JWT validation.
pub async fn auth(State(state): State<AppState>, mut request: Request, next: Next) -> Response {
    let claims = match decode_request_claims(&state, request.headers()) {
        Ok(claims) => claims,
        Err(error) => return unauthorized_response(error),
    };

    match state.user_service.active_user(claims.user_id).await {
        Ok(_) => {}
        Err(error) => return error.into_response(),
    }

    request.extensions_mut().insert(claims);
    next.run(request).await
}

fn decode_request_claims(state: &AppState, headers: &HeaderMap) -> Result<Claims, AppError> {
    let token = bearer_token(headers.get(axum::http::header::AUTHORIZATION))
        .ok_or_else(|| ErrUnauthorized.clone())?;
    state.cfg.jwt.decode_token(token)
}

fn bearer_token(value: Option<&HeaderValue>) -> Option<&str> {
    let value = value?.to_str().ok()?;
    let (scheme, token) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() || token.contains(' ') {
        return None;
    }
    Some(token)
}

fn unauthorized_response(error: AppError) -> Response {
    let mut response = error.into_response();
    response
        .headers_mut()
        .insert(axum::http::header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
    response
}

#[cfg(test)]
mod tests {}
