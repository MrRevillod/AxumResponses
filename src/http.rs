use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// ## HttpResponse
/// Represents a structured HTTP response
/// that can be used in Axum applications.
///
/// It implements `IntoResponse` to convert
/// the response into an Axum-compatible response.
///
/// The IntoResponse returns a HttpResponse follows two conventions:
///
/// 1. `RFC 7231` - HTTP/1.1 Semantics and Content
/// 2. `RFC 8259` - The JavaScript Object Notation (JSON) Data Interchange Format
///
/// The headers are stored in a `HeaderMap`
/// but they are not serialized into the final JSON body.
///
/// ### Http Code Variants
/// The struct provides methods for common HTTP status codes for example:
/// - `HttpResponse::Ok()` for 200 OK
/// - `HttpResponse::NotFound()` for 404 Not Found
///
/// These methods create a new `HttpResponse`
/// with the appropriate status code and a default message.
#[derive(Debug)]
pub struct HttpResponse {
    data: Option<Value>,
    code: StatusCode,
    success: bool,
    message: String,
    timestamp: String,
    headers: HeaderMap,
}

impl HttpResponse {
    pub fn builder(code: StatusCode) -> Self {
        Self {
            code,
            data: None,
            success: code.is_success(),
            message: String::new(),
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            headers: HeaderMap::new(),
        }
    }

    /// Sets the response message.
    /// The `message` parameter should be convertible to a `String`.
    /// This message is typically a human-readable description of the response.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Adds a header to the response.
    /// The `key` and `value` parameters should be convertible to `HeaderName` and `HeaderValue`, respectively.
    /// If the conversion fails, the header is not added.
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(header_name), Ok(header_value)) =
            (HeaderName::try_from(key), HeaderValue::try_from(value))
        {
            self.headers.insert(header_name, header_value);
        }

        self
    }

    /// Adds data to the response.
    /// The `data` parameter should implement `Serialize`.
    /// If serialization fails, it logs a warning and sets `data` to an error message.
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        let data = serde_json::to_value(data).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response data: {err}");
            json!({ "error": "Serialization failed" })
        });

        self.data = Some(data);
        self
    }
}

/// Represents the body of the HTTP response.
/// Can be used to tests to verify the structure of the response.
/// This is the structure of the JSON response body that will
/// be returned by the `HttpResponse`.
///
/// The data field is optional and will be included
/// only if is setted in the `HttpResponse` builder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody {
    pub code: u16,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> axum::response::Response {
        let mut body = json!({
            "code": self.code.as_u16(),
            "success": self.success,
            "message": self.message,
            "timestamp": self.timestamp,
        });

        if let Some(content) = self.data {
            body["data"] = content;
        } 

        let mut response = (self.code, Json(body)).into_response();

        for (key, value) in self.headers.iter() {
            response.headers_mut().insert(key, value.clone());
        }

        response
    }
}

impl HttpResponse {
    pub fn Continue() -> Self {
        Self::builder(StatusCode::CONTINUE).message("Continue")
    }

    pub fn SwitchingProtocols() -> Self {
        Self::builder(StatusCode::SWITCHING_PROTOCOLS).message("Switching Protocols")
    }

    pub fn Processing() -> Self {
        Self::builder(StatusCode::PROCESSING).message("Processing")
    }

    pub fn Ok() -> Self {
        Self::builder(StatusCode::OK).message("OK")
    }

    pub fn Created() -> Self {
        Self::builder(StatusCode::CREATED).message("Created")
    }

    pub fn Accepted() -> Self {
        Self::builder(StatusCode::ACCEPTED).message("Accepted")
    }

    pub fn NonAuthoritativeInformation() -> Self {
        Self::builder(StatusCode::NON_AUTHORITATIVE_INFORMATION)
            .message("Non-Authoritative Information")
    }

    pub fn NoContent() -> Self {
        Self::builder(StatusCode::NO_CONTENT).message("No Content")
    }

    pub fn ResetContent() -> Self {
        Self::builder(StatusCode::RESET_CONTENT).message("Reset Content")
    }

    pub fn PartialContent() -> Self {
        Self::builder(StatusCode::PARTIAL_CONTENT).message("Partial Content")
    }

    pub fn MultiStatus() -> Self {
        Self::builder(StatusCode::MULTI_STATUS).message("Multi-Status")
    }

    pub fn AlreadyReported() -> Self {
        Self::builder(StatusCode::ALREADY_REPORTED).message("Already Reported")
    }

    pub fn ImUsed() -> Self {
        Self::builder(StatusCode::IM_USED).message("IM Used")
    }

    pub fn MultipleChoices() -> Self {
        Self::builder(StatusCode::MULTIPLE_CHOICES).message("Multiple Choices")
    }

    pub fn MovedPermanently() -> Self {
        Self::builder(StatusCode::MOVED_PERMANENTLY).message("Moved Permanently")
    }

    pub fn Found() -> Self {
        Self::builder(StatusCode::FOUND).message("Found")
    }

    pub fn SeeOther() -> Self {
        Self::builder(StatusCode::SEE_OTHER).message("See Other")
    }

    pub fn NotModified() -> Self {
        Self::builder(StatusCode::NOT_MODIFIED).message("Not Modified")
    }

    pub fn TemporaryRedirect() -> Self {
        Self::builder(StatusCode::TEMPORARY_REDIRECT).message("Temporary Redirect")
    }

    pub fn PermanentRedirect() -> Self {
        Self::builder(StatusCode::PERMANENT_REDIRECT).message("Permanent Redirect")
    }

