FROM rust:latest

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock /app/

RUN rustup component add rustfmt

RUN cargo fetch || true

COPY ./src /app/src

RUN cargo build --release || true

CMD ["sleep", "infinity"]