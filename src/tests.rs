
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseStruct {
    status_code: u16,
    message: String,
    data: TestStruct, // could be serde_json::Value
}

#[derive(Serialize, Deserialize)]
struct TestStruct {
    field: String
}

impl ToJson for TestStruct {}

#[cfg(test)]
mod tests {
    
    use super::*;
    use axum::Router;
    use axum::routing::get;
    use axum_test::TestServer;

    async fn standard_handler() -> AxumResponse {
        Ok(HttpResponse::OK)
    }

    async fn data_handler() -> AxumResponse {
        
        let status = to_http_status(200);
        
        let data = TestStruct {
            field: "value".to_string()
        };
        
        Ok(HttpResponse::JSON(status.as_u16(), "Success", "data", data.to_json()))
    }

    fn app() -> Router {
        Router::new()
            .route("/standard", get(standard_handler))
            .route("/with-data", get(data_handler))
    }

    #[tokio::test]
    async fn test_data_response() {

        let server = TestServer::new(app()).unwrap();
        let response = server.get("/with-data").await;
        let json = response.json::<ResponseStruct>();
        
        assert_eq!(response.status_code(), to_http_status(200));
        assert_eq!(json.data.field, "value".to_string());
    }
}