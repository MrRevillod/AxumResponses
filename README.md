<div align="center">
    <img src="https://raw.githubusercontent.com/MrRevillod/AxumResponses/refs/heads/master/images/logo.png" width=250 />
</div>

<div align="center">
    <h1>Axum Responses</h1>
</div>

<div align="center">
  <a href="https://github.com/MrRevillod/AxumResponses/blob/master/README.md" title="English README">🇺🇸 English</a>
  &nbsp;&nbsp;|&nbsp;&nbsp;
  <a href="https://github.com/MrRevillod/AxumResponses/blob/master/README%5BES%5D.md" title="README en Español">🇪🇸 Español</a>
</div>

<div align="center">
    <strong>A simple way to handle responses and results in Axum</strong>
</div>

---

## Description

**Axum Responses** is a library designed to simplify the creation and handling of HTTP responses in applications built with [Axum](https://github.com/tokio-rs/axum). It provides a clear abstraction to handle standard and custom responses, along with useful tools like macros to reduce code repetition.

---

## Installation

Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
axum_responses = "0.2.2"
```

Make sure to also include the necessary dependencies like `axum`, `serde`, and `serde_json`.

---

## Features

- **Standard and custom responses**: Handle common HTTP responses like `200 OK`, `404 Not Found`.
- **Useful macros**: Use the `response!` macro to simplify the creation of responses with status codes and custom bodies.
- **Integration with Axum**: Specifically designed to work with the Axum framework.
- **Error handling**: Facilitates the propagation of HTTP errors in your controllers using types like `ControllerResult` or `HandlerResult`.

---

## Usage Example

### Standard and Custom Responses

The `Response` enum includes variants for the most common HTTP status codes, such as `Response::OK`, `Response::NOT_FOUND`, and more. You can also create custom responses with `Response::CUSTOM`.

```rust
use axum::{Router, routing::get};
use axum_responses::Response;
use serde_json::json;

async fn handler() -> Response {
    Response::OK
}

async fn custom_handler() -> Response {
    Response::CUSTOM(201, json!({ "message": "Successfully created" }))
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/custom", get(custom_handler))
}
```

### Macro `response!`

The `response!` macro allows you to create responses with a status code and a JSON body more concisely. This macro returns a `Result`, where responses with successful status codes (`200..399`) are wrapped in `Ok`, and others in `Err`.

```rust
use axum_responses::{response, HttpResponse, ControllerResult, HandlerResult};

async fn example_handler() -> Result<HttpResponse, HttpResponse> {
    response!(200, { "status": "success", "data": "Example" })
}

async fn error_handler() -> ControllerResult {
    response!(404, { "error": "Resource not found" })
}

async fn another_error_handler() -> HandlerResult {
    response!(500, { "error": "Internal server error" })
}
```

### Differences Between `Response` and `HttpResponse`

- **`Response`**: An enumeration designed to handle standard and custom responses easily. It is useful for controllers that need to return predefined responses.
- **`HttpResponse`**: A more flexible structure that allows you to define an arbitrary status code and JSON body. It is ideal for cases where you need more granular control over the response.

Both types implement the `IntoResponse` trait, meaning they can be used directly as responses in Axum.

---

## Considerations

1. **Compatibility Between `Response` and `HttpResponse`**: Although both implement `IntoResponse`, they are not directly interchangeable. Use `Response` for standard responses and `HttpResponse` for custom ones.
2. **Using the `response!` Macro**: This macro returns a `Result`, which can be useful for handling errors in controllers. However, note that this may not be a common practice in Axum, as controllers typically return a type that directly implements `IntoResponse`. Therefore, you should always use a `Result` in the controller. For this, the `ControllerResult` and `HandlerResult` types were introduced, both being aliases for `Result<HttpResponse, HttpResponse>`.
3. **Macro Errors**: If you use an invalid status code in the `response!` macro, it will generate a `panic!`. Make sure to use valid codes.
