mod errors;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/limited", get(rate_limited))
        .route("/io", get(io_error));

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use axum::{Router, routing::get};
    use axum_responses::{JsonResponse, JsonResponseBody, response};
    use axum_test::TestServer;
    use serde::Serialize;
    use serde_json::json;

    async fn json_with_data() -> JsonResponse {
        #[derive(Serialize)]
        struct Data {
            id: u32,
            name: String,
        }

        let data = Data {
            id: 1,
            name: "Test".into(),
        };

        JsonResponse::Created().message("Created").data(data)
    }

    async fn json_with_error() -> JsonResponse {
        JsonResponse::BadRequest()
            .message("Validation failed")
            .error(json!({"field": "email"}))
    }

    async fn response_macro() -> JsonResponse {
        response!(200, { "message": "Macro works!" })
    }

    fn app() -> TestServer {
        let router = Router::new()
            .route("/data", get(json_with_data))
            .route("/error", get(json_with_error))
            .route("/macro", get(response_macro));

        TestServer::new(router).unwrap()
    }

    #[tokio::test]
    async fn test_json_with_data() {
        let server = app();
        let response = server.get("/data").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 201);
        assert_eq!(*body.message, *"Created");
        assert!(body.data.is_some());
    }

    #[tokio::test]
    async fn test_json_with_error() {
        let server = app();
        let response = server.get("/error").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 400);
        assert!(body.error.is_some());
    }

    #[tokio::test]
    async fn test_response_macro() {
        let server = app();
        let response = server.get("/macro").await;
        let body = response.json::<serde_json::Value>();

        assert_eq!(response.status_code().as_u16(), 200);
        assert_eq!(body.get("message").unwrap(), "Macro works!");
    }
}

#[cfg(test)]
mod http_error_tests {
    use axum::{Router, response::IntoResponse, routing::get};
    use axum_responses::{
        HttpError, JsonResponse, JsonResponseBody, thiserror::Error,
    };
    use axum_test::TestServer;
    use serde::Serialize;

    #[derive(Debug, Error, HttpError)]
    pub enum AuthError {
        #[error("Unauthorized")]
        #[http(code = 401)]
        Unauthorized,

        #[error("Forbidden")]
        #[http(code = 403, error = role)]
        Forbidden { role: String },
    }

    #[derive(Debug, Serialize)]
    pub struct FieldError {
        pub field: String,
        pub message: String,
    }

    #[derive(Debug, Error, HttpError)]
    pub enum ValidationError {
        #[error("Invalid input")]
        #[http(code = 400, errors = errors)]
        InvalidInput { errors: Vec<FieldError> },
    }

    #[derive(Debug, Error, HttpError)]
    pub enum AppError {
        #[error("Auth: {0}")]
        #[http(transparent)]
        Auth(#[from] AuthError),

        #[error("IO error")]
        #[http(code = 500, message = "Internal error")]
        Io(#[from] std::io::Error),
    }

    async fn auth_error() -> Result<JsonResponse, AuthError> {
        Err(AuthError::Unauthorized)
    }

    async fn error_with_field() -> Result<JsonResponse, AuthError> {
        Err(AuthError::Forbidden {
            role: "admin".into(),
        })
    }

    async fn error_with_errors() -> Result<JsonResponse, ValidationError> {
        Err(ValidationError::InvalidInput {
            errors: vec![FieldError {
                field: "email".into(),
                message: "invalid".into(),
            }],
        })
    }

    async fn transparent_error() -> Result<JsonResponse, AppError> {
        Err(AuthError::Unauthorized)?
    }

    async fn wrapped_external_error() -> Result<JsonResponse, AppError> {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "not found",
        ))?
    }

    fn app() -> TestServer {
        let router = Router::new()
            .route("/auth", get(auth_error))
            .route("/forbidden", get(error_with_field))
            .route("/validation", get(error_with_errors))
            .route("/transparent", get(transparent_error))
            .route("/io", get(wrapped_external_error));

        TestServer::new(router).unwrap()
    }

    #[tokio::test]
    async fn test_basic_http_error() {
        let server = app();
        let response = server.get("/auth").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 401);
        assert_eq!(*body.message, *"Unauthorized"); // canonical
    }

    #[tokio::test]
    async fn test_error_field() {
        let server = app();
        let response = server.get("/forbidden").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 403);
        assert_eq!(body.error.unwrap().as_str().unwrap(), "admin");
    }

    #[tokio::test]
    async fn test_errors_field() {
        let server = app();
        let response = server.get("/validation").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 400);
        assert_eq!(body.errors.unwrap().as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_transparent_delegates() {
        let server = app();
        let response = server.get("/transparent").await;

        assert_eq!(response.status_code().as_u16(), 401);
    }

    #[tokio::test]
    async fn test_fixed_code_hides_details() {
        let server = app();
        let response = server.get("/io").await;
        let body = response.json::<JsonResponseBody>();

        assert_eq!(response.status_code().as_u16(), 500);
        assert_eq!(*body.message, *"Internal error");
    }

    #[test]
    fn test_from_impl() {
        let error = AuthError::Unauthorized;
        let json = JsonResponse::from(error);
        let response = json.into_response();

        assert_eq!(response.status().as_u16(), 401);
    }
}
