FROM rust:alpine

WORKDIR /aoc

COPY Cargo.toml .

RUN mkdir src \
    && echo "//dummy" > src/lib.rs \
    && cargo build

COPY . .

RUN cargo install --path .