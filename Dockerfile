FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/rust-chat-server .

# Render expects PORT binding
ENV PORT=10000

CMD ["./chat-server"]