use anyhow::Result;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use crate::infrastructure::task_repository::TaskRepositoryImpl;
use crate::usecase::task_usecase::TaskUsecase;

mod app;
mod error;
mod infrastructure;
mod logger;
mod models;
mod repositories;
mod routes;
mod usecase;

#[tokio::main]
async fn main() -> Result<()> {
    // .env読み込み
    dotenv().ok();

    // ロガー初期化（RUST_LOG 環境変数にも対応）
    logger::init();

    //  データベース接続
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // 依存関係のセットアップ
    let task_repository = TaskRepositoryImpl::new(pool.clone());
    let task_service = TaskUsecase::new(task_repository);

    // アプリ初期化
    let app = app::create_app(task_service.clone());
    // アドレス指定 & ログ出力
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚀 Server listening on http://{}", addr);

    // サーバ起動
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
