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
axum_responses = "0.2.3"
```

Asegúrate de incluir también las dependencias necesarias como `axum`, `serde` y `serde_json`.

---

## Características

- **Respuestas estándar y personalizadas**: Maneja respuestas HTTP comunes como `200 OK`, `404 Not Found`.
- **Macros útiles**: Usa el macro `response!` para simplificar la creación de respuestas con códigos de estado y cuerpos personalizados.
- **Integración con Axum**: Diseñado específicamente para trabajar con el framework Axum.
- **Manejo de errores**: Facilita la propagación de errores HTTP en tus controladores mediante tipos como `ControllerResult` o `HandlerResult`.

---

## Ejemplo de Uso

### Respuestas estándar y personalizadas

La enum `Response` incluye variantes para los códigos de estado HTTP más comunes, como `Response::OK`, `Response::NOT_FOUND`, y más. También puedes crear respuestas personalizadas con `Response::CUSTOM`.

```rust
use axum::{Router, routing::get};
use axum_responses::Response;
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

### Macro `response!`

El macro `response!` permite crear respuestas con un código de estado y un cuerpo JSON de manera más concisa. Este macro retorna un `Result`, donde las respuestas con códigos de estado exitosos (`200..399`) se envuelven en `Ok`, y las demás en `Err`.

```rust
use axum_responses::{response, HttpResponse, ControllerResult, HandlerResult};

async fn example_handler() -> Result<HttpResponse, HttpResponse> {
    response!(200, { "status": "success", "data": "Ejemplo" })
}

async fn error_handler() -> ControllerResult {
    response!(404, { "error": "Recurso no encontrado" })
}

async fn another_error_handler() -> HandlerResult {
    response!(500, { "error": "Error interno del servidor" })
}

```

### Diferencias entre `Response` y `HttpResponse`

- **`Response`**: Es una enumeración diseñada para manejar respuestas estándar y personalizadas de manera sencilla. Es útil para controladores que necesitan devolver respuestas predefinidas.
- **`HttpResponse`**: Es una estructura más flexible que permite definir un código de estado y un cuerpo JSON arbitrarios. Es ideal para casos donde necesitas un control más granular sobre la respuesta.

Ambos tipos implementan el trait `IntoResponse`, lo que significa que pueden ser utilizados directamente como respuestas en Axum.

---

## Consideraciones

1. **Compatibilidad entre `Response` y `HttpResponse`**: Aunque ambos implementan `IntoResponse`, no son intercambiables directamente. Usa `Response` para respuestas estándar y `HttpResponse` para personalizadas.
2. **Uso del macro `response!`**: Este macro retorna un `Result`, lo que puede ser útil para manejar errores en controladores. Sin embargo, ten en cuenta que esto puede no ser una práctica común en Axum, ya que normalmente los controladores devuelven un tipo que implementa `IntoResponse` directamente. Entonces deberás utilizar siempre un Result en el controlador. Para esto se incorporaron los tipos `ControllerResult` y `HandlerResult`, ambos siendo un alias para `Result<HttpResponse, HttpResponse>`.
3. **Errores en macros**: Si usas un código de estado inválido en el macro `response!`, este generará un `panic!`. Asegúrate de usar códigos válidos.
