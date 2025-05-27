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
    <strong>Una forma sencilla de manejar respuestas y resultados en Axum</strong>
</div>

---

## Descripción

**Axum Responses** es una biblioteca diseñada para simplificar la creación y manejo de respuestas HTTP en aplicaciones construidas con [Axum](https://github.com/tokio-rs/axum). Proporciona una abstracción clara para manejar respuestas estándar y personalizadas, junto con herramientas útiles como macros para reducir la repetición de código.

---

## Instalación

Agrega la dependencia en tu archivo `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.3.1"
```

Asegúrate de incluir también las dependencias necesarias como `axum`, `serde` y `serde_json`.

---

## Características

- **Respuestas estándar y personalizadas**: Maneja respuestas HTTP comunes como `200 OK`, `404 Not Found`.
- **Macros útiles**: Usa el macro `response!` para simplificar la creación de respuestas con códigos de estado y cuerpos personalizados.
- **Integración con Axum**: Diseñado específicamente para trabajar con el framework Axum.
---

## Ejemplo de Uso

### Respuestas estándar y personalizadas

La enum `Response` incluye variantes para los códigos de estado HTTP más comunes, como `Response::OK`, `Response::NOT_FOUND`, y más. También puedes crear respuestas personalizadas con `Response::CUSTOM`.

```rust
use axum::{Router, routing::get};
use axum_responses::standard::Response;
use serde_json::json;

async fn handler() -> Response {
    Response::OK
}

async fn custom_handler() -> Response {
    Response::CUSTOM(201, json!({ "message": "Creado exitosamente" }))
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/custom", get(custom_handler))
}
```

### Usando `HttpResponse` para respuestas más flexibles

La estructura `HttpResponse` te permite construir respuestas con un código de estado, cuerpo JSON y cabeceras personalizadas utilizando un patrón builder.

```rust
use axum_responses::http::HttpResponse;
use axum::http::StatusCode;
use serde_json::json;
use serde::Serialize;

// Handler simple con HttpResponse
async fn http_response_handler() -> HttpResponse {
    HttpResponse::build()
        .status(StatusCode::OK)
        .add_header("X-Custom-Header", "valor_custom")
        .body(json!({ "message": "Hola desde HttpResponse" }))
}

// Handler usando el método `json()` para serializar structs
#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
}

async fn http_response_json_struct_handler() -> HttpResponse {
    let user_data = User {
        id: 1,
        username: "usuario_ejemplo".to_string(),
    };
    HttpResponse::build()
        .status(StatusCode::OK)
        .json(user_data) // Serializa automáticamente User a JSON
}

// Ejemplo en una aplicación Axum
use axum::{Router, routing::get};

fn app_with_http_response() -> Router {
    Router::new()
        .route("/http", get(http_response_handler))
        .route("/user", get(http_response_json_struct_handler))
}
```

### Macro `response!`

El macro `response!` permite crear respuestas `HttpResponse` con un código de estado y un cuerpo JSON de manera más concisa. También soporta la auto-serialización de structs que implementan `Serialize`.

```rust
use axum_responses::{response, http::HttpResponse};
use serde::Serialize;

async fn example_handler() -> HttpResponse {
    response!(200, { "status": "success", "data": "Ejemplo" })
}

async fn error_handler() -> HttpResponse {
    response!(404, { "error": "Recurso no encontrado" })
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
        name: "Producto Ejemplo".to_string(),
        price: 99.99,
    };
    // La struct product_data se serializará automáticamente a JSON
    response!(201, { product_data })
}

async fn another_error_handler() -> HttpResponse {
    response!(500, { "error": "Error interno del servidor" })
}

// Ejemplo en una aplicación Axum
use axum::{Router, routing::get};

fn app_with_macro() -> Router {
    Router::new()
        .route("/macro-example", get(example_handler))
        .route("/macro-product", get(product_handler))
        .route("/macro-error", get(error_handler))
}

```

### Diferencias entre `Response` y `HttpResponse`

- **`Response`**: Es una enumeración diseñada para manejar respuestas estándar y personalizadas de manera sencilla. Es útil para controladores que necesitan devolver respuestas predefinidas.

- **`HttpResponse`**: Es una estructura más flexible que permite definir un código de estado y un cuerpo JSON arbitrarios. Es ideal para casos donde necesitas un control más granular sobre la respuesta con un patrón de diseño builder.

Ambos tipos implementan el trait `IntoResponse`, lo que significa que pueden ser utilizados directamente como respuestas en Axum.

---
