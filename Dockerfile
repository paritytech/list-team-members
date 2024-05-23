FROM rust:latest

WORKDIR /usr/src/action

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs

CMD ["cargo", "run", "--release"]
