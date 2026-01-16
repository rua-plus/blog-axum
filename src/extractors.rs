use axum::{
    extract::{FromRequest, Json},
    response::IntoResponse,
};
use validator::Validate;

use crate::response::{ErrorDetail, StatusCode as AppStatusCode};

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
