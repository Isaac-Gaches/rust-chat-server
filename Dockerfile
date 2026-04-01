# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/chat-server .

CMD ["./chat-server"]