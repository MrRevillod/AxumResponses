use std::collections::HashMap;

use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};

use serde_json::Value;

pub type ControllerResult = Result<HttpResponse, HttpResponse>;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    status: StatusCode,
    body: Value,
    headers: HashMap<HeaderName, HeaderValue>,
}

impl HttpResponse {
    pub fn build() -> Self {
        Self {
            status: StatusCode::OK,
            body: Value::Null,
            headers: HashMap::new(),
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn code(mut self, code: u16) -> Self {
        let status = ::axum::http::StatusCode::from_u16(code);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", code);
        };

        self.status = status;
        self
    }

    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    pub fn json<T: serde::Serialize>(mut self, data: T) -> Self {
        match serde_json::to_value(data) {
            Ok(value) => {
                self.body = value;
                self
            }
            Err(e) => {
                eprintln!("Failed to serialize data to JSON: {}", e);
                self.status = StatusCode::INTERNAL_SERVER_ERROR;
                self.body = serde_json::json!({
                    "error": "Failed to serialize response data"
                });
                self
            }
        }
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(header_name), Ok(header_value)) = (
            HeaderName::try_from(key),
            HeaderValue::try_from(value)
        ) {
            self.headers.insert(header_name, header_value);
        }
        self
    }

    pub fn wrap_ok(self) -> Result<Self, ()> {
        Ok(self)
    }

    pub fn wrap_err(self) -> Result<(), Self> {
        Err(self)
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status, Json(self.body)).into_response();
        
        for (key, value) in self.headers.iter() {
            response.headers_mut().insert(key, value.clone());
        }
        
        response
    }
}

#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ "status": status.to_string() });

        $crate::http::HttpResponse::build()
            .status(status)
            .body(json)
    }};

    ($status:expr, { $value:ident }) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::to_value(&$value).unwrap_or_else(|_| {
            panic!("Failed to serialize value to JSON");
        });

        $crate::http::HttpResponse::build()
            .status(status)
            .body(json)
    }};

    ($status:expr, { $($json:tt)* }) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ $($json)* });

        $crate::http::HttpResponse::build()
            .status(status)
            .body(json)
    }};
}
