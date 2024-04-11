FROM rust:1.77.1-slim-buster as builder

WORKDIR /src
COPY . .

RUN apt-get update \
      && apt-get install -y libpq-dev \
      && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /src/target/release/api /app/api

RUN useradd -m runner
USER runner

EXPOSE 8080
CMD ["./api"]