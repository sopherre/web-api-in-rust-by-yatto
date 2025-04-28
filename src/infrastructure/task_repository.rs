use crate::infrastructure::db::DbPool;
use crate::models::task::Task;
use crate::repositories::task_repository::TaskRepository;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskRepositoryImpl {
    pub pool: DbPool,
}

impl TaskRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, title, completed, created_at, updated_at FROM tasks",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(tasks)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, title, completed, created_at, updated_at FROM tasks WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(task)
    }

    async fn create(&self, task: Task) -> Result<Task, sqlx::Error> {
        let created_task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (id, title, completed, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, title, completed, created_at, updated_at",
        )
        .bind(task.id)
        .bind(&task.title)
        .bind(task.completed)
        .bind(task.created_at)
        .bind(task.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(created_task)
    }

    async fn update(&self, task: Task) -> Result<Task, sqlx::Error> {
        let updated_task = sqlx::query_as::<_, Task>(
            "UPDATE tasks SET title = $1, completed = $2, updated_at = (NOW() AT TIME ZONE 'Asia/Tokyo')
             WHERE id = $3
             RETURNING id, title, completed, created_at, updated_at"
        )
        .bind(&task.title)
        .bind(task.completed)
        .bind(task.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(updated_task)
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
