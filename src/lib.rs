mod tests;

use axum::{
    http::StatusCode, response::IntoResponse, Json,
};

use serde_json::{json, Value};

pub type ControllerResult = Result<HttpResponse, HttpResponse>;
pub type HandlerResult = Result<HttpResponse, HttpResponse>;

pub struct HttpResponse {
    pub status: StatusCode,
    pub body: Value,
}

impl HttpResponse {
    pub fn new(status: StatusCode, body: Value) -> Self {
        Self { status, body }
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self.body)).into_response()
    }
}

#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ "status": status.to_string() });

        match status.as_u16() {
            200..=299 => Ok($crate::HttpResponse::new(status, json)),
            _ => Err($crate::HttpResponse::new(status, json))
        }
    }};

    ($status:expr, { $value:ident }) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::to_value(&$value).unwrap_or_else(|_| {
            panic!("Failed to serialize value to JSON");
        });

        match status.as_u16() {
            200..=399 => Ok($crate::HttpResponse { status, body: json }),
            _ => Err($crate::HttpResponse { status, body: json })
        }
    }};

    ($status:expr, { $($json:tt)* }) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ $($json)* });

        match status.as_u16() {
            200..=399 => Ok($crate::HttpResponse { status, body: json }),
            _ => Err($crate::HttpResponse { status, body: json })
        }
    }};
}

#[allow(non_camel_case_types)]
pub enum Response {
    CONTINUE,
    SWITCHING_PROTOCOLS,
    OK,
    CREATED,
    ACCEPTED,
    NON_AUTHORITATIVE_INFORMATION,
    NO_CONTENT,
    RESET_CONTENT,
    PARTIAL_CONTENT,
    MULTIPLE_CHOICES,
    MOVED_PERMANENTLY,
    FOUND,
    SEE_OTHER,
    NOT_MODIFIED,
    USE_PROXY,
    TEMPORARY_REDIRECT,
    BAD_REQUEST,
    UNAUTHORIZED,
    PAYMENT_REQUIRED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED,
    REQUEST_TIMEOUT,
    CONFLICT,
    GONE,
    LENGTH_REQUIRED,
    PRECONDITION_FAILED,
    UNSUPPORTED_MEDIA_TYPE,
    EXPECTATION_FAILED,
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    CUSTOM(u16, Value),
}

impl IntoResponse for Response {
    fn into_response(self: Self) -> axum::response::Response {
        let (status, body) = match self {
            Response::CUSTOM(code, value) => {
                (StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(value))
            }
            _ => {
                let (status, message) = match self {
                    Response::CONTINUE => (StatusCode::CONTINUE, "Continue"),
                    Response::SWITCHING_PROTOCOLS => (StatusCode::SWITCHING_PROTOCOLS, "Switching Protocols"),
                    Response::OK => (StatusCode::OK, "OK"),
                    Response::CREATED => (StatusCode::CREATED, "Created"),
                    Response::ACCEPTED => (StatusCode::ACCEPTED, "Accepted"),
                    Response::NON_AUTHORITATIVE_INFORMATION => (StatusCode::NON_AUTHORITATIVE_INFORMATION, "Non Authoritative Information"),
                    Response::NO_CONTENT => (StatusCode::NO_CONTENT, "No Content"),
                    Response::RESET_CONTENT => (StatusCode::RESET_CONTENT, "Reset Content"),
                    Response::PARTIAL_CONTENT => (StatusCode::PARTIAL_CONTENT, "Partial Content"),
                    Response::MULTIPLE_CHOICES => (StatusCode::MULTIPLE_CHOICES, "Multiple Choices"),
                    Response::MOVED_PERMANENTLY => (StatusCode::MOVED_PERMANENTLY, "Moved Permanently"),
                    Response::FOUND => (StatusCode::FOUND, "Found"),
                    Response::SEE_OTHER => (StatusCode::SEE_OTHER, "See Other"),
                    Response::NOT_MODIFIED => (StatusCode::NOT_MODIFIED, "Not Modified"),
                    Response::USE_PROXY => (StatusCode::USE_PROXY, "Use Proxy"),
                    Response::TEMPORARY_REDIRECT => (StatusCode::TEMPORARY_REDIRECT, "Temporary Redirect"),
                    Response::BAD_REQUEST => (StatusCode::BAD_REQUEST, "Bad Request"),
                    Response::UNAUTHORIZED => (StatusCode::UNAUTHORIZED, "Unauthorized"),
                    Response::PAYMENT_REQUIRED => (StatusCode::PAYMENT_REQUIRED, "Payment Required"),
                    Response::FORBIDDEN => (StatusCode::FORBIDDEN, "Forbidden"),
                    Response::NOT_FOUND => (StatusCode::NOT_FOUND, "Not Found"),
                    Response::METHOD_NOT_ALLOWED => (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed"),
                    Response::NOT_ACCEPTABLE => (StatusCode::NOT_ACCEPTABLE, "Not Acceptable"),
                    Response::PROXY_AUTHENTICATION_REQUIRED => (StatusCode::PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required"),
                    Response::REQUEST_TIMEOUT => (StatusCode::REQUEST_TIMEOUT, "Request Timeout"),
                    Response::CONFLICT => (StatusCode::CONFLICT, "Conflict"),
                    Response::GONE => (StatusCode::GONE, "Gone"),
                    Response::LENGTH_REQUIRED => (StatusCode::LENGTH_REQUIRED, "Length Required"),
                    Response::PRECONDITION_FAILED => (StatusCode::PRECONDITION_FAILED, "Precondition Failed"),
                    Response::UNSUPPORTED_MEDIA_TYPE => (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type"),
                    Response::EXPECTATION_FAILED => (StatusCode::EXPECTATION_FAILED, "Expectation Failed"),
                    Response::INTERNAL_SERVER_ERROR => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
                    Response::NOT_IMPLEMENTED => (StatusCode::NOT_IMPLEMENTED, "Not Implemented"),
                    Response::BAD_GATEWAY => (StatusCode::BAD_GATEWAY, "Bad Gateway"),
                    Response::SERVICE_UNAVAILABLE => (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable"),
                    Response::GATEWAY_TIMEOUT => (StatusCode::GATEWAY_TIMEOUT, "Gateway Timeout"),
                    Response::HTTP_VERSION_NOT_SUPPORTED => (StatusCode::HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported"),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown Error"),
                };
                (status, Json(json!({ "message": message })))
            }
        };

        (status, body).into_response()
    }
}
