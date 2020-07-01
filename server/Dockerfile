FROM rust:1.44.0-slim-buster

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config
RUN cargo install diesel_cli --no-default-features --features postgres

COPY Cargo.toml Cargo.lock diesel.toml ./
RUN mkdir src && touch src/lib.rs && cargo build --release && rm -rf src

COPY src/ src/
RUN cargo build --release

COPY migrations/ migrations/
CMD sleep 1 && diesel migration run && cargo run --release
