FROM rust:1 AS builder
WORKDIR /build

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY Cargo.toml Cargo.lock /build/
COPY src /build/src

# Build for the target platform
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /build/target/release/vaulpner /vaulpner
CMD ["/vaulpner"]

# Debug stage for development and troubleshooting
FROM rust:1 AS debug-builder
WORKDIR /build

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY Cargo.toml Cargo.lock /build/
COPY src /build/src

# Build debug version with symbols
RUN rustup target add x86_64-unknown-linux-gnu \
    && cargo build --bin vaulpner

# Debug runtime stage
FROM ubuntu:22.04 AS debug
WORKDIR /build

# Install system dependencies and debugging tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cargo \
    curl \
    vim \
    htop \
    strace \
    gdb \
    lldb \
    valgrind \
    netcat-openbsd \
    dnsutils \
    file \
    less \
    jq \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the debug binary (with symbols)
COPY --from=debug-builder /build/target/debug/vaulpner /vaulpner

# Copy source code for debugging
COPY --from=debug-builder /build/src /src
COPY --from=debug-builder /build/Cargo.toml /Cargo.toml

# Make the binary executable
RUN chmod +x /vaulpner


# Set environment variables for debugging
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
ENV RUST_SRC_PATH=/src

# Default command
CMD ["/vaulpner"]