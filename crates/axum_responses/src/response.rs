use crate::{FileResponse, JsonResponse};
use axum::response::IntoResponse;

pub enum Response {
    #[doc(hidden)]
    Json(JsonResponse),

    #[doc(hidden)]
    File(FileResponse),

    #[doc(hidden)]
    Sse,

    #[doc(hidden)]
    Stream,

    #[doc(hidden)]
    Redirect(String),
}

impl Response {
    pub fn file() -> FileResponse {
        FileResponse::builder()
    }

    pub fn json() -> JsonResponse {
        JsonResponse::builder(200)
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Json(json_response) => json_response.into_response(),
            Response::File(file_response) => file_response.into_response(),
        }
    }
}
