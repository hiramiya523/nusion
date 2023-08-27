use std::net::SocketAddr;

use axum::response::Html;
use axum::{Router, routing::{get}};

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", get(hello_world));

  axum::Server::bind(&"0.0.0.0:8083".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn hello_world() -> String {
  "Hello".to_owned()
}