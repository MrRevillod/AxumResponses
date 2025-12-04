use std::path::Path;

use crate::{AppResult, errors::*};
use axum::response::IntoResponse;
use axum_responses::{FileResponse, HttpResult, JsonResponse};

// async fn service() -> AppResult<()> {
//     Err(SimpleError::InvalidRequest(vec![]))?
// }

// async fn service_2() -> AppResult<()> {
//     Err(AppError::IoError(std::io::Error::new(
//         std::io::ErrorKind::Other,
//         "simulated I/O error",
//     )))?
// }
