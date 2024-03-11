FROM rust:latest AS builder
RUN cargo new --vcs=none --bin --offline /crypto-sanctions-server
WORKDIR /crypto-sanctions-server
COPY Cargo.toml Cargo.lock .
RUN cargo build --release && rm -rf ./src && rm ./target/release/deps/crypto_sanctions_server*
COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /crypto-sanctions-server/target/release/crypto-sanctions-server ./server
CMD ["./server", "--host=0.0.0.0", "--log=info"]
