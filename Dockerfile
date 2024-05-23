FROM rust:latest

WORKDIR /usr/src/action

COPY Cargo.toml .

COPY src src

RUN cargo build --release

CMD ["/usr/src/action/target/release/list-team-members"]
