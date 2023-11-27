# FROM messense/rust-musl-cross:x86_64-musl as builder
# ENV SQLX_OFFLINE=true
# workdir /backend
# COPY . .
# RUN apt-get update && apt-get install -y pkg-config && apt-get -y install openssl && apt-get -y install libssl-dev
# # RUN ls /usr/include/openssl
# RUN dpkg -L openssl
# RUN cargo install sqlx-cli && sqlx prepare
# RUN cargo build --release
#
# FROM debian:buster-slim AS runtime
# COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/backend /api
# ENTRYPOINT ["/api"]
# EXPOSE 8080

FROM rust:1.67 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]
EXPOSE 8080