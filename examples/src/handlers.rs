use crate::errors::*;
use axum_responses::JsonResponse;

pub type AppResult = Result<JsonResponse, AppError>;

pub async fn create_user() -> AppResult {
    let errors = vec![
        FieldError {
            field: "email".into(),
            message: "Invalid email format".into(),
        },
        FieldError {
            field: "password".into(),
            message: "Password must be at least 8 characters".into(),
        },
    ];

    // ValidationError -> AppError (#[from]) -> IntoResponse
    if !errors.is_empty() {
        return Err(ValidationError::invalid_input(errors))?;
    }

    Ok(JsonResponse::Created().message("User created"))
}

pub async fn missing_field() -> AppResult {
    // ValidationError -> AppError (via #[from]) -> IntoResponse
    Err(ValidationError::missing_field("username".into()))?
}

pub async fn rate_limited() -> AppResult {
    // Direct AppError variant
    Err(AppError::RateLimited)
}

pub async fn io_error() -> AppResult {
    // std::io::Error -> AppError (via #[from]) -> IntoResponse
    let _ = std::fs::read_to_string("/nonexistent/file")?;
    Ok(JsonResponse::Ok())
}
