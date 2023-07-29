FROM rust:1.71.0-slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.18.2
RUN apk --no-cache update
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/clean-rust .
CMD ["/app/clean-rust"]