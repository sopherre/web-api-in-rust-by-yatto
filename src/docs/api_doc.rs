use crate::models::task::Task;
use crate::routes::tasks;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        tasks::get_tasks,
        tasks::get_task,
        tasks::create_task,
        tasks::update_task,
        tasks::delete_task,
    ),
    components(
        schemas(Task),
        schemas(tasks::CreateTaskRequest),
        schemas(tasks::UpdateTaskRequest),
        schemas(tasks::TaskResponse),
    ),
    tags(
        (name = "Tasks", description = "タスク管理API")
    )
)]
pub struct ApiDoc;
