FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
workdir /api
copy . .
RUN apt-get update && apt-get install -y pkg-config
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
copy --from=builder /api/target/x86_64-unknown-linux-musl/release/api /api
ENTRYPOINT ["/api"]
EXPOSE 8080