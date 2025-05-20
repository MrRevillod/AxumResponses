#[cfg(test)]
mod tests {
    
    use crate::Response;
    
    use axum::{http::StatusCode, Router};
    use axum::routing::get;
    use axum_test::TestServer;
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseStruct {
        message: String,
    }

    async fn data_handler() -> Response {

        // Simulate an error
        let _: Result<(), Response> = Err("error")
            .map_err(|_| Response::INTERNAL_SERVER_ERROR);


        let data = ResponseStruct {
            message: "Hello, world!".to_string(),
        };
        
        Response::CUSTOM(200, json!({ "data": data, "status": "success" }))
    }

    fn app() -> Router {
        Router::new()
            .route("/", get(data_handler))
    }

    #[tokio::test]
    async fn test_response() {

        let server = TestServer::new(app()).unwrap();
        let response = server.get("/").await;
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
}