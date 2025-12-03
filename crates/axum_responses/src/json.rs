use std::collections::HashMap;

use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse as AxumIntoResponse, Response as AxumResponse},
    Json,
};

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

/// ## JsonResponse | HttpResponse
///
/// Represents a structured HTTP response
/// that can be used in Axum applications.
///
/// It implements `IntoResponse` to convert
/// the response into an Axum-compatible response.
///
/// The IntoResponse returns a Response follows a standard JSON structure:
///
/// ```json
/// {
///    "code": 200,
///    "success": true,
///    "message": "OK",
///    "timestamp": "2023-10-01T12:00:00Z",
///    "requestId": "optional-request-id"
/// }
/// ```
/// Additionally, it can include optional fields such as `data`, `error`, and `errors`
/// to provide more context about the response.
///
/// ### Http Code Variants
/// The struct provides methods for common HTTP status codes for example:
/// - `HttpResponse::Ok()` for 200 OK
/// - `HttpResponse::NotFound()` for 404 Not Found
#[derive(Debug)]
pub struct JsonResponse {
    request_id: Option<Box<str>>,
    json: Box<Map<String, Value>>,
    code: StatusCode,
    message: Box<str>,
    headers: Option<HashMap<HeaderName, HeaderValue>>,
}

/// The body structure of the JSON response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponseBody {
    pub request_id: Option<Box<str>>,
    pub code: u16,
    pub success: bool,
    pub message: Box<str>,
    pub timestamp: String,
    pub data: Option<Value>,
    pub error: Option<Value>,
    pub errors: Option<Value>,
}

impl JsonResponse {
    #[doc(hidden)]
    pub fn builder(code: impl TryInto<StatusCode>) -> Self {
        let code = code.try_into().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        Self {
            code,
            json: Box::new(Map::new()),
            message: code.canonical_reason().unwrap_or("No Message").into(),
            request_id: None,
            headers: None,
        }
    }

    /// Sets the request ID for the response.
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into().into_boxed_str());
        self
    }

    /// Sets the response message.
    /// The `message` parameter should be convertible to a `String`.
    /// This message is typically a human-readable description of the response.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into().into_boxed_str();
        self
    }

    /// Adds a header to the response.
    /// The `key` and `value` parameters should be convertible to `HeaderName` and `HeaderValue`, respectively.
    /// If the conversion fails, the header is not added.
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(header_name), Ok(header_value)) =
            (HeaderName::try_from(key), HeaderValue::try_from(value))
        {
            (*self.headers.get_or_insert_with(HashMap::new)).insert(header_name, header_value);
        }

        self
    }

    /// Adds `data` field to the response.
    /// The `data` parameter should implement `Serialize`.
    /// If serialization fails, it logs a warning and sets `data` to an error message.
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        let data = serde_json::to_value(data).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response 'data' field: {err}");
            Value::String("Serialization failed".into())
        });

        self.json.insert("data".into(), data);

        self
    }

    /// Adds `error` field to the response.
    /// The `error` parameter should implement `Serialize`.
    /// If serialization fails, it logs a warning and sets `error` to an error message.
    pub fn error<T: Serialize>(mut self, error: T) -> Self {
        let error = serde_json::to_value(error).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response 'error' field: {err}");
            Value::String("Serialization failed".into())
        });

        self.json.insert("error".into(), error);

        self
    }

    /// Adds `errors` field to the response.
    /// The `errors` parameter should implement `Serialize`.
    /// If serialization fails, it logs a warning and sets `errors` to an error message.
    pub fn errors<T: Serialize>(mut self, errors: T) -> Self {
        let errors = serde_json::to_value(errors).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response errors field: {err}");
            Value::String("Serialization failed".into())
        });

        self.json.insert("errors".into(), errors);

        self
    }

    #[doc(hidden)]
    pub fn builder_u16(code: u16) -> Self {
        let status_code = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self::builder(status_code)
    }
}

impl AxumIntoResponse for JsonResponse {
    fn into_response(self) -> AxumResponse {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let mut body = json!({
            "code": self.code.as_u16(),
            "success": self.code.is_success(),
            "message": self.message,
            "timestamp": timestamp,
        });

        if let Some(request_id) = self.request_id {
            body["request_id"] = Value::String(request_id.into());
        }

        if let Some(data) = self.json.get("data") {
            body["data"] = data.clone();
        }

        if let Some(error) = self.json.get("error") {
            body["error"] = error.clone();
        }

        if let Some(errors) = self.json.get("errors") {
            body["errors"] = errors.clone();
        }

        let mut response = (self.code, Json(body)).into_response();

        if let Some(headers) = self.headers {
            for (key, value) in headers.iter() {
                response.headers_mut().insert(key, value.clone());
            }
        }

        response
    }
}

