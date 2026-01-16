use anyhow::Error;
use axum::response::{IntoResponse, Response};
use tracing::error;

use crate::response::StatusCode;

#[derive(Debug)]
pub struct AppError(Error);

impl AppError {
    pub fn new<E: Into<Error>>(err: E) -> Self {
        AppError(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{:?}", self);
        StatusCode::internal_error()
            .with_debug(self.0.to_string())
            .into_response()
    }
}

pub type AppResult<T> = anyhow::Result<T, AppError>;

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError(err)
    }
}

// Common error conversions
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::new(err)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::new(err)
    }
}

impl From<crate::utils::jwt::JwtError> for AppError {
    fn from(err: crate::utils::jwt::JwtError) -> Self {
        AppError::new(err)
    }
}
