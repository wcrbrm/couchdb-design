FROM rust:stretch as builder
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
RUN USER=root cargo new --bin couchdb-design
WORKDIR /couchdb-design
COPY ./Cargo.toml ./
RUN cargo build --release
RUN rm -f ./src/main.rs ./target/release/couchdb-design* ./target/release/deps/couchdb-design*
ADD . ./
RUN cargo build --release

FROM debian:stretch
RUN apt-get -y update && apt-get -y install ca-certificates
WORKDIR /docs
VOLUME  /docs
COPY --from=builder /couchdb-design/target/release/couchdb-design /usr/bin/couchdb-design
ENV RUST_BACKTRACE=1
ENTRYPOINT ["/usr/bin/couchdb-design"]