impl JsonResponse {
    /// 100 Continue
    /// [[RFC9110, Section 15.2.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.1)]
    pub fn Continue() -> Self {
        Self::builder(StatusCode::CONTINUE)
    }

    /// 101 Switching Protocols
    /// [[RFC9110, Section 15.2.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.2)]
    pub fn SwitchingProtocols() -> Self {
        Self::builder(StatusCode::SWITCHING_PROTOCOLS)
    }

    /// 102 Processing
    /// [[RFC2518, Section 10.1](https://datatracker.ietf.org/doc/html/rfc2518#section-10.1)]
    pub fn Processing() -> Self {
        Self::builder(StatusCode::PROCESSING)
    }

    /// 200 OK
    /// [[RFC9110, Section 15.3.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.1)]
    pub fn Ok() -> Self {
        Self::builder(StatusCode::OK)
    }

    /// 201 Created
    /// [[RFC9110, Section 15.3.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.2)]
    pub fn Created() -> Self {
        Self::builder(StatusCode::CREATED)
    }

    /// 202 Accepted
    /// [[RFC9110, Section 15.3.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.3)]
    pub fn Accepted() -> Self {
        Self::builder(StatusCode::ACCEPTED)
    }

    /// 203 Non-Authoritative Information
    /// [[RFC9110, Section 15.3.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.4)]
    pub fn NonAuthoritativeInformation() -> Self {
        Self::builder(StatusCode::NON_AUTHORITATIVE_INFORMATION)
    }

    /// 204 No Content
    /// [[RFC9110, Section 15.3.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.5)]
    pub fn NoContent() -> Self {
        Self::builder(StatusCode::NO_CONTENT)
    }

    /// 205 Reset Content
    /// [[RFC9110, Section 15.3.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.6)]
    pub fn ResetContent() -> Self {
        Self::builder(StatusCode::RESET_CONTENT)
    }

    /// 206 Partial Content
    /// [[RFC9110, Section 15.3.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.7)]
    pub fn PartialContent() -> Self {
        Self::builder(StatusCode::PARTIAL_CONTENT)
    }

    /// 207 Multi-Status
    /// [[RFC4918, Section 11.1](https://datatracker.ietf.org/doc/html/rfc4918#section-11.1)]
    pub fn MultiStatus() -> Self {
        Self::builder(StatusCode::MULTI_STATUS)
    }

    /// 208 Already Reported
    /// [[RFC5842, Section 7.1](https://datatracker.ietf.org/doc/html/rfc5842#section-7.1)]
    pub fn AlreadyReported() -> Self {
        Self::builder(StatusCode::ALREADY_REPORTED)
    }

    /// 226 IM Used
    /// [[RFC3229, Section 10.4.1](https://datatracker.ietf.org/doc/html/rfc3229#section-10.4.1)]
    pub fn ImUsed() -> Self {
        Self::builder(StatusCode::IM_USED)
    }

    /// 300 Multiple Choices
    /// [[RFC9110, Section 15.4.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.1)]
    pub fn MultipleChoices() -> Self {
        Self::builder(StatusCode::MULTIPLE_CHOICES)
    }

    /// 301 Moved Permanently
    /// [[RFC9110, Section 15.4.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.2)]
    pub fn MovedPermanently(location: &str) -> Self {
        Self::builder(StatusCode::MOVED_PERMANENTLY).add_header("Location", location)
    }

    /// 302 Found
    /// [[RFC9110, Section 15.4.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.3)]
    pub fn Found(location: &str) -> Self {
        Self::builder(StatusCode::FOUND).add_header("Location", location)
    }

    /// 303 See Other
    /// [[RFC9110, Section 15.4.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.4)]
    pub fn SeeOther(location: &str) -> Self {
        Self::builder(StatusCode::SEE_OTHER).add_header("Location", location)
    }

    /// 304 Not Modified
    /// [[RFC9110, Section 15.4.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.5)]
    pub fn NotModified() -> Self {
        Self::builder(StatusCode::NOT_MODIFIED)
    }

    /// 305 Use Proxy
    /// [[RFC9110, Section 15.4.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.6)]
    pub fn UseProxy() -> Self {
        Self::builder(StatusCode::USE_PROXY)
    }

    /// 307 Temporary Redirect
    /// [[RFC9110, Section 15.4.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.7)]
    pub fn TemporaryRedirect(location: &str) -> Self {
        Self::builder(StatusCode::TEMPORARY_REDIRECT).add_header("Location", location)
    }

