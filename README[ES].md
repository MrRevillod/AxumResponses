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
    <strong>Una forma simple de gestionar respuestas HTTP y resultados en Axum</strong>
</div>

## Descripción

**Axum Responses** es una librería diseñada para simplificar la creación y manejo de respuestas HTTP en aplicaciones construidas con [Axum](https://github.com/tokio-rs/axum). Proporciona una abstracción clara para manejar respuestas estándar y personalizadas, junto con herramientas útiles.

## Instalación

Añade la dependencia a tu `Cargo.toml`:

```toml
[dependencies]
axum_responses = "0.4.0"
```

## Características

- **Respuestas Estándar y Personalizadas**: Maneja respuestas HTTP comunes como `200 OK`, `404 Not Found`.
- **Macro Útil**: Usa la macro `response!` para simplificar la creación de respuestas con códigos de estado y cuerpos personalizados.
- **Integración con Axum**: Específicamente diseñado para trabajar con el framework Axum.

## Uso

### La Estructura `HttpResponse`

Esta estructura te permite construir respuestas con un código de estado, cuerpo JSON y encabezados personalizados usando un patrón builder.

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
        .message("Datos de usuario obtenidos exitosamente")
        .data(user_data)
}
```

#### Respuesta Resultante

```json
{
  "code": 201,
  "success": true,
  "message": "Datos de usuario obtenidos exitosamente",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "id": 1,
    "username": "example_user"
  }
}
```

Por otro lado, si respondes con un error HTTP, por ejemplo validación de datos tienes:

```rust
use axum_responses::http::HttpResponse;
use serde_json::json;

async fn error_handler() -> HttpResponse {
    let validation_error = json!({
        "type": "ValidationError",
        "errors": [
            {
                "field": "username",
                "message": "El nombre de usuario es requerido"
            },
            {
                "field": "email",
                "message": "El email debe ser una dirección de email válida"
            }
        ]
    });

    HttpResponse::BadRequest()
        .message("Datos de solicitud inválidos")
        .data(validation_error)
}
```

#### Respuesta Resultante

```json
{
  "code": 400,
  "success": false,
  "message": "Datos de solicitud inválidos",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "type": "ValidationError",
    "errors": [
      {
        "field": "username",
        "message": "El nombre de usuario es requerido"
      },
      {
        "field": "email",
        "message": "El email debe ser una dirección de email válida"
      }
    ]
  }
}
```

### La Macro `response!`

La macro `response!` te permite crear respuestas `HttpResponse` con un código de estado y un cuerpo JSON siendo más flexible. También soporta auto-serialización de estructuras que implementen `Serialize`.

```rust
use axum_responses::{response, http::HttpResponse};

async fn example_handler() -> HttpResponse {
    response!(200, { "page": 10, "total": 100, "message": "Respuesta Exitosa (OK)" })
}
```

#### Respuesta Resultante

```json
{
  "code": 200,
  "success": true,
  "message": "Respuesta Exitosa (OK)",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "page": 10,
    "total": 100
  }
}
```

La macro también soporta objetos únicos en el campo `data`, lo cual es útil para retornar un solo recurso o entidad. Esto está diseñado para ser similar a la notación de javascript.

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
        name: "Producto de Ejemplo".to_string(),
        price: 99.99,
    };

    response!(201, { product_data })
}
```

#### Respuesta Resultante

```json
{
  "code": 201,
  "success": true,
  "message": "Created",
  "timestamp": "2023-10-01T12:00:00Z",
  "data": {
    "id": "prod_123",
    "name": "Producto de Ejemplo",
    "price": 99.99
  }
}
```

## Cambios Importantes

- El enum `Response` ha sido deprecado a favor de la estructura `HttpResponse`.
- El tipo `ControllerResult` ha sido eliminado, y ahora puedes usar `Result<T, HttpResponse>` directamente en tus manejadores, crear tu propio tipo Result personalizado, o simplemente usar `HttpResponse` directamente.

- La librería ahora implementa convenciones RFC para respuestas HTTP.

## Por Hacer

- [ ] Añadir ejemplos para diferentes casos de uso.
- [ ] Añadir soporte para Tower Cookies en una feature flag.
