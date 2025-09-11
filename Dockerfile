# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy manifests and build dependencies first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build only the server binary with native features
RUN cargo build --release --features native --bin server

# Runtime stage - minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/rustdht-server

# Create non-root user
RUN useradd -r -s /bin/false -u 1001 rustdht
USER rustdht

# Expose signaling server port
EXPOSE 9090

# Set environment
ENV RUST_LOG=info

# Run the server
CMD ["rustdht-server"]