use crate::models::task::Task;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error>;
    async fn create(&self, task: Task) -> Result<Task, sqlx::Error>;
    async fn update(&self, task: Task) -> Result<Task, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

// 非同期トレイトをモックするために mock! を使う
mock! {
    pub TaskRepository {}

    #[async_trait]
    impl TaskRepository for TaskRepository {
        async fn find_all(&self) -> Result<Vec<Task>, sqlx::Error>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error>;
        async fn create(&self, task: Task) -> Result<Task, sqlx::Error>;
        async fn update(&self, task: Task) -> Result<Task, sqlx::Error>;
        async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    }
}

// MockTaskRepository に Clone を追加する
impl Clone for MockTaskRepository {
    fn clone(&self) -> Self {
        MockTaskRepository::new()
    }
}
