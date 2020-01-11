FROM rust:latest as cargo-build

RUN apt-get update && apt-get install -y tar

WORKDIR /tmp/clipboard

COPY . .

RUN cd html/ && tar zxvf ./html/html.tar.gz && rm -rf html.tar.gz

RUN cargo build --release

RUN cargo install --path .

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/clipboard /usr/local/bin/clipboard

RUN mkdir -p /server/html && mkdir -p /server/log

COPY --from=cargo-build /tmp/clipboard/html/ /server/

WORKDIR /server/

CMD ["clipboard"]
