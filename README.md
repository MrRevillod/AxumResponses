
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
use axum_responses::{AxumResult, Response, AxumResponse};

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    email: String,
    password: String
}

async fn get_user_by_id(filter: doc) -> AxumResult<User> {

    let user = User::find_by_id(filter).await
        .map_err(|_| Response::Standard(500, "Internal Server Error"))? 
    ;

    Ok(user)
}

// And then we can use it in a simple login controller like this

async fn login_controller(Json(body): Json<LoginData>) -> AxumResponse {

    let filter = doc! { "email": body.email };

    let user = get_user_by_id(filter).await?;

    if user.password != body.password {
        return Err(Response::Standard(401, "Unauthorized"))
    }

    Ok(Response::Standard(200, "OK"))
}

```
