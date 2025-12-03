use axum_responses::{FileResult, HttpError, HttpResult, JsonResponse};

#[derive(Debug, HttpError)]
enum SimpleError {
    #[code(400)]
    #[message("Bad Request")]
    #[error("Invalid request format")]
    InvalidRequest,

    #[code(404)]
    #[message("Not Found")]
    NotFound,
}

fn a() -> HttpResult {
    Ok(JsonResponse::Ok())
}
