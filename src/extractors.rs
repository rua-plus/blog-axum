use std::{fmt, ops::Deref};

use axum::{
    extract::{FromRequest, FromRequestParts, Json},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use validator::Validate;

use crate::{
    response::{ErrorDetail, StatusCode as AppStatusCode},
    utils::jwt::{Claims, JwtError, JwtService},
};

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: Validate + Send,
    Json<T>: FromRequest<S>,
{
    type Rejection = axum::response::Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let json = match Json::<T>::from_request(req, state).await {
            Ok(json) => json,
            Err(_rejection) => {
                let error_response = AppStatusCode::bad_request()
                    .with_debug("Invalid JSON request body")
                    .into_response();
                return Err(error_response);
            }
        };

        let value = json.0;

        match value.validate() {
            Ok(_) => Ok(ValidatedJson(value)),
            Err(errors) => {
                let error = errors.field_errors();
                let error_details: Vec<ErrorDetail> = error
                    .iter()
                    .flat_map(|(field, error_list)| {
                        error_list.iter().map(move |err| ErrorDetail {
                            field: Some(field.to_string()),
                            message: err
                                .message
                                .as_ref()
                                .unwrap_or(&"Validation error".into())
                                .to_string(),
                        })
                    })
                    .collect();

                let error_response = AppStatusCode::validation_error()
                    .with_errors(error_details)
                    .into_response();
                Err(error_response)
            }
        }
    }
}

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Authentication extractor that validates JWT tokens
#[derive(Debug, Clone)]
pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Deref<Target = JwtService> + Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract token from Authorization header
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AuthError::MissingAuthHeader)?;

        let auth_header = auth_header
            .to_str()
            .map_err(|_| AuthError::InvalidAuthHeader)?;

        // Bearer token format: "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidTokenFormat)?;

        // Validate token
        let claims = state.validate_token(token).map_err(AuthError::Jwt)?;

        Ok(Auth(claims))
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingAuthHeader,
    InvalidAuthHeader,
    InvalidTokenFormat,
    Jwt(JwtError),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::MissingAuthHeader => write!(f, "Missing Authorization header"),
            AuthError::InvalidAuthHeader => write!(f, "Invalid Authorization header"),
            AuthError::InvalidTokenFormat => {
                write!(f, "Invalid token format. Use 'Bearer <token>'")
            }
            AuthError::Jwt(e) => write!(f, "Authentication failed: {}", e),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::MissingAuthHeader
            | AuthError::InvalidAuthHeader
            | AuthError::InvalidTokenFormat => StatusCode::UNAUTHORIZED,
            AuthError::Jwt(JwtError::InvalidToken) | AuthError::Jwt(JwtError::ExpiredToken) => {
                StatusCode::UNAUTHORIZED
            }
            AuthError::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

impl std::error::Error for AuthError {}
