<div align="center">
    <img src="https://pillan.inf.uct.cl/~lrevillod/images/logo-ax-responses.png" width=250 />
</div>

<div align="center">
    <h1>Axum Responses</h1>
</div>

<div align="center">
    <strong>A better way to manage responses and errors in Axum</strong>
</div>

## Description

**Axum Responses** is a crate designed to simplify the creation and handling of HTTP responses in applications built with [Axum](https://github.com/tokio-rs/axum). It provides abstractions for handling standardized JSON responses, file responses, and error management, making it easier to build robust web applications.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.5.0"
```

## Features

- **Standarized JSON Responses**: Easily create standardized JSON responses for common HTTP status codes like `200 OK`, `201 Created`, `400 Bad Request`, `404 Not Found`, and more.

- **Friendly File Response Creation**: Use a builder pattern to create file responses and downloads with ease.

- **Error Handling**: Define and manage HTTP errors with custom error types using the `HttpError` derive macro and convert them directly into JSON responses. All of this being compatible with `thiserror`.

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

## Examples

You can find complete examples in the examples directory.

## Breaking Changes

- From `HttpResponse` to `JsonResponse`: The main response structure has been renamed to better reflect its purpose of handling JSON responses.

- `add_header` Method: The method to add custom headers has been renamed from `add_header` to `header` for improved clarity.
