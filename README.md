<div align="center">
    <img src="https://pillan.inf.uct.cl/~lrevillod/images/logo-ax-responses.png" width=250 />
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

## Description

**Axum Responses** is a library designed to simplify the creation and handling of HTTP responses in applications built with [Axum](https://github.com/tokio-rs/axum). It provides a clear abstraction for handling standard and custom responses, along with useful tools.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.4.1"
```

## Features

- **Standard and Custom Responses**: Handle common HTTP responses like `200 OK`, `404 Not Found`.
- **Useful Macro**: Use the `response!` macro to simplify creating responses with custom status codes and bodies.
- **Integration with Axum**: Specifically designed to work with the Axum framework.
- **RFC Conventions**: Follows RFC conventions for HTTP responses, ensuring consistency and clarity in your API responses.

- **Version 0.4.1**: Improve the `HttpResponse` memory size removing unnecessary fields and optimizing the structure for better performance.

## Usage

### The `HttpResponse` Structure

This structure allows you to build responses with a status code, JSON body, and custom headers using a builder pattern.

```rust
use axum_responses::http::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
}

async fn handler() -> HttpResponse {
    let user_data = User {
        id: 1,
        username: "example_user".to_string(),
    };

    HttpResponse::Created()
        .message("User data retrieved successfully")
        .data(user_data)
}
```

#### Resulting Response

```json
{
  "code": 201,
  "success": true,
  "message": "User data retrieved successfully",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "id": 1,
    "username": "example_user"
  }
}
```

Otherwise if you response with an http error, for example data validation you have:

```rust
use axum_responses::http::HttpResponse;
use serde_json::json;

async fn error_handler() -> HttpResponse {
    let validation_error = json!({
        "type": "ValidationError",
        "errors": [
            {
                "field": "username",
                "message": "Username is required"
            },
            {
                "field": "email",
                "message": "Email must be a valid email address"
            }
        ]
    });

    HttpResponse::BadRequest()
        .message("Invalid request data")
        .data(validation_error)
}
```

#### Resulting Response

```json
{
  "code": 400,
  "success": false,
  "message": "Invalid request data",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "type": "ValidationError",
    "errors": [
      {
        "field": "username",
        "message": "Username is required"
      },
      {
        "field": "email",
        "message": "Email must be a valid email address"
      }
    ]
  }
}
```

### The `response!` Macro

The `response!` macro allows you to create `HttpResponse` responses with a status code and a JSON body being more lax. It also supports auto-serialization of structs that implement `Serialize`.

```rust
use axum_responses::{response, http::HttpResponse};

async fn example_handler() -> HttpResponse {
    response!(200, { "page": 10, "total": 100, "message": "Success Response (OK)" })
}
```

#### Resulting Response

```json
{
  "code": 200,
  "success": true,
  "message": "Success Response (OK)",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "page": 10,
    "total": 100
  }
}
```

The macro also supports single objects in the `data` field, which is useful for returning a single resource or entity. This is designed to be similar to javascript notation.

```rust
use axum_responses::{response, http::HttpResponse};
use serde::Serialize;

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

    response!(201, { product_data })
}
```

#### Resulting Response

```json
{
  "code": 201,
  "success": true,
  "message": "Created",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "id": "prod_123",
    "name": "Example Product",
    "price": 99.99
  }
}
```

## Breaking Changes

- The `Response` enum has been deprecated in favor of the `HttpResponse` structure.
- The `ControllerResult` type has been removed, and now you can use `Result<T, HttpResponse>` directly in your handlers, create your own custom Result type, or just use `HttpResponse` directly.

- The library now implements RFC conventions for HTTP responses.

## To Do

- [ ] Add examples for different use cases.
- [ ] Add support for Tower Cookies on a feature flag.
