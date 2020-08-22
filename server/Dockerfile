FROM rust:1.44.0-slim-buster as builder

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config

COPY Cargo.toml Cargo.lock diesel.toml ./
RUN mkdir src && touch src/lib.rs && cargo build --release && rm -rf src

COPY src/ src/
COPY migrations/ migrations/
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev ca-certificates
COPY --from=builder ./target/release/portifolio-server ./server

CMD sleep 1 && ./server
