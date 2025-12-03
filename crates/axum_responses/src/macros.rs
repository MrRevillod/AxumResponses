/// The `response!` macro allows you to create `JsonResponse` responses
/// with a status code and a JSON body being more lax.
/// It also supports auto-serialization of structs that implement `Serialize`.
///
/// ```rust
/// use axum_responses::{response, JsonResponse};
/// async fn example_handler() -> JsonResponse {
///     response!(200, { "page": 10, "total": 100, "message": "OK" })
/// }
/// ```
///
/// Resulting Response:
///
/// ```json
/// {
///   "code": 200,
///   "success": true,
///   "message": "OK",
///   "timestamp": "2023-10-01T12:00:00Z",
///   "data": {
///     "page": 10,
///     "total": 100
///   }
/// }
/// ```
///
/// The macro also supports single objects in the `data` field,
/// which is useful for returning a single resource or entity.
/// This is designed to be similar to javascript notation.
///
/// ```rust,ignore
/// use axum_responses::{response, JsonResponse};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Product {
///     id: String,
///     name: String,
///     price: f64,
/// }
///
/// async fn product_handler() -> JsonResponse {
///     let product_data = Product {
///         id: "prod_123".to_string(),
///         name: "Example Product".to_string(),
///         price: 99.99,
///     };
///
///     response!(201, { product_data })
/// }
/// ```
/// Resulting Response
///
/// ```json
/// {
///   "code": 201,
///   "success": true,
///   "message": "Created",
///   "timestamp": "2023-10-01T12:00:00Z",
///   "data": {
///     "id": "prod_123",
///     "name": "Example Product",
///     "price": 99.99
///   }
/// }
/// ```
#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        $crate::JsonResponse::builder($status)
    }};
    ($status:expr, { $value:ident }) => {{
        let json = ::serde_json::to_value(&$value).unwrap_or_else(|err| {
            eprintln!("Warning: Failed to serialize response data: {err}");
            json!({ "error": "Serialization failed" })
        });

        $crate::JsonResponse::builder($status).data(json)
    }};
    ($status:expr, { $($json:tt)* }) => {{
        let mut json = ::serde_json::json!({ $($json)* });
        let mut response = $crate::JsonResponse::builder($status);

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
