use crate::repositories::task_repository::{MockTaskRepository, TaskRepository};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_repository() {
        let mut mock_repo = MockTaskRepository::new();

        // find_allメソッドが空のベクターを返すように設定
        mock_repo
            .expect_find_all()
            .times(1)
            .returning(|| Ok(vec![]));

        // テスト実行
        let result = mock_repo.find_all().await.unwrap();

        // 空のベクターが返されることを確認
        assert_eq!(result.len(), 0);
    }
}
