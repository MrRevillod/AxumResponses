
mod tests;
pub mod extra;
pub mod impls;

use extra::*;
use serde_json::Value;

/// `AxumResponse` data type that represents an HTTP response. 
/// Can be used as a return type of a controller.
 
pub type AxumResponse = Result<HttpResponse, HttpResponse>;

/// `AxumResult` data type that represents a response 
/// from some service in the API.
/// 
/// ### Parameters
/// 
/// * `T`: Data type of the response.
/// 
/// ### Example
/// Returns a type T if the response is successful, 
/// otherwise it returns a negative `ApiResponse`, 
/// that is, an error HttpResponse.
 
pub type AxumResult<T> = Result<T, HttpResponse>;

pub enum Response {
    
    /// `Standard` is a standard response.
    /// 
    /// ### Parameters
    /// 
    /// * `u16`: HTTP status code.
    /// * `&'static str`: Response message.
     
    Standard(u16, &'static str),

    /// `JsonData` is a response that contains data.
    /// 
    /// ### Parameters
    /// 
    /// * `u16`: HTTP status code.
    /// * `&'static str`: Response message.
    /// * `&'static str`: Name | key of the Value.
    /// * `Value`: The data of the response.
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// use axum_responses::Response;
    /// use axum_responses::extra::ToJson;
    /// 
    /// use serde_json::Value;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct TestStruct {
    ///    field: String
    /// }
    /// 
    /// impl ToJson for TestStruct {}
    /// 
    /// let test_struct = TestStruct {
    ///     field: "value".to_string()
    /// };
    /// 
    /// let response = Response::JsonData(
    ///     200, "Success", "data", test_struct.to_json()
    /// );
    /// ```

    JsonData(u16, &'static str, &'static str, Value)
}

#[allow(non_camel_case_types)]
pub enum HttpResponse {
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
    REQUEST_ENTITY_TOO_LARGE,
    REQUEST_URI_TOO_LONG,
    UNSUPPORTED_MEDIA_TYPE,
    REQUESTED_RANGE_NOT_SATISFIABLE,
    EXPECTATION_FAILED,
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    CUSTOM(u16, &'static str),
    JSON(u16, &'static str, &'static str, Value),
}
