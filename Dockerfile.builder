####################################################################################################
## Builder
####################################################################################################

FROM rust:bullseye AS builder

RUN rustup target add x86_64-unknown-linux-musl \
    &&  apt-get update && apt-get install -y musl-tools=1.2.2-1 musl-dev=1.2.2-1 --no-install-recommends \
    && update-ca-certificates


WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release
