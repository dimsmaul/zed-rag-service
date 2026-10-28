# Multi-stage build for minimal image size and maximum security
FROM rust:1.80-slim AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
# Create dummy main to cache dependency compilations
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -ms /bin/bash appuser

WORKDIR /app
COPY --from=builder /app/target/release/zed-rag-service /app/zed-rag-service
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser
EXPOSE 8080
ENV PORT=8080
ENV RUST_LOG=info
ENV DB_PATH=/app/data/rag.db

VOLUME ["/app/data"]

CMD ["/app/zed-rag-service"]
