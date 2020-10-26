FROM rust:stretch as builder
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
RUN USER=root cargo new --bin couchdb-design
WORKDIR /couchdb-design
COPY ./Cargo.toml ./Cargo.lock /couchdb-design/
RUN cargo build --release && rm src/*.rs
# dependenies are cached. ready to build
COPY ./src ./src
RUN cargo build --release

FROM debian:stretch
RUN apt-get -y update && apt-get -y install ca-certificates
WORKDIR /docs
VOLUME  /docs
COPY --from=0 /couchdb-design/target/release/couchdb-design /app/couchdb-design
CMD ["./app/couchdb-design"]
