FROM rust:1.71-slim-buster

WORKDIR /app

RUN apt-get update && apt-get -y install libssl-dev pkg-config

COPY libyards/ ./libyards/

COPY libyards-api/ ./libyards-api/

WORKDIR /app/libyards-api

RUN cargo build --release

CMD ["./target/release/yards-api"]
