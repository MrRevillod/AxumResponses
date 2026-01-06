use crate::JsonResponse;
use axum::extract::multipart::MultipartError;
use axum::extract::multipart::MultipartRejection;

impl From<MultipartRejection> for JsonResponse {
    fn from(err: MultipartRejection) -> Self {
        tracing::error!("MultipartRejection: {err:?}");
        JsonResponse::status(err.status())
    }
}

impl From<MultipartError> for JsonResponse {
    fn from(err: MultipartError) -> Self {
        tracing::error!("MultipartError: {err:?}");
        JsonResponse::status(err.status())
    }
}
