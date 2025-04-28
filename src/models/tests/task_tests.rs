use crate::models::task::Task;
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let title = "テストタイトル";
        
        let task = Task::new(title.to_string());
        
        // タイトルが正しく設定されていることを確認
        assert_eq!(task.title, title);
        
        // 初期状態では完了していないことを確認
        assert!(!task.completed);
        
        // created_atとupdated_atが同じであることを確認
        assert_eq!(task.created_at, task.updated_at);
        
        // UUIDがバージョン7であることを確認
        assert_eq!(task.id.get_version_num(), 7);
        
        // 現在時刻との差が小さいことを確認（1秒以内）
        let now = Utc::now();
        let diff = now.signed_duration_since(task.created_at);
        assert!(diff.num_seconds().abs() < 1);
    }
}
