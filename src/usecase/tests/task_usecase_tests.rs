use crate::models::task::Task;
use crate::repositories::task_repository::MockTaskRepository;
use crate::usecase::task_usecase::{TaskService, TaskUsecase};
use chrono::{FixedOffset, TimeZone, Utc};
use mockall::predicate::*;
use uuid::Uuid;

// テスト用のTodoを作成するヘルパー関数
fn create_test_task(title: &str) -> Task {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
    let now_utc = now_jst.with_timezone(&Utc);

    Task {
        id: Uuid::now_v7(),
        title: title.to_string(),
        completed: false,
        created_at: now_utc,
        updated_at: now_utc,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_tasks() {
        // モックリポジトリの作成
        let mut mock_repo = MockTaskRepository::new();

        // テスト用のTaskリスト
        let tasks = vec![create_test_task("タスク1"), create_test_task("タスク2")];

        // find_allメソッドのモック設定
        mock_repo
            .expect_find_all()
            .times(1)
            .returning(move || Ok(tasks.clone()));

        // ユースケースの作成
        let usecase = TaskUsecase::new(mock_repo);

        // テスト実行
        let result = usecase.get_all_tasks().await.unwrap();

        // 検証
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "タスク1");
        assert_eq!(result[1].title, "タスク2");
    }

    #[tokio::test]
    async fn test_get_task_by_id() {
        // モックリポジトリの作成
        let mut mock_repo = MockTaskRepository::new();

        // テスト用のTask
        let task = create_test_task("タスク1");
        let task_id = task.id;

        // find_by_idメソッドのモック設定
        mock_repo
            .expect_find_by_id()
            .with(eq(task_id))
            .times(1)
            .returning(move |_| Ok(Some(task.clone())));

        // ユースケースの作成
        let usecase = TaskUsecase::new(mock_repo);

        // テスト実行
        let result = usecase.get_task_by_id(task_id).await.unwrap();

        // 検証
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.title, "タスク1");
    }

    #[tokio::test]
    async fn test_create_task() {
        // モックリポジトリの作成
        let mut mock_repo = MockTaskRepository::new();

        // テスト用のTask
        let task = create_test_task("タスク1");
        let expected_title = task.title.clone();

        // createメソッドのモック設定
        mock_repo
            .expect_create()
            .withf(move |t| t.title == expected_title)
            .times(1)
            .returning(move |_| Ok(task.clone()));

        // ユースケースの作成
        let usecase = TaskUsecase::new(mock_repo);

        // テスト実行
        let result = usecase.create_task("タスク1".to_string()).await.unwrap();

        // 検証
        assert_eq!(result.title, "タスク1");
        assert!(!result.completed);
        assert!(!result.id.is_nil());
    }

    #[tokio::test]
    async fn test_update_task() {
        // モックリポジトリの作成
        let mut mock_repo = MockTaskRepository::new();

        // テスト用のTask
        let task = create_test_task("タスク1");
        let task_id = task.id;

        // find_by_id メソッドのモック設定
        mock_repo
            .expect_find_by_id()
            .with(eq(task_id))
            .times(1)
            .returning(move |_| Ok(Some(task.clone())));

        // updateメソッドのモック設定
        mock_repo
            .expect_update()
            .withf(move |t| t.id == task_id)
            .times(1)
            .returning(move |updated_task| Ok(updated_task.clone()));

        // ユースケースの作成
        let usecase = TaskUsecase::new(mock_repo);

        // テスト実行
        let result = usecase
            .update_task(task_id, None, Some(true))
            .await
            .unwrap();

        // 検証
        assert_eq!(result.title, "タスク1");
        assert!(result.completed);
        assert!(!result.id.is_nil());
    }

    #[tokio::test]
    async fn test_delete_task() {
        // モックリポジトリの作成
        let mut mock_repo = MockTaskRepository::new();

        // テスト用のTask
        let task = create_test_task("タスク1");
        let task_id = task.id;

        // deleteメソッドのモック設定
        mock_repo
            .expect_delete()
            .with(eq(task_id))
            .times(1)
            .returning(|_| Ok(()));

        // ユースケースの作成
        let usecase = TaskUsecase::new(mock_repo);

        // テスト実行
        usecase.delete_task(task_id).await.unwrap();
    }
}
