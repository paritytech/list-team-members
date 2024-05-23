FROM rust:1.67

WORKDIR /usr/src/action
COPY . .

RUN cargo install --path .

CMD ["action"]
