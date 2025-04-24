use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
