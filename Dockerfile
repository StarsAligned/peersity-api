FROM rust:slim as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:12-slim

COPY --from=builder /usr/src/app/target/release/peersity-api /usr/local/bin/peersity-api

EXPOSE 8080
CMD ["peersity-api"]