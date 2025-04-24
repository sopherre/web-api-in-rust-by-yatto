use crate::models::task::{CreateTask, Task, UpdateTask};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

type TaskList = Arc<Mutex<Vec<Task>>>;

pub fn router(task_list: TaskList) -> Router {
    Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route(
            "/tasks/:id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .with_state(task_list)
}

// 全件取得
async fn get_tasks(state: State<TaskList>) -> impl IntoResponse {
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}

// 単一取得
async fn get_task(state: State<TaskList>, Path(id): Path<u32>) -> impl IntoResponse {
    let tasks = state.lock().unwrap();
    match tasks.iter().find(|t| t.id == id) {
        Some(task) => Json(task.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, "Task not found").into_response(),
    }
}

// 作成
async fn create_task(state: State<TaskList>, Json(task): Json<CreateTask>) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();
    let new_task: Task = Task {
        id: tasks.len() as u32 + 1,
        title: task.title,
        completed: false,
    };
    tasks.push(new_task.clone());
    (StatusCode::CREATED, Json(new_task)).into_response()
}

// 更新
async fn update_task(
    state: State<TaskList>,
    Path(id): Path<u32>,
    Json(updated): Json<UpdateTask>,
) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            if let Some(title) = updated.title {
                task.title = title;
            }
            if let Some(completed) = updated.completed {
                task.completed = completed;
            }
            Json(task.clone()).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Task not found").into_response(),
    }
}

// 削除
async fn delete_task(state: State<TaskList>, Path(id): Path<u32>) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();
    let len_before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == len_before {
        (StatusCode::NOT_FOUND, "Task not found").into_response()
    } else {
        (StatusCode::NO_CONTENT, "").into_response()
    }
}
