# backend_rs — Rust (Axum + Tokio). Northflank auto-detect Cargo.toml.
FROM rust:1.75-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock* ./
COPY crates ./crates
# Build (release optimisé)
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/scy-* ./target/release/
EXPOSE 3000
CMD ["./target/release/scy-server"]
