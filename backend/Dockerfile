# Rust 编译探索 API — 部署到 Railway / Fly.io / VPS
FROM rustlang/rust:nightly-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-compile-backend /usr/local/bin/rust-compile-backend
ENV HOST=0.0.0.0
ENV PORT=8080
EXPOSE 8080
CMD ["rust-compile-backend"]
