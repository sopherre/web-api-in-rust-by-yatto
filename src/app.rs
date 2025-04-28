use crate::routes;
use crate::usecase::task_usecase::TaskService;
use axum::Router;

pub fn create_app<T: TaskService + Send + Sync + 'static + Clone>(task_service: T) -> Router {
    Router::new()
        .merge(routes::hello::router())
        .merge(routes::users::router())
        .merge(routes::tasks::router(task_service))
}