    /// 308 Permanent Redirect
    /// [[RFC9110, Section 15.4.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.8)]
    pub fn PermanentRedirect(location: &str) -> Self {
        Self::builder(StatusCode::PERMANENT_REDIRECT).add_header("Location", location)
    }

    /// 400 Bad Request
    /// [[RFC9110, Section 15.5.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1)]
    pub fn BadRequest() -> Self {
        Self::builder(StatusCode::BAD_REQUEST)
    }

    /// 401 Unauthorized
    /// [[RFC9110, Section 15.5.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.2)]
    pub fn Unauthorized() -> Self {
        Self::builder(StatusCode::UNAUTHORIZED)
    }

    /// 402 Payment Required
    /// [[RFC9110, Section 15.5.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.3)]
    pub fn PaymentRequired() -> Self {
        Self::builder(StatusCode::PAYMENT_REQUIRED)
    }

    /// 403 Forbidden
    /// [[RFC9110, Section 15.5.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.4)]
    pub fn Forbidden() -> Self {
        Self::builder(StatusCode::FORBIDDEN)
    }

    /// 404 Not Found
    /// [[RFC9110, Section 15.5.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.5)]
    pub fn NotFound() -> Self {
        Self::builder(StatusCode::NOT_FOUND)
    }

    /// 405 Method Not Allowed
    /// [[RFC9110, Section 15.5.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.6)]
    pub fn MethodNotAllowed() -> Self {
        Self::builder(StatusCode::METHOD_NOT_ALLOWED)
    }

    /// 406 Not Acceptable
    /// [[RFC9110, Section 15.5.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.7)]
    pub fn NotAcceptable() -> Self {
        Self::builder(StatusCode::NOT_ACCEPTABLE)
    }

