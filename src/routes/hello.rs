use axum::{routing::get, Router};

pub fn router() -> Router {
  Router::new().route("/hello", get(hello_handler))
}
pub async fn hello_handler() -> &'static str {
    "Hello Rust World"
}
