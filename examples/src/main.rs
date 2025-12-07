mod errors;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("Starting server");

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
