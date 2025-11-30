# Build stage
FROM rust:latest as builder

WORKDIR /app

# Install dependencies for sqlx and PostgreSQL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Update Rust to latest stable and add required components
RUN rustup update stable && \
    rustup default stable && \
    rustup component add rustfmt

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Update dependencies to resolve edition2024 conflicts
RUN cargo update

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder
COPY --from=builder /app/target/release/backend /app/backend

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Expose the port
EXPOSE 3000

# Run the application
CMD ["/app/backend"]
