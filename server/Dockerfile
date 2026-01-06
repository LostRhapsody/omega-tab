# Rust backend Dockerfile for Railway deployment

# Build stage
FROM rust:1.75-slim as builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/bnt-server /app/bnt-server

# Expose port
EXPOSE 3000

# Run the binary
CMD ["/app/bnt-server"]
