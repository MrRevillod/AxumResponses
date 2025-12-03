mod errors;
mod handlers;

use axum::Router;
use axum::routing::get;
use axum_responses::JsonResponseBody;
use axum_test::TestServer;
use errors::*;
use handlers::*;

type AppResult<T> = Result<T, AppError>;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/file", get(handler));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/file").await.json::<JsonResponseBody>();

    dbg!(response);
}
