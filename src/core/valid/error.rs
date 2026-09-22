use axum::http::StatusCode;
use tracing::warn;
use validator::ValidationError;
use validator::ValidationErrors;

use crate::core::rest::AppError;

pub(super) const ERR_NO_BAD_REQUEST: i64 = 14000;

pub(super) fn bad_request(message: impl std::fmt::Display) -> AppError {
    warn!(
        status = %StatusCode::BAD_REQUEST,
        err_no = ERR_NO_BAD_REQUEST,
        detail = %message,
        "bad request params"
    );
    AppError::new(
        StatusCode::BAD_REQUEST,
        ERR_NO_BAD_REQUEST,
        format!("Bad Request Params: {message}"),
    )
}

pub(super) fn validation_rejection(errors: &ValidationErrors) -> AppError {
    bad_request(format_validation_errors(errors))
}

fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();
    for (field, errs) in errors.field_errors() {
        for err in errs {
            messages.push(format_field_error(&field, err));
        }
    }
    messages.join("; ")
}

fn format_field_error(field: &str, err: &ValidationError) -> String {
    let base = err.message.as_deref().unwrap_or(err.code.as_ref());

    let mut params = Vec::new();
    for (name, value) in &err.params {
        if name.as_ref() != "__type__" {
            params.push(format!("{name}={value}"));
        }
    }

    if params.is_empty() {
        format!("{field}: {base}")
    } else {
        format!("{field}: {base} ({})", params.join(", "))
    }
}
