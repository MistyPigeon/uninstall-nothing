# Build stage
FROM rust:1. 75-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy project files
COPY Cargo.toml .
COPY main.rs .

# Build the release binary with optimizations
RUN cargo build --release --locked

# Runtime stage - lightweight Ubuntu
FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
ARG VERSION=0.1.0

LABEL maintainer="MistyPigeon"
LABEL description="Linux package uninstaller - removes packages"
LABEL version="${VERSION}"

WORKDIR /app

# Install runtime dependencies for all supported package managers
RUN apt-get update && apt-get install -y \
    sudo \
    apt-utils \
    dpkg \
    gnupg2 \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/package-uninstaller /usr/local/bin/package-uninstaller

# Make binary executable
RUN chmod +x /usr/local/bin/package-uninstaller

# Create a non-root user with sudo privileges
RUN useradd -m -s /bin/bash packageuser && \
    echo "packageuser ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    echo "Defaults:packageuser env_keep+=\"PATH\"" >> /etc/sudoers

# Switch to non-root user
USER packageuser

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD which package-uninstaller || exit 1

# Set entrypoint
ENTRYPOINT ["uninstall-nothing"]
