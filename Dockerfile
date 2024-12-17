FROM rust:latest

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY rustfmt.toml .

RUN cargo fetch

COPY . .

RUN cargo build --release

CMD ["cargo", "run"]


