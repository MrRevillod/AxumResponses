use crate::http::HttpResponse;
use crate::{response, Result};

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

pub async fn http_response_error_handler() -> Result<HttpResponse> {
    Err(HttpResponse::BadRequest()
            .message("This is an error response")
            .data(json!({
                "type": "ValidationError",
                "errors": {
                    "param1": "Invalid value for param1",
                    "param2": "Param2 is required"
                },
            })))
}
