# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

# Copy .sqlx
COPY .sqlx .sqlx

# Cache dependencies
RUN cargo fetch

# enable SQLx offline
ENV SQLX_OFFLINE=true

# Build release
RUN cargo build --release --bins

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install dependencies for Rust binary
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary dari builder
COPY --from=builder /app/target/release/nexantara-api /app
COPY --from=builder /app/target/release/import_wilayah /app

# Expose port
EXPOSE 9000

CMD ["./nexantara-api"]
