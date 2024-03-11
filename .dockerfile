FROM rust:latest AS builder
RUN cargo new --vcs=none --bin --offline /crypto-sanctions
WORKDIR /crypto-sanctions
COPY Cargo.toml Cargo.lock .
RUN cargo build --release && rm -rf ./src && rm ./target/release/deps/crypto_sanctions*
COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /crypto-sanctions/target/release/crypto-sanctions ./crypto-sanctions
CMD ["./crypto-sanctions", "serve", "--host=0.0.0.0"]
