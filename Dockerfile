# Multi-stage Dockerfile for CrabMQ
# Builder: compile the Rust binary
# Use a Rust/Cargo new enough for lockfile v4 and deps like indexmap (>=1.82)
FROM rust:1.84-bullseye AS builder
WORKDIR /usr/src/crabmq

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config

RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Build release
RUN cargo build --release

# Runtime: small Debian image
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/crabmq/target/release/crabmq /usr/local/bin/crabmq
COPY --from=builder /usr/src/crabmq/config/default.toml /etc/crabmq/default.toml

EXPOSE 1883
VOLUME ["/etc/crabmq"]

ENTRYPOINT ["/usr/local/bin/crabmq"]
CMD ["/etc/crabmq/default.toml"]
