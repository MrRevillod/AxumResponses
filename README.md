# Axum Responses

> <img src="https://i.imgur.com/4ysY3bu.png" width=150 align="right"/>

Simplify HTTP responses and error handling in axum based applications.

It uses a builder pattern to create standardized JSON responses, file responses, and derive macro to declare, manage, log, and convert errors into standarized json responses.

<br/>

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.5.3"

# For data serialization and deserialization
serde = { version = "*", features = ["derive"] }
```

## Usage

### The `JsonResponse` Structure

This structure allows you to build responses with a status code, JSON body, and custom headers using a builder pattern.

```rust
use axum_responses::JsonResponse;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
}

async fn handler() -> JsonResponse {
    let user_data = User {
        id: 1,
        username: "example_user".to_string(),
    };

    JsonResponse::Created()
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

### Error Handling with HttpError

Define custom error types that automatically convert to JSON responses:

```rust
use axum_responses::{HttpError, thiserror::Error};

#[derive(Debug, Error, HttpError)]
pub enum ApiError {
    #[error("Not found")]
    #[http(code = 404)]
    NotFound,

    // Log: error_type="BadRequest", status_code=400, reason=?reason
    #[tracing(warn)]
    #[error("Bad request: {message}")]
    #[http(code = 400, error = reason)]
    BadRequest { reason: String },
}
```

Use in handlers:

```rust
use axum::response::IntoResponse;

async fn get_user() -> Result<JsonResponse, ApiError> {
    Err(ApiError::NotFound)
}
```

The `http` attibute converts the error into a `JsonResponse` with the specified status code, and optionally other fields that are defined in the builder of `JsonResponse`. In the same way, if you dont provide a message field, it will use the cannonical message for that status code.

## Examples

You can find complete examples in the examples directory, including advanced usage with tracing, thiserror attributes, and other features.

## Breaking Changes

- From `HttpResponse` to `JsonResponse`: The main response structure has been renamed to better reflect its purpose of handling JSON responses.

- `add_header` Method: The method to add custom headers has been renamed from `add_header` to `header` for improved clarity.
