# マルチステージビルドを使用し、Rustのプログラムをビルドする。
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 不要なソフトウェアを同梱する必要はないので、軽量な bookworm-slim を使用する。
FROM debian:bookworm-slim
WORKDIR /app

# 後続の説明で使用するため、ユーザを作成しておく。
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

# 8080番ポートを解放し、アプリケーションを起動する。。
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]
