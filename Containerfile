FROM rust:1 AS builder
WORKDIR /build

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for the target platform
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:debug
COPY --from=builder /build/target/release/vaulpner /vaulpner
CMD ["/vaulpner"]