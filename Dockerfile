# Rust の公式イメージ（最新バージョン）をベースとして使用
FROM rust:latest

# コンテナ内の作業ディレクトリを `/app` に設定
WORKDIR /app

# システムパッケージの更新と make のインストール
RUN apt-get update && apt-get install -y --no-install-recommends \
  make \  
  && rm -rf /var/lib/apt/lists/*  # キャッシュ削除によりイメージサイズを最小化

# Rust のフォーマットツール `rustfmt` をインストール
RUN rustup component add rustfmt

# `cargo-watch` をインストールし、ファイル変更を監視して自動ビルド・実行を可能にする
RUN cargo install cargo-watch

# Rust の依存関係を事前にフェッチ（キャッシュ効率を向上）
# このステップで依存関係を分離することで、コードの変更によるビルドの影響を軽減
COPY ./Cargo.toml ./Cargo.lock /app/
RUN cargo fetch || true  # ネットワーク問題で失敗しても続行

# プロジェクトのソースコードをコンテナ内にコピー
COPY ./src /app/src

# プロジェクトのビルド（エラーが発生しても続行）
RUN cargo build --release || true

# デフォルトのコマンドを設定
# コンテナ起動後は無限にスリープし、コンテナを終了させない（開発時の利便性向上）
CMD ["sleep", "infinity"]