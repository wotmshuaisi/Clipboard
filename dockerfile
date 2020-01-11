FROM ekidd/rust-musl-builder:stable

WORKDIR /usr/src/app

COPY . .

RUN sudo apt-get update && sudo apt-get install -y \
    libmysqlclient-dev \
    openssl

RUN cd ./html && tar zxvf ./html.tar.gz && rm -rf ./html.tar.gz

RUN sudo chown -R rust:rust /usr/src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN mkdir -p /server/html && mkdir -p /server/log

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/clipboard /server/
COPY --from=builder /usr/src/app/html/ /server/

WORKDIR /server/

CMD ["./clipboard"]
