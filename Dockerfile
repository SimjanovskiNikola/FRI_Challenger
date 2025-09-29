FROM rust:nightly

WORKDIR /app

RUN rustup component add rust-analyzer

COPY Cargo.toml .
COPY Cargo.lock .
COPY rustfmt.toml .

RUN cargo fetch

COPY . .

RUN cargo build --release

CMD ["cargo", "run"]


