# Chef ----
FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 AS chef
WORKDIR /app


# planner ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# builder ----
FROM chef AS builder 

RUN apt update && apt -y install cmake

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release 


# runtime ------
# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/energy-monitor-cli /usr/local/bin
ENTRYPOINT ["/usr/local/bin/energy-monitor-cli"]
