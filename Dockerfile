####################################################################################################
## Builder
####################################################################################################
FROM rust:bullseye AS builder

RUN rustup target add x86_64-unknown-linux-musl \
    &&  apt-get update && apt-get install -y musl-tools musl-dev \
    && update-ca-certificates


WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:3.15

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/slack-message ./


ENTRYPOINT [ "/app/slack-message" ]