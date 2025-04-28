use crate::infrastructure::db::DbPool;
use crate::infrastructure::task_repository::TaskRepositoryImpl;
use crate::models::task::Task;
use crate::repositories::task_repository::TaskRepository;
use dotenvy::dotenv;
use sqlx::{Error, PgPool};

pub async fn setup_test_db() -> DbPool {
    // .envファイルを読み込む
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    // 接続プールの作成
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // データベースリセット処理
    reset_test_db(&pool)
        .await
        .expect("Failed to reset test database");

    pool
}

// テスト用にデータベースをリセットする関数
async fn reset_test_db(pool: &DbPool) -> Result<(), Error> {
    // すべてのテーブルのデータを削除する（必要に応じてテーブル名を変更）
    // DELETE文でタスクテーブルをリセット
    sqlx::query!("DELETE FROM tasks").execute(pool).await?;

    Ok(())
}

#[tokio::test]
#[ignore = "Requires DATABASE_URL to be set"]
async fn test_create_and_find_by_id() {
    let pool = setup_test_db().await;
    let repo = TaskRepositoryImpl::new(pool);

    // 新しいTaskを作成
    let title = "テストタスク".to_string();
    let task = Task::new(title.clone());

    // 作成したTaskをデータベースに保存
    let created_task = repo.create(task).await.unwrap();

    // IDで検索
    let found_task = repo.find_by_id(created_task.id).await.unwrap();

    // 検証
    assert!(found_task.is_some());
    let found_task = found_task.unwrap();
    assert_eq!(found_task.id, created_task.id);
    assert_eq!(found_task.title, title);
    assert!(!found_task.completed);

    // 後処理：作成したTaskを削除
    repo.delete(created_task.id).await.unwrap();
}

#[tokio::test]
#[ignore = "Requires DATABASE_URL to be set"]
async fn test_find_all() {
    let pool = setup_test_db().await;
    let repo = TaskRepositoryImpl::new(pool);

    // 新しいTaskを作成
    let title1 = "テストタスク1".to_string();
    let task1 = Task::new(title1.clone());
    let title2 = "テストタスク2".to_string();
    let task2 = Task::new(title2.clone());

    // 作成したTaskをデータベースに保存
    let created_task1 = repo.create(task1).await.unwrap();
    let created_task2 = repo.create(task2).await.unwrap();

    // 全件取得
    let tasks = repo.find_all().await.unwrap();

    // 検証
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, created_task1.id);
    assert_eq!(tasks[0].title, title1);
    assert!(!tasks[0].completed);
    assert_eq!(tasks[1].id, created_task2.id);
    assert_eq!(tasks[1].title, title2);
    assert!(!tasks[1].completed);

    // 後処理：作成したTaskを削除
    repo.delete(created_task1.id).await.unwrap();
    repo.delete(created_task2.id).await.unwrap();
}

#[tokio::test]
#[ignore = "Requires DATABASE_URL to be set"]
async fn test_update() {
    let pool = setup_test_db().await;
    let repo = TaskRepositoryImpl::new(pool);

    // 新しいTaskを作成
    let title = "テストタスク".to_string();
    let task = Task::new(title.clone());

    // 作成したTaskをデータベースに保存
    let created_task = repo.create(task).await.unwrap();

    // Taskを更新
    let update_task = Task {
        id: created_task.id,
        title: "更新後のタスク".to_string(),
        completed: true,
        created_at: created_task.created_at,
        updated_at: created_task.updated_at,
    };
    let updated_task = repo.update(update_task).await.unwrap();

    // 検証
    assert_eq!(updated_task.id, created_task.id);
    assert_eq!(updated_task.title, "更新後のタスク");
    assert!(updated_task.completed);

    // 後処理：作成したTaskを削除
    repo.delete(created_task.id).await.unwrap();
}

#[tokio::test]
#[ignore = "Requires DATABASE_URL to be set"]
async fn test_delete() {
    let pool = setup_test_db().await;
    let repo = TaskRepositoryImpl::new(pool);

    // 新しいTaskを作成
    let title = "テストタスク".to_string();
    let task = Task::new(title.clone());

    // 作成したTaskをデータベースに保存
    let created_task = repo.create(task).await.unwrap();

    // Taskを削除
    repo.delete(created_task.id).await.unwrap();

    // 検証
    let found_task = repo.find_by_id(created_task.id).await.unwrap();
    assert!(found_task.is_none());
}
