# Rust の公式イメージ（最新バージョン）をベースとして使用
FROM rust:latest as builder

# コンテナ内の作業ディレクトリを `/app` に設定
WORKDIR /app

# システムパッケージの更新と make のインストール
RUN apt-get update && \
  apt-get install -y --no-install-recommends make && \
  rm -rf /var/lib/apt/lists/*

# Rust のツールをインストール
RUN rustup update && \
    rustup component add rustfmt clippy rust-src rust-analyzer

# `cargo-watch` をインストールし、ファイル変更を監視して自動ビルド・実行を可能にする
RUN cargo install cargo-watch

# cargo test に存在するボトルネックを解消する `cargo-nextest` をインストールする
RUN cargo install cargo-nextest

# 複数のコマンドや依存関係を整理したりするために タスクランナーを導入
RUN cargo install --force cargo-make

# プロジェクトのソースコードをコンテナ内にコピー
COPY Cargo.toml Cargo.lock /app/
RUN cargo fetch
COPY . /app/

# プロジェクトのビルド
RUN cargo build --release --verbose

# デフォルトのコマンドを設定
# コンテナ起動後は無限にスリープし、コンテナを終了させない（開発時の利便性向上）
CMD ["sleep", "infinity"]

