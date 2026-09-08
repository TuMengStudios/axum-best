pub mod rest;
pub mod state;

use crate::core::rest::AppError;
use crate::core::rest::AppResult;

pub type Result<T> = core::result::Result<AppResult<T>, AppError>;

/// ok!(a) equal Ok(AppResult(a))
#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        Ok($crate::core::rest::AppResult($expr))
    };
}
