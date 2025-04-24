use crate::routes;
use axum::Router;
use std::sync::{Arc, Mutex};

pub fn create_app() -> Router {
    let task_list = Arc::new(Mutex::new(Vec::new()));
    Router::new()
        .merge(routes::hello::router())
        .merge(routes::users::router())
        .merge(routes::tasks::router(task_list.clone()))
}
