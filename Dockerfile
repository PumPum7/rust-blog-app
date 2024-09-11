FROM rust:1.81.0 AS builder
WORKDIR /usr/src/app
COPY . .
RUN rustup target add aarch64-unknown-linux-gnu
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu
RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libsqlite3-0 libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-gnu/release/blogpost_app /usr/local/bin/blogpost_app
COPY --from=builder /usr/src/app/src/index.html /usr/local/bin/index.html

WORKDIR /usr/local/bin
RUN mkdir images
EXPOSE 3000
CMD ["blogpost_app"]