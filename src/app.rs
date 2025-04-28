use crate::routes;
use crate::usecase::task_usecase::TaskService;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::api_doc::ApiDoc;

pub fn create_app<T: TaskService + Send + Sync + 'static + Clone>(task_service: T) -> Router {
    Router::new()
        .merge(routes::hello::router())
        .merge(routes::users::router())
        .merge(routes::tasks::router(task_service))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
