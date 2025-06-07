mod handlers;

use handlers::{
    http_response_data_handler, http_response_error_handler, http_response_macro_handler,
    http_response_simple_handler, single_object_response_handler
};

use axum::routing::get;
use axum::Router;
use axum_test::TestServer;

#[allow(dead_code)]
fn app() -> TestServer {
    let router = Router::new()
        .route("/http-response-ok", get(http_response_simple_handler))
        .route("/http-response-macro", get(http_response_macro_handler))
        .route("/http-response-data", get(http_response_data_handler))
        .route("/http-response-error", get(http_response_error_handler))
        .route("/single-object-response", get(single_object_response_handler));

    TestServer::new(router).unwrap()
}

#[tokio::test]
async fn test_http_response_simple() {
    use crate::http::ResponseBody;

    let server = app();
    let response = server.get("/http-response-ok").await;

    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code().as_u16(), 200_u16);
    assert_eq!(body.message, "This is a simple response");
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
    use crate::http::ResponseBody;

    let server = app();
    let response = server.get("/http-response-data").await;

    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code().as_u16(), 201_u16);
    assert_eq!(body.message, "Item created successfully");

    let Some(data) = body.data else {
        panic!("Expected data field in response body");
    };

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
    use crate::http::ResponseBody;

    let server = app();
    let response = server.get("/http-response-error").await;

    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code().as_u16(), 400_u16);
    assert_eq!(body.message, "This is an error response");

    let Some(error) = body.data else {
        panic!("Expected data field in response body");
    };

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
    use crate::http::ResponseBody;

    let server = app();
    let response = server.get("/single-object-response").await;

    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code().as_u16(), 200_u16);

    let Some(data) = body.data else {
        panic!("Expected data field in response body");
    };

    assert_eq!(data.get("id").unwrap().as_u64().unwrap(), 1);
    assert_eq!(data.get("name").unwrap().as_str().unwrap(), "Test Object");
}
