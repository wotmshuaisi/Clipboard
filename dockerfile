FROM rust:alpine3.10 as cargo-build

RUN apk update && apk add tar

WORKDIR /tmp/clipboard

COPY . .

RUN cd ./html && tar zxvf ./html.tar.gz && rm -rf ./html.tar.gz

RUN cargo build --release

RUN cargo install --path .

FROM alpine:latest

RUN mkdir -p /server/html && mkdir -p /server/log

COPY --from=cargo-build /usr/local/cargo/bin/clipboard /server/

COPY --from=cargo-build /tmp/clipboard/html/ /server/

WORKDIR /server/

CMD ["./clipboard"]
