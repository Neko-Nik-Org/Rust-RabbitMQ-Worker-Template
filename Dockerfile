FROM rust:1.92.0-bookworm AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:13-slim

RUN apt-get update

RUN apt-get install -y --no-install-recommends ca-certificates

RUN apt-get clean

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/rust-rmq-worker .

CMD ["/app/rust-rmq-worker"]
