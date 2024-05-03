###############################################################################
# for local
###############################################################################
FROM rust:1.77.2-slim as local
RUN apt-get update && apt-get install -y \
  libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

###############################################################################
# for release
# 以下参考に軽量ビルドを実行
# - https://zenn.dev/kyoheiu/articles/dcefe0c75f0e17
# - https://blog.rust-jp.rs/tatsuya6502/posts/2019-12-small-docker-image/
# - https://zenn.dev/kasega0/articles/18c225bae23a00
# - https://blog.bouzuya.net/2023/01/10/
###############################################################################
FROM rust:1.77.2-slim as release-builder
WORKDIR /home/rust
COPY . .
RUN apt-get update && apt-get install -y \
  musl-dev \
  libpq-dev

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch as release-entry
WORKDIR /webapi_mvp
COPY --from=release-builder /home/rust/target/x86_64-unknown-linux-musl/release/webapi_mvp . 
EXPOSE 8080
ENTRYPOINT [ "./webapi_mvp" ]
