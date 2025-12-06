/// Creates a JSON response with the specified status code and optional data.
///
/// # Examples
///
/// ```rust
/// use axum_responses::response;
/// use serde::Serialize;
///
/// let resp = response!(200);
/// let resp = response!(200, { "message": "Success!" });
///
/// // Response with data object
/// #[derive(Serialize)]
/// struct User { name: String }
///
/// let user = User { name: "John".to_string() };
/// let resp = response!(201, { user });
/// ```
#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        $crate::JsonResponse::status($status)
    }};
    ($status:expr, { $value:ident }) => {{
        let json = ::serde_json::to_value(&$value).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response data: {err}");
            ::serde_json::json!({ "error": "Serialization failed" })
        });

        $crate::JsonResponse::status($status).data(json)
    }};
    ($status:expr, { $($json:tt)* }) => {{
        let mut json = ::serde_json::json!({ $($json)* });
        let mut response = $crate::JsonResponse::status($status);

        let message = json.get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(obj) = json.as_object_mut() {
            if message.is_some() {
                obj.remove("message");
            }
        }

        if let ::serde_json::Value::Object(obj) = &json {
            if obj.is_empty() {
                json = ::serde_json::Value::Null;
            }
        }

        if let Some(msg) = message {
            response = response.message(msg);
        }

        response.data(json)
    }};
}
