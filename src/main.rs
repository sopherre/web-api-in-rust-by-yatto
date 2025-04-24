use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod app;
mod error;
mod logger;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // ロガー初期化（RUST_LOG 環境変数にも対応）
    logger::init();

    // アプリ初期化
    let app = app::create_app();
    // アドレス指定 & ログ出力
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚀 Server listening on http://{}", addr);

    // サーバ起動
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
