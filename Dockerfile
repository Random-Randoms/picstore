FROM rust AS builder

WORKDIR /

RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
COPY ./Cargo.toml .
RUN cargo build --release

RUN rm -rf ./src
COPY ./src ./src
RUN cargo build --release
