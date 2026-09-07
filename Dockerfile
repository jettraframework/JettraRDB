# Build Stage
FROM rust:1.85-bookworm AS builder
WORKDIR /usr/src/jettrardb

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release || true

# Copy source code and build production binary
COPY . .
RUN cargo build --release

# Final Stage: Minimal Debian Slim Container
FROM debian:bookworm-slim
WORKDIR /opt/jettrardb

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/jettrardb/target/release/jettrardb /opt/jettrardb/jettrardb
COPY jettrardb.properties /opt/jettrardb/jettrardb.properties

# Ports: REST API (8086), GUI Console (50050), Raft Consensus (50051)
EXPOSE 8086 50050 50051

ENV JETTRA_DATA_DIR=/data
VOLUME ["/data"]

ENTRYPOINT ["/opt/jettrardb/jettrardb"]

