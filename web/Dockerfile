FROM rust:1.77.1-slim-buster as builder

WORKDIR /src
COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /src/target/release/web /app/web

RUN useradd -m web
USER web

EXPOSE 8080
CMD ["./web"]