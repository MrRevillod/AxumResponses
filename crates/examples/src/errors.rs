use axum_responses::{HttpError, JsonResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Simple(#[from] SimpleError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, HttpError)]
pub enum SimpleError {
    #[code(400)]
    #[message("Bad Request")]
    #[errors(details)]
    InvalidRequest { details: Vec<Detail> },
}

#[derive(Debug, Serialize)]
pub struct Detail {
    pub field: String,
    pub issue: String,
}

impl From<AppError> for JsonResponse {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DatabaseError(db_err) => {
                eprintln!("Database error occurred: {}", db_err);
                JsonResponse::builder(500).message("Internal Server Error")
            }
            other => JsonResponse::from(other),
        }
    }
}
