FROM rust:1.76.0 as builder

WORKDIR /usr/src/nkap
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/nkap /usr/local/bin/nkap

CMD ["nkap"]
