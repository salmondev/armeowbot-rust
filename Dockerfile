# Build stage
FROM rust:slim AS builder
WORKDIR /usr/src/app

# First, copy only Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

# Then copy the source code
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/armeowbot-rust /app/
CMD ["/app/armeowbot-rust"]
