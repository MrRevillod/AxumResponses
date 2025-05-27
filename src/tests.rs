#[cfg(test)]
mod tests {

    use crate::http::HttpResponse;
    use crate::response;
    use crate::standard::Response;

    use axum::routing::get;
    use axum::{http::StatusCode, Router};
    use axum_test::TestServer;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseStruct {
        message: String,
    }

    async fn response_custom_data_handler() -> Response {
        let _: Result<(), Response> = Err("error").map_err(|_| Response::INTERNAL_SERVER_ERROR);

        let data = ResponseStruct {
            message: "Hello, world!".to_string(),
        };

        Response::CUSTOM(200, json!({ "data": data, "status": "success" }))
    }

    async fn http_response_handler() -> HttpResponse {
        HttpResponse::build()
            .status(StatusCode::OK)
            .add_header("Content-Type", "application/json")
            .body(json!({ "hi": "Hello, world!" }))
    }

    // Nuevo handler que demuestra el uso del método json()
    async fn http_response_json_handler() -> HttpResponse {
        let data = ResponseStruct {
            message: "Hello from json method!".to_string(),
        };

        HttpResponse::build()
            .status(StatusCode::OK)
            .add_header("Content-Type", "application/json")
            .json(data) // Usando el nuevo método json()
    }

    // Otro handler que demuestra json() con structs más complejos
    async fn http_response_complex_json_handler() -> HttpResponse {
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

        response!(201, { complex_data })
    }

    fn app() -> Router {
        Router::new()
            .route("/response", get(response_custom_data_handler))
            .route("/http-response", get(http_response_handler))
            .route("/http-response-json", get(http_response_json_handler))
            .route("/http-response-complex", get(http_response_complex_json_handler))
    }

    #[tokio::test]
    async fn test_response() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/response").await;
        let json = response.json::<Value>();

        let message = json
            .get("data")
            .and_then(|data| data.get("message"))
            .and_then(|message| message.as_str())
            .unwrap();

        let status = json
            .get("status")
            .and_then(|status| status.as_str())
            .unwrap();

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(message, "Hello, world!");
        assert_eq!(status, "success");
    }

    #[tokio::test]
    async fn test_http_response() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/http-response").await;
        let json = response.json::<Value>();

        let message = json
            .get("hi")
            .and_then(|message| message.as_str())
            .unwrap();

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(message, "Hello, world!");

        dbg!(response.headers());
    }

    #[tokio::test]
    async fn test_http_response_json_method() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/http-response-json").await;
        let json = response.json::<Value>();

        let message = json
            .get("message")
            .and_then(|message| message.as_str())
            .unwrap();

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(message, "Hello from json method!");
    }

    #[tokio::test]
    async fn test_http_response_complex_json() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/http-response-complex").await;
        let json = response.json::<Value>();

        let name = json
            .get("name")
            .and_then(|name| name.as_str())
            .unwrap();

        let id = json
            .get("id")
            .and_then(|id| id.as_u64())
            .unwrap();

        assert_eq!(response.status_code(), StatusCode::CREATED);
        assert_eq!(name, "Test Item");
        assert_eq!(id, 1);
    }
}