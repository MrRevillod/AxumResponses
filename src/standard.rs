//! Standard HTTP responses with just status codes and default messages
//!
//! Use this module for quick, simple responses without custom data.
//! All responses return just the HTTP status code and a standard message.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

/// Standard HTTP responses enum for simple status + message responses
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Response {
    // 2xx Success responses
    OK,
    CREATED,
    ACCEPTED,
    NO_CONTENT,

    // 3xx Redirection responses
    NOT_MODIFIED,

    // 4xx Client Error responses
    BAD_REQUEST,
    UNAUTHORIZED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    CONFLICT,
    UNPROCESSABLE_ENTITY,
    TOO_MANY_REQUESTS,

    // 5xx Server Error responses
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,

    CUSTOM(u16, Value),
}

impl Response {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Response::OK => StatusCode::OK,
            Response::CREATED => StatusCode::CREATED,
            Response::ACCEPTED => StatusCode::ACCEPTED,
            Response::NO_CONTENT => StatusCode::NO_CONTENT,
            Response::NOT_MODIFIED => StatusCode::NOT_MODIFIED,
            Response::BAD_REQUEST => StatusCode::BAD_REQUEST,
            Response::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
            Response::FORBIDDEN => StatusCode::FORBIDDEN,
            Response::NOT_FOUND => StatusCode::NOT_FOUND,
            Response::METHOD_NOT_ALLOWED => StatusCode::METHOD_NOT_ALLOWED,
            Response::CONFLICT => StatusCode::CONFLICT,
            Response::UNPROCESSABLE_ENTITY => StatusCode::UNPROCESSABLE_ENTITY,
            Response::TOO_MANY_REQUESTS => StatusCode::TOO_MANY_REQUESTS,
            Response::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
            Response::NOT_IMPLEMENTED => StatusCode::NOT_IMPLEMENTED,
            Response::BAD_GATEWAY => StatusCode::BAD_GATEWAY,
            Response::SERVICE_UNAVAILABLE => StatusCode::SERVICE_UNAVAILABLE,
            Response::CUSTOM(status, _) => {
                StatusCode::from_u16(*status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Response::OK => "OK",
            Response::CREATED => "Created",
            Response::ACCEPTED => "Accepted",
            Response::NO_CONTENT => "No Content",
            Response::NOT_MODIFIED => "Not Modified",
            Response::BAD_REQUEST => "Bad Request",
            Response::UNAUTHORIZED => "Unauthorized",
            Response::FORBIDDEN => "Forbidden",
            Response::NOT_FOUND => "Not Found",
            Response::METHOD_NOT_ALLOWED => "Method Not Allowed",
            Response::CONFLICT => "Conflict",
            Response::UNPROCESSABLE_ENTITY => "Unprocessable Entity",
            Response::TOO_MANY_REQUESTS => "Too Many Requests",
            Response::INTERNAL_SERVER_ERROR => "Internal Server Error",
            Response::NOT_IMPLEMENTED => "Not Implemented",
            Response::BAD_GATEWAY => "Bad Gateway",
            Response::SERVICE_UNAVAILABLE => "Service Unavailable",
            Response::CUSTOM(_, _) => "",
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();

        match self {
            Response::NO_CONTENT | Response::NOT_MODIFIED => status.into_response(),
            Response::CUSTOM(_, body) => (status, Json(body)).into_response(),
            _ => {
                let body = json!({
                    "message": self.message()
                });
                (status, Json(body)).into_response()
            }
        }
    }
}
