FROM rust:1.83-bookworm AS chef

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked
WORKDIR /app/sgbf

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/sgbf/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --package sgbf-api

FROM debian:bookworm-slim AS runtime
WORKDIR /app/sgbf

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user (UID 10001 to avoid conflicts with system users)
RUN useradd -u 10001 -U -m -s /bin/bash appuser && \
    mkdir -p /tmp && \
    chown -R appuser:appuser /app/sgbf /tmp

EXPOSE 8080

COPY --from=builder /app/sgbf/target/release/sgbf-api .
COPY ./sgbf-api/config.yaml /app/sgbf/config.yaml

# Switch to non-root user
USER appuser

LABEL org.opencontainers.image.source = "https://github.com/cfi2017/sgbf"
CMD ["/app/sgbf/sgbf-api"]
