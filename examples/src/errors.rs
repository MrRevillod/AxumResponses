use axum_responses::{HttpError, thiserror::Error};
use serde::Serialize;

// Root error that composes other errors
// - thiserror provides: Display, Error, #[from]
// - HttpError provides: From<Self> for JsonResponse -> IntoResponse
#[derive(Debug, Error, HttpError)]
pub enum AppError {
    // transparent: uses the inner HttpError From<T> for JsonResponse
    #[error("Validation error: {0}")]
    #[http(transparent)]
    Validation(#[from] ValidationError),

    // code: fixed response, hides internal error details
    #[error("I/O error: {0}")]
    #[http(code = 500, message = "An internal error occurred")]
    Io(#[from] std::io::Error),

    // Unit variant with fixed response
    #[error("Rate limit exceeded")]
    #[http(code = 429, message = "Too many requests, please slow down")]
    RateLimited,
}

#[derive(Debug, Error, HttpError)]
pub enum ValidationError {
    #[error("Validation failed")]
    #[tracing(error)]
    #[http(code = 400, errors = details)]
    InvalidInput { details: Vec<FieldError> },

    #[error("Field is required")]
    #[http(code = 400, message = "{field} is required")]
    MissingField { field: String },

    #[error("Custom error")]
    #[tracing(error)]
    #[http(code = 400, message = error_msg)]
    CustomMessage { error_msg: String },
}

impl ValidationError {
    pub fn invalid_input(errors: Vec<FieldError>) -> Self {
        ValidationError::InvalidInput { details: errors }
    }

    pub fn missing_field(field: String) -> Self {
        ValidationError::MissingField { field }
    }

    pub fn custom_message(msg: String) -> Self {
        ValidationError::CustomMessage { error_msg: msg }
    }
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
