FROM rust:stretch as builder

RUN apt-get update \
    && apt-get -y install tar ca-certificates cmake musl-tools pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp/clipboard

COPY . .

RUN cd ./html && tar zxvf ./html.tar.gz && rm -rf ./html.tar.gz

RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

RUN mkdir -p /server/html && mkdir -p /server/log

COPY --from=builder /tmp/clipboard/target/x86_64-unknown-linux-musl/release/clipboard /server/

COPY --from=builder /tmp/clipboard/html/ /server/

WORKDIR /server/

CMD ["./clipboard"]
