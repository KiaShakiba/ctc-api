FROM rust:1.95.0-slim-bookworm AS builder

WORKDIR /usr/src/app

RUN apt-get update -y && apt-get install -y pkg-config libssl-dev libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/app

RUN apt-get update -y && apt-get install -y pkg-config libssl-dev libpq-dev curl

COPY --from=builder /usr/src/app/target/release/ctc-api ./

ENTRYPOINT ["/usr/src/app/ctc-api"]
