# FROM: https://hub.docker.com/_/rust/
FROM rust:1.92-bullseye AS builder

WORKDIR /usr/
COPY src/ src/
COPY Cargo.toml Cargo.toml

RUN cargo install --path .

# ---------------------- #

FROM debian:bullseye-slim
RUN apt update && apt install -y ca-certificates glibc-source && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/network_lookup /usr/local/bin/network_lookup

EXPOSE 443

ENTRYPOINT ["network_lookup"]
