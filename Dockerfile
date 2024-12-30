# Build stage
FROM rust:slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/armeowbot-rust /app/
COPY .env /env/.env
CMD ["/app/armeowbot-rust"]
