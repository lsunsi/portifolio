FROM rust:1.44.0-slim-buster

RUN apt-get update && apt-get install -y libpq-dev

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/lib.rs && cargo build --release && rm -rf src

COPY src/ src/
RUN cargo build --release

CMD cargo run --release
