use axum::response::IntoResponse;
use axum_responses::HttpError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Simple(#[from] SimpleError),

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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            any => any.into_response(),
        }
    }
}
