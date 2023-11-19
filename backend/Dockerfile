FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
ENV OPENSSL_DIR=/usr/include/rust-musl-cross/openssl
workdir /backend
COPY . .
RUN apt-get update && apt-get install -y pkg-config && apt-get -y install openssl && apt-get -y install libssl-dev
# RUN ls /usr/include/openssl
RUN dpkg -L openssl
RUN cargo install sqlx-cli && sqlx prepare
RUN cargo build --release

FROM debian:buster-slim AS runtime
COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/backend /api
ENTRYPOINT ["/api"]
EXPOSE 8080