    pub fn BadRequest() -> Self {
        Self::builder(StatusCode::BAD_REQUEST).message("Bad Request")
    }

    pub fn Unauthorized() -> Self {
        Self::builder(StatusCode::UNAUTHORIZED).message("Unauthorized")
    }

    pub fn PaymentRequired() -> Self {
        Self::builder(StatusCode::PAYMENT_REQUIRED).message("Payment Required")
    }

    pub fn Forbidden() -> Self {
        Self::builder(StatusCode::FORBIDDEN).message("Forbidden")
    }

    pub fn NotFound() -> Self {
        Self::builder(StatusCode::NOT_FOUND).message("Not Found")
    }

    pub fn MethodNotAllowed() -> Self {
        Self::builder(StatusCode::METHOD_NOT_ALLOWED).message("Method Not Allowed")
    }

    pub fn NotAcceptable() -> Self {
        Self::builder(StatusCode::NOT_ACCEPTABLE).message("Not Acceptable")
    }

    pub fn ProxyAuthenticationRequired() -> Self {
        Self::builder(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
            .message("Proxy Authentication Required")
    }

    pub fn RequestTimeout() -> Self {
        Self::builder(StatusCode::REQUEST_TIMEOUT).message("Request Timeout")
    }

    pub fn Conflict() -> Self {
        Self::builder(StatusCode::CONFLICT).message("Conflict")
    }

    pub fn Gone() -> Self {
        Self::builder(StatusCode::GONE).message("Gone")
    }

    pub fn LengthRequired() -> Self {
        Self::builder(StatusCode::LENGTH_REQUIRED).message("Length Required")
    }

    pub fn PreconditionFailed() -> Self {
        Self::builder(StatusCode::PRECONDITION_FAILED).message("Precondition Failed")
    }

    pub fn PayloadTooLarge() -> Self {
        Self::builder(StatusCode::PAYLOAD_TOO_LARGE).message("Payload Too Large")
    }

    pub fn UriTooLong() -> Self {
        Self::builder(StatusCode::URI_TOO_LONG).message("URI Too Long")
    }

    pub fn UnsupportedMediaType() -> Self {
        Self::builder(StatusCode::UNSUPPORTED_MEDIA_TYPE).message("Unsupported Media Type")
    }

    pub fn RangeNotSatisfiable() -> Self {
        Self::builder(StatusCode::RANGE_NOT_SATISFIABLE).message("Range Not Satisfiable")
    }

    pub fn ExpectationFailed() -> Self {
        Self::builder(StatusCode::EXPECTATION_FAILED).message("Expectation Failed")
    }

    pub fn ImATeapot() -> Self {
        Self::builder(StatusCode::IM_A_TEAPOT).message("I'm a teapot")
    }

    pub fn UnprocessableEntity() -> Self {
        Self::builder(StatusCode::UNPROCESSABLE_ENTITY).message("Unprocessable Entity")
    }

    pub fn Locked() -> Self {
        Self::builder(StatusCode::LOCKED).message("Locked")
    }

    pub fn FailedDependency() -> Self {
        Self::builder(StatusCode::FAILED_DEPENDENCY).message("Failed Dependency")
    }

    pub fn TooEarly() -> Self {
        Self::builder(StatusCode::TOO_EARLY).message("Too Early")
    }

    pub fn UpgradeRequired() -> Self {
        Self::builder(StatusCode::UPGRADE_REQUIRED).message("Upgrade Required")
    }

    pub fn PreconditionRequired() -> Self {
        Self::builder(StatusCode::PRECONDITION_REQUIRED).message("Precondition Required")
    }

    pub fn TooManyRequests() -> Self {
        Self::builder(StatusCode::TOO_MANY_REQUESTS).message("Too Many Requests")
    }

    pub fn RequestHeaderFieldsTooLarge() -> Self {
        Self::builder(StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
            .message("Request Header Fields Too Large")
    }

    pub fn InternalServerError() -> Self {
        Self::builder(StatusCode::INTERNAL_SERVER_ERROR).message("Internal Server Error")
    }

    pub fn NotImplemented() -> Self {
        Self::builder(StatusCode::NOT_IMPLEMENTED).message("Not Implemented")
    }

    pub fn BadGateway() -> Self {
        Self::builder(StatusCode::BAD_GATEWAY).message("Bad Gateway")
    }

    pub fn ServiceUnavailable() -> Self {
        Self::builder(StatusCode::SERVICE_UNAVAILABLE).message("Service Unavailable")
    }

    pub fn GatewayTimeout() -> Self {
        Self::builder(StatusCode::GATEWAY_TIMEOUT).message("Gateway Timeout")
    }

    pub fn HttpVersionNotSupported() -> Self {
        Self::builder(StatusCode::HTTP_VERSION_NOT_SUPPORTED).message("HTTP Version Not Supported")
    }

    pub fn VariantAlsoNegotiates() -> Self {
        Self::builder(StatusCode::VARIANT_ALSO_NEGOTIATES).message("Variant Also Negotiates")
    }

    pub fn InsufficientStorage() -> Self {
        Self::builder(StatusCode::INSUFFICIENT_STORAGE).message("Insufficient Storage")
    }

    pub fn LoopDetected() -> Self {
        Self::builder(StatusCode::LOOP_DETECTED).message("Loop Detected")
    }

    pub fn NotExtended() -> Self {
        Self::builder(StatusCode::NOT_EXTENDED).message("Not Extended")
    }

    pub fn NetworkAuthenticationRequired() -> Self {
        Self::builder(StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
            .message("Network Authentication Required")
    }
}
