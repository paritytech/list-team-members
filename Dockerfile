FROM rust:1.78

WORKDIR /usr/src/action
COPY . .

RUN cargo install --path .

CMD ["action"]
