FROM rust:1.77.1-slim-buster as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update \
      && apt-get install -y libpq-dev \
      && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/api /usr/src/app

RUN apt-get update \
      && apt-get install -y libpq-dev \
      && rm -rf /var/lib/apt/lists/*

RUN useradd -m runner
USER runner

EXPOSE 8080
CMD ["./api"]