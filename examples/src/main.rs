mod errors;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;
use tokio::net::TcpListener;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/limited", get(rate_limited))
        .route("/io", get(io_error))
        .route("/missing_field", get(missing_field))
        .route("/custom_message", get(custom_message));

    axum::serve(TcpListener::bind("0.0.0.0:9000").await.unwrap(), app)
        .await
        .unwrap();
}
