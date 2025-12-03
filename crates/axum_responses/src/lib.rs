mod file;
mod macros;

#[allow(non_snake_case)]
mod json;

pub use axum_responses_macros::HttpError;
pub use file::{ContentDisposition, FileResponse, FileResult};
pub use json::{JsonResponse, JsonResponseBody};
pub use thiserror::Error;

/// Type alias for standard JSON responses. As this library is primarily focused
/// on JSON first responses, this `HttpResponse` alias is provided for convenience.
pub type HttpResponse = JsonResponse;

/// A specialized `Result` for Axum handlers that return JSON responses.
///
/// This type alias can receive a generic type parameter `T` which
/// defaults to `JsonResponse`.
pub type HttpResult<T = JsonResponse> = std::result::Result<T, JsonResponse>;

#[cfg(test)]
mod tests {
    use crate::{response, HttpResponse, HttpResult, JsonResponseBody};
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    use serde::Serialize;
    use serde_json::json;

    pub async fn http_response_simple_handler() -> HttpResponse {
        HttpResponse::Ok().message("This is a simple response")
    }

    pub async fn http_response_macro_handler() -> HttpResponse {
        response!(200, { "message": "This is the response macro!" })
    }

    pub async fn single_object_response_handler() -> HttpResponse {
        #[derive(Serialize)]
        struct SingleObject {
            id: u32,
            name: String,
        }

        let single_object = SingleObject {
            id: 1,
            name: "Test Object".to_string(),
        };

        response!(200, { single_object })
    }

    pub async fn http_response_data_handler() -> HttpResponse {
        #[derive(Serialize)]
        struct ComplexData {
            id: u32,
            name: String,
            tags: Vec<String>,
        }

        let complex_data = ComplexData {
            id: 1,
            name: "Test Item".to_string(),
            tags: vec!["test".to_string(), "demo".to_string()],
        };

        HttpResponse::Created()
            .data(complex_data)
            .message("Item created successfully")
    }

    pub async fn http_response_error_handler() -> HttpResult {
        Err(HttpResponse::BadRequest()
            .message("This is an error response")
            .error(json!({
                "type": "ValidationError",
                "errors": {
                    "param1": "Invalid value for param1",
                    "param2": "Param2 is required"
                },
            })))
    }

    pub async fn http_message_macro_handler() -> HttpResponse {
        response!(200, { "message": "This is a message macro response!" })
    }

    pub async fn http_no_data_handler() -> HttpResponse {
        HttpResponse::Ok().message("This is a no data response")
    }

    #[allow(dead_code)]
    fn app() -> TestServer {
        let router = Router::new()
            .route("/http-response-ok", get(http_response_simple_handler))
            .route("/http-response-macro", get(http_response_macro_handler))
            .route("/http-response-data", get(http_response_data_handler))
            .route("/http-response-error", get(http_response_error_handler))
            .route(
                "/single-object-response",
                get(single_object_response_handler),
            )
            .route("/http-message-macro", get(http_message_macro_handler))
            .route("/http-no-data", get(http_no_data_handler));

        TestServer::new(router).unwrap()
    }

    #[tokio::test]
    async fn test_http_response_simple() {
        let server = app();
        let response = server.get("/http-response-ok").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 200_u16);
        assert_eq!(*body.message, *"This is a simple response");
    }

    #[tokio::test]
    async fn test_http_response_macro() {
        let server = app();
        let response = server.get("/http-response-macro").await;

        let body = response.json::<serde_json::Value>();
        let message = body.get("message").unwrap().as_str().unwrap();

        assert_eq!(response.status_code().as_u16(), 200_u16);
        assert_eq!(message, "This is the response macro!");
    }

    #[tokio::test]
    async fn test_http_response_data() {
        let server = app();
        let response = server.get("/http-response-data").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 201_u16);
        assert_eq!(*body.message, *"Item created successfully");

        assert!(body.data.is_some());
        let data = body.data.unwrap();

        assert_eq!(data.get("id").unwrap().as_u64().unwrap(), 1);
        assert_eq!(data.get("name").unwrap().as_str().unwrap(), "Test Item");
        assert_eq!(
            data.get("tags").unwrap().as_array().unwrap(),
            &vec![
                serde_json::Value::String("test".to_string()),
                serde_json::Value::String("demo".to_string())
            ]
        );
    }

    #[tokio::test]
    async fn test_http_response_error() {
        let server = app();
        let response = server.get("/http-response-error").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 400_u16);
        assert_eq!(*body.message, *"This is an error response");

        assert!(body.error.is_some());
        let error = body.error.unwrap();

        assert_eq!(
            error.get("type").unwrap().as_str().unwrap(),
            "ValidationError"
        );

        let errors = error.get("errors").unwrap().as_object().unwrap();
        assert_eq!(
            errors.get("param1").unwrap().as_str().unwrap(),
            "Invalid value for param1"
        );
        assert_eq!(
            errors.get("param2").unwrap().as_str().unwrap(),
            "Param2 is required"
        );
    }

    #[tokio::test]
    async fn test_single_object_response() {
        let server = app();
        let response = server.get("/single-object-response").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 200_u16);

        assert!(body.data.is_some());
        let data = body.data.unwrap();

        assert_eq!(data.get("id").unwrap().as_u64().unwrap(), 1);
        assert_eq!(data.get("name").unwrap().as_str().unwrap(), "Test Object");
    }

    #[tokio::test]
    async fn test_http_message_macro() {
        let server = app();
        let response = server.get("/http-message-macro").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 200_u16);
        assert_eq!(*body.message, *"This is a message macro response!");
        assert_eq!(body.data, None);
    }

    #[tokio::test]
    async fn test_http_no_data() {
        let server = app();
        let response = server.get("/http-no-data").await;

        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 200_u16);
        assert_eq!(*body.message, *"This is a no data response");
        assert_eq!(body.data, None);
    }
}
