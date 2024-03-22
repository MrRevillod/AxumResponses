
<div align="center">
    <img src="./images/ferris.png" width=250 />
</div>

<div align="center">
    <h1>Axum Responses</h1>
</div>

<div align="center">
    <strong>A Simple way to use Axum responses and results</strong>
</div>

## Example

```rust 

// Asume we have a mongodb database connection and a user model
// A simple service that returns a generic or an error

use bson::
use axum::Json;
use serde_json::{Value, to_value};
use axum_responses::{AxumResult, HttpResponse, AxumResponse};

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    email: String,
    password: String
}

async fn get_user_by_id(filter: doc) -> AxumResult<User> {

    let user = User::find_by_id(filter).await
        .map_err(|_| HttpResponse::INTERNAL_SERVER_ERROR)?
    ;

    Ok(user)
}

// And then we can use it in a simple login controller like this

async fn login_controller(Json(body): Json<LoginData>) -> AxumResponse {

    let filter = doc! { "email": body.email };
    let user = get_user_by_id(filter).await?;

    if user.password != body.password {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    Ok(HttpResponse::JSON(
        200, "OK", "user", user.to_value().unwrap_or(Value::Null))
    )
}

```
