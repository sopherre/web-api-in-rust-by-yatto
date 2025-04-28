use crate::models::task::Task;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::usecase::task_usecase::TaskService;

#[derive(Clone)]
pub struct AppState<T: TaskService> {
    pub task_service: Arc<T>,
}

pub fn router<T: TaskService + Send + Sync + 'static + Clone>(task_service: T) -> Router {
    let state = AppState {
        task_service: Arc::new(task_service),
    };
    Router::new()
        .route("/tasks", get(get_tasks::<T>).post(create_task::<T>))
        .route(
            "/tasks/:id",
            get(get_task::<T>)
                .put(update_task::<T>)
                .delete(delete_task::<T>),
        )
        .with_state(state)
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTaskRequest {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Serialize)]
struct TaskResponse {
    id: Uuid,
    title: String,
    completed: bool,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            completed: task.completed,
        }
    }
}

// 全件取得
async fn get_tasks<T: TaskService>(State(state): State<AppState<T>>) -> impl IntoResponse {
    match state.task_service.get_all_tasks().await {
        Ok(tasks) => Json(
            tasks
                .into_iter()
                .map(TaskResponse::from)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todos").into_response(),
    }
}

// 単一取得
async fn get_task<T: TaskService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.task_service.get_task_by_id(id).await {
        Ok(Some(task)) => Json(TaskResponse::from(task)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todo").into_response(),
    }
}

// 作成
async fn create_task<T: TaskService>(
    State(state): State<AppState<T>>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    match state.task_service.create_task(payload.title).await {
        Ok(task) => (StatusCode::CREATED, Json(TaskResponse::from(task))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create todo").into_response(),
    }
}

// 更新
async fn update_task<T: TaskService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    match state
        .task_service
        .update_task(id, payload.title, payload.completed)
        .await
    {
        Ok(task) => Json(TaskResponse::from(task)).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update todo").into_response(),
    }
}

// 削除
async fn delete_task<T: TaskService>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.task_service.delete_task(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete todo").into_response(),
    }
}
