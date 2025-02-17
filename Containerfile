FROM rust:1 AS builder
WORKDIR /build
COPY . /build
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:debug
COPY --from=builder /build/target/release/ /
CMD ["/vaulpner"]