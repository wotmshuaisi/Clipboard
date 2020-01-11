FROM rust:latest as cargo-build

RUN apt-get update && apt-get -y install tar ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /tmp/clipboard

COPY . .

RUN cd ./html && tar zxvf ./html.tar.gz && rm -rf ./html.tar.gz

RUN cargo build --release  --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN mkdir -p /server/html && mkdir -p /server/log

COPY --from=cargo-build /tmp/clipboard/target/x86_64-unknown-linux-musl/release/clipboard /server/

COPY --from=cargo-build /tmp/clipboard/html/ /server/

WORKDIR /server/

CMD ["./clipboard"]
