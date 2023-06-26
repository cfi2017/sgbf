FROM rustlang/rust:nightly AS chef

RUN apt-get update && apt-get install protobuf-compiler -y

RUN cargo install cargo-chef
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

FROM debian:stable-slim AS runtime
WORKDIR /app/sgbf

RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

COPY --from=builder /app/sgbf/target/release/sgbf-api .
COPY ./sgbf-api/config.yaml /app/sgbf/config.yaml

LABEL org.opencontainers.image.source = "https://github.com/cfi2017/sgbf"
CMD ["/app/sgbf/sgbf-api"]
