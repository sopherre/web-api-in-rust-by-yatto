use crate::models::task::Task;
use crate::repositories::task_repository::TaskRepository;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskUsecase<T: TaskRepository + Clone> {
    repository: T,
}

impl<T: TaskRepository + Clone> TaskUsecase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait TaskService {
    async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error>;
    async fn get_task_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error>;
    async fn create_task(&self, title: String) -> Result<Task, sqlx::Error>;
    async fn update_task(
        &self,
        id: Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Task, sqlx::Error>;
    async fn delete_task(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl<T: TaskRepository + Send + Sync + Clone> TaskService for TaskUsecase<T> {
    async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        self.repository.find_all().await
    }

    async fn get_task_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    async fn create_task(&self, title: String) -> Result<Task, sqlx::Error> {
        let new_task = Task::new(title);
        self.repository.create(new_task).await
    }

    async fn update_task(
        &self,
        id: Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Task, sqlx::Error> {
        let existing_task = self.repository.find_by_id(id).await?;
        if let Some(mut task) = existing_task {
            if let Some(t) = title {
                task.title = t;
            }
            if let Some(c) = completed {
                task.completed = c;
            }
            return self.repository.update(task).await;
        }
        Err(sqlx::Error::RowNotFound)
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}
