FROM rust:1.67

WORKDIR /usr/src/backend
copy . .

RUN cargo install --path .

RUN cargo build --release

CMD cargo run --release