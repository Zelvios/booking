FROM rust:1.77.1-slim-buster as builder

WORKDIR /src
COPY .. .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY .. .

COPY --from=builder //target/release/web_server /app/web_server

RUN useradd -m web
USER web_server

EXPOSE 8080
CMD ["./web_server"]