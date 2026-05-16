FROM rust:1.91 AS builder
WORKDIR /app

COPY services/gateway/ services/gateway/

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target \
    cargo build --release --manifest-path services/gateway/Cargo.toml --target-dir /app/target && \
    cp /app/target/release/gateway /app/gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/gateway /app/gateway
EXPOSE 8080
CMD ["/app/gateway"]
