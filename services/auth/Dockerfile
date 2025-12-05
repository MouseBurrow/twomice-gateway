# Build stage
FROM rust:1.91 AS builder
WORKDIR /app

# Copy workspace definition first (better caching)
COPY Cargo.toml Cargo.lock ./
COPY migrator migrator
COPY services services
COPY shared shared
#COPY services/auth services/auth

# Build only this service
RUN cargo build --release -p auth

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/auth /app/auth

#EXPOSE 8080
CMD ["/app/auth"]
