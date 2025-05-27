<div align="center">
    <img src="./images/logo.png" width=250 />
</div>

<div align="center">
    <h1>Axum Responses</h1>
</div>

<div align="center">
  <a href="README.md" title="English README">🇺🇸 English</a>
  &nbsp;&nbsp;|&nbsp;&nbsp;
  <a href="README[ES].md" title="README en Español">🇪🇸 Español</a>
</div>

<div align="center">
    <strong>A simple way to manage HTTP responses and results in Axum</strong>
</div>

---

## Description

**Axum Responses** is a library designed to simplify the creation and handling of HTTP responses in applications built with [Axum](https://github.com/tokio-rs/axum). It provides a clear abstraction for handling standard and custom responses, along with useful tools like macros to reduce boilerplate code.

---

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.3.1"
```

Make sure to also include the necessary dependencies such as `axum`, `serde`, and `serde_json`.

---

## Features
- **Standard and Custom Responses**: Handle common HTTP responses like `200 OK`, `404 Not Found`.
- **Useful Macros**: Use the `response!` macro to simplify creating responses with custom status codes and bodies.
- **Integration with Axum**: Specifically designed to work with the Axum framework.

## Example Usage

```rust
use axum::{Router, routing::get};
use axum_responses::standard::Response;
use serde_json::json;

async fn handler() -> Response {
    Response::OK
}

async fn custom_handler() -> Response {
    Response::CUSTOM(201, json!({ "message": "Created successfully" }))
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/custom", get(custom_handler))
}
```

### Using `HttpResponse` for More Flexible Responses

The `HttpResponse` structure allows you to build responses with a status code, JSON body, and custom headers using a builder pattern.

```rust
use axum_responses::http::HttpResponse;
use axum::http::StatusCode;
use serde_json::json;
use serde::Serialize;

// Simple handler using HttpResponse
async fn http_response_handler() -> HttpResponse {
    HttpResponse::build()
        .status(StatusCode::OK)
        .add_header("X-Custom-Header", "custom_value")
        .body(json!({ "message": "Hello from HttpResponse" }))
}

// Handler using the `json()` method to serialize structs
#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
}

async fn http_response_json_struct_handler() -> HttpResponse {
    let user_data = User {
        id: 1,
        username: "example_user".to_string(),
    };
    HttpResponse::build()
        .status(StatusCode::OK)
        .json(user_data) // Automatically serializes `User` to JSON
}

// Example in an Axum application
use axum::{Router, routing::get};

fn app_with_http_response() -> Router {
    Router::new()
        .route("/http", get(http_response_handler))
        .route("/user", get(http_response_json_struct_handler))
}
```

### `response!` Macro

The `response!` macro allows you to create `HttpResponse` responses with a status code and a JSON body more concisely. It also supports auto-serialization of structs that implement `Serialize`.

```rust
use axum_responses::{response, http::HttpResponse};
use serde::Serialize;

async fn example_handler() -> HttpResponse {
    response!(200, { "status": "success", "data": "Example" })
}

async fn error_handler() -> HttpResponse {
    response!(404, { "error": "Resource not found" })
}

#[derive(Serialize)]
struct Product {
    id: String,
    name: String,
    price: f64,
}

async fn product_handler() -> HttpResponse {
    let product_data = Product {
        id: "prod_123".to_string(),
        name: "Example Product".to_string(),
        price: 99.99,
    };
    // The `product_data` struct will be automatically serialized to JSON
    response!(201, { product_data })
}

async fn another_error_handler() -> HttpResponse {
    response!(500, { "error": "Internal server error" })
}

// Example in an Axum application
use axum::{Router, routing::get};

fn app_with_macro() -> Router {
    Router::new()
        .route("/macro-example", get(example_handler))
        .route("/macro-product", get(product_handler))
        .route("/macro-error", get(error_handler))
}
```

### Differences Between `Response` and `HttpResponse`

- **`Response`**: An enum designed to handle standard and custom responses in a simple way. It is useful for handlers that need to return predefined responses.

- **`HttpResponse`**: A more flexible structure that lets you define a status code and arbitrary JSON body. It's ideal for cases where you need more granular control over the response using a builder-style pattern.

Both types implement the `IntoResponse` trait, which means they can be used directly as responses in Axum.

