FROM rust:1 AS builder
WORKDIR /build

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src/

# Build for the target platform
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:debug
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
COPY Cargo.toml Cargo.lock ./
COPY src ./src/

# Build debug version with symbols
RUN cargo build --bin vaulpner

# Debug runtime stage
FROM ubuntu:22.04 AS debug
WORKDIR /app

# Install system dependencies and debugging tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    wget \
    vim \
    nano \
    htop \
    strace \
    gdb \
    lldb \
    valgrind \
    netcat-openbsd \
    telnet \
    dnsutils \
    iputils-ping \
    tcpdump \
    procps \
    lsof \
    file \
    less \
    jq \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the debug binary (with symbols)
COPY --from=debug-builder /build/target/debug/vaulpner /usr/local/bin/vaulpner

# Copy source code for debugging
COPY --from=debug-builder /build/src /usr/local/src/vaulpner/src
COPY --from=debug-builder /build/Cargo.toml /usr/local/src/vaulpner/

# Make the binary executable
RUN chmod +x /usr/local/bin/vaulpner

# Create a non-root user for security
RUN useradd -m -s /bin/bash debug && \
    chown -R debug:debug /app /usr/local/src/vaulpner

USER debug
WORKDIR /app

# Set environment variables for debugging
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
ENV RUST_SRC_PATH=/usr/local/src/vaulpner

# Default command
CMD ["/usr/local/bin/vaulpner"]