    /// 407 Proxy Authentication Required
    /// [[RFC9110, Section 15.5.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.8)]
    pub fn ProxyAuthenticationRequired() -> Self {
        Self::builder(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }

    /// 408 Request Timeout
    /// [[RFC9110, Section 15.5.9](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.9)]
    pub fn RequestTimeout() -> Self {
        Self::builder(StatusCode::REQUEST_TIMEOUT)
    }

    /// 409 Conflict
    /// [[RFC9110, Section 15.5.10](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.10)]
    pub fn Conflict() -> Self {
        Self::builder(StatusCode::CONFLICT)
    }

    /// 410 Gone
    /// [[RFC9110, Section 15.5.11](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.11)]
    pub fn Gone() -> Self {
        Self::builder(StatusCode::GONE)
    }

    /// 411 Length Required
    /// [[RFC9110, Section 15.5.12](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.12)]
    pub fn LengthRequired() -> Self {
        Self::builder(StatusCode::LENGTH_REQUIRED)
    }

    /// 412 Precondition Failed
    /// [[RFC9110, Section 15.5.13](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.13)]
    pub fn PreconditionFailed() -> Self {
        Self::builder(StatusCode::PRECONDITION_FAILED)
    }

    /// 413 Payload Too Large
    /// [[RFC9110, Section 15.5.14](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.14)]
    pub fn PayloadTooLarge() -> Self {
        Self::builder(StatusCode::PAYLOAD_TOO_LARGE)
    }

    /// 414 URI Too Long
    /// [[RFC9110, Section 15.5.15](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.15)]
    pub fn UriTooLong() -> Self {
        Self::builder(StatusCode::URI_TOO_LONG)
    }

    /// 415 Unsupported Media Type
    /// [[RFC9110, Section 15.5.16](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.16)]
    pub fn UnsupportedMediaType() -> Self {
        Self::builder(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    /// 416 Range Not Satisfiable
    /// [[RFC9110, Section 15.5.17](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.17)]
    pub fn RangeNotSatisfiable() -> Self {
        Self::builder(StatusCode::RANGE_NOT_SATISFIABLE)
    }

    /// 417 Expectation Failed
    /// [[RFC9110, Section 15.5.18](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.18)]
    pub fn ExpectationFailed() -> Self {
        Self::builder(StatusCode::EXPECTATION_FAILED)
    }

    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324, Section 2.3.2](https://datatracker.ietf.org/doc/html/rfc2324#section-2.3.2)]
    pub fn ImATeapot() -> Self {
        Self::builder(StatusCode::IM_A_TEAPOT)
    }

    /// 421 misdirected request
    /// [[rfc9110, section 15.5.20](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.20)]
    pub fn MisdirectedRequest() -> Self {
        Self::builder(StatusCode::MISDIRECTED_REQUEST)
    }

    /// 422 Unprocessable Entity
    /// [[RFC9110, Section 15.5.21](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.21)]
    pub fn UnprocessableEntity() -> Self {
        Self::builder(StatusCode::UNPROCESSABLE_ENTITY)
    }

    /// 423 Locked
    /// [[RFC4918, Section 11.3](https://datatracker.ietf.org/doc/html/rfc4918#section-11.3)]
    pub fn Locked() -> Self {
        Self::builder(StatusCode::LOCKED)
    }

    /// 424 Failed Dependency
    /// [[RFC4918, Section 11.4](https://tools.ietf.org/html/rfc4918#section-11.4)]
    pub fn FailedDependency() -> Self {
        Self::builder(StatusCode::FAILED_DEPENDENCY)
    }

    /// 425 Too early
    /// [[RFC8470, Section 5.2](https://httpwg.org/specs/rfc8470.html#status)]
    pub fn TooEarly() -> Self {
        Self::builder(StatusCode::TOO_EARLY)
    }

    /// 426 Upgrade Required
    /// [[RFC9110, Section 15.5.22](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.22)]
    pub fn UpgradeRequired() -> Self {
        Self::builder(StatusCode::UPGRADE_REQUIRED)
    }

    /// 428 Precondition Required
    /// [[RFC6585, Section 3](https://datatracker.ietf.org/doc/html/rfc6585#section-3)]
    pub fn PreconditionRequired() -> Self {
        Self::builder(StatusCode::PRECONDITION_REQUIRED)
    }

    /// 429 Too Many Requests
    /// [[RFC6585, Section 4](https://datatracker.ietf.org/doc/html/rfc6585#section-4)]
    pub fn TooManyRequests() -> Self {
        Self::builder(StatusCode::TOO_MANY_REQUESTS)
    }

    /// 431 Request Header Fields Too Large
    /// [[RFC6585, Section 5](https://datatracker.ietf.org/doc/html/rfc6585#section-5)]
    pub fn RequestHeaderFieldsTooLarge() -> Self {
        Self::builder(StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }

    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725, Section 3](https://tools.ietf.org/html/rfc7725#section-3)]
    pub fn UnavailableForLegalReasons() -> Self {
        Self::builder(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }

    /// 500 Internal Server Error
    /// [[RFC9110, Section 15.6.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1)]
    pub fn InternalServerError() -> Self {
        Self::builder(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// 501 Not Implemented
    /// [[RFC9110, Section 15.6.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.2)]
    pub fn NotImplemented() -> Self {
        Self::builder(StatusCode::NOT_IMPLEMENTED)
    }

    /// 502 Bad Gateway
    /// [[RFC9110, Section 15.6.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.3)]
    pub fn BadGateway() -> Self {
        Self::builder(StatusCode::BAD_GATEWAY)
    }

    /// 503 Service Unavailable
    /// [[RFC9110, Section 15.6.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.4)]
    pub fn ServiceUnavailable() -> Self {
        Self::builder(StatusCode::SERVICE_UNAVAILABLE)
    }

    /// 504 Gateway Timeout
    /// [[RFC9110, Section 15.6.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.5)]
    pub fn GatewayTimeout() -> Self {
        Self::builder(StatusCode::GATEWAY_TIMEOUT)
    }

    /// 505 HTTP Version Not Supported
    /// [[RFC9110, Section 15.6.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.6)]
    pub fn HttpVersionNotSupported() -> Self {
        Self::builder(StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }

    /// 506 Variant Also Negotiates
    /// [[RFC2295, Section 8.1](https://datatracker.ietf.org/doc/html/rfc2295#section-8.1)]
    pub fn VariantAlsoNegotiates() -> Self {
        Self::builder(StatusCode::VARIANT_ALSO_NEGOTIATES)
    }

    /// 507 Insufficient Storage
    /// [[RFC4918, Section 11.5](https://datatracker.ietf.org/doc/html/rfc4918#section-11.5)]
    pub fn InsufficientStorage() -> Self {
        Self::builder(StatusCode::INSUFFICIENT_STORAGE)
    }

    /// 508 Loop Detected
    /// [[RFC5842, Section 7.2](https://datatracker.ietf.org/doc/html/rfc5842#section-7.2)]
    pub fn LoopDetected() -> Self {
        Self::builder(StatusCode::LOOP_DETECTED)
    }

    /// 510 Not Extended
    /// [[RFC2774, Section 7](https://datatracker.ietf.org/doc/html/rfc2774#section-7)]
    pub fn NotExtended() -> Self {
        Self::builder(StatusCode::NOT_EXTENDED)
    }

    /// 511 Network Authentication Required
    /// [[RFC6585, Section 6](https://datatracker.ietf.org/doc/html/rfc6585#section-6)]
    pub fn NetworkAuthenticationRequired() -> Self {
        Self::builder(StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}
