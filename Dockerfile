# Stage 1: Build the application
FROM rust:1.70-slim-bullseye as builder

# Install required system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/nexantara-api

# Copy only the dependency files first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies (this step will be cached as long as Cargo.toml doesn't change)
RUN cargo build --release

# Now copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libpq5 \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m appuser

# Copy the binary from the builder stage
COPY --from=builder /usr/src/nexantara-api/target/release/nexantara-api /usr/local/bin/

# Set the working directory
WORKDIR /app

# Switch to non-root user
USER appuser

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["nexantara-api"]
