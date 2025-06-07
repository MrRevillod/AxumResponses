#[cfg(test)]
mod tests;

#[allow(non_snake_case)]
pub mod http;

#[deprecated(note = "Use `http::HttpResponse` variants or builder instead")]
pub mod standard;

use http::HttpResponse;

/// This type alias is used to simplify the result type for handlers.
/// It represents a result that can either be an Type `T` or an `HttpResponse`.
///
/// Can be used in handler functions to return either a successful
/// HttpResponse with data or an error HttpResponse.
///
/// In this context you can use it like this:
/// ```rust
/// use axum_responses::http::HttpResponse;
/// use axum_responses::Result;
///
/// async fn axum_post_handler() -> Result<HttpResponse> {
///     Ok(HttpResponse::Created()
///         .message("Data saved successfully!")
///     )
/// }
/// ```
/// Also this type is util to map errors to `HttpResponse` types directly.
/// So this allows to use the `?` to map errors to `HttpResponse` types directly,
/// without needing to manually construct the `HttpResponse` each time.
pub type Result<T> = std::result::Result<T, HttpResponse>;

/// This macro is used to create a response builder with a specified status code.
/// It's a shorthand for constructing an `HttpResponse` with a status code.
/// It can also be used to create a response with a JSON body.
/// The macro takes a status code as an argument, and optionally a JSON object.
///
/// If a JSON object is provided, it will be serialized and included in the response body `data` key.
/// If the JSON object contains a `message` key, it will be extracted and set as the
/// response message and removed from the data object.
#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status)
            .expect("Invalid status code");

        $crate::http::HttpResponse::builder(status)
            .message(status.canonical_reason().unwrap_or("No reason"))
    }};
    ($status:expr, { $value:ident }) => {{
        let status = ::axum::http::StatusCode::from_u16($status)
            .expect("Invalid status code");

        let json = ::serde_json::to_value(&$value).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response data: {err}");
            json!({ "error": "Serialization failed" })
        });

        $crate::http::HttpResponse::builder(status)
            .message(status.canonical_reason().unwrap_or("No reason"))
            .data(json)
    }};
    ($status:expr, { $($json:tt)* }) => {{
        let status = ::axum::http::StatusCode::from_u16($status)
            .expect("Invalid status code");

        let mut json = ::serde_json::json!({ $($json)* });

        let message = json.get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(obj) = json.as_object_mut() {
            obj.remove("message");
        }

        let mut response = $crate::http::HttpResponse::builder(status)
            .message(status.canonical_reason().unwrap_or("No reason"))
            .data(json);

        if let Some(msg) = message {
            response = response.message(msg);
        }

        response
    }};
}
