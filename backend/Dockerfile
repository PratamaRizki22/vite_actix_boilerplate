# Build stage
FROM rustlang/rust:nightly-alpine as builder

# Install build dependencies
RUN apk update && apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconf

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Copy sqlx offline data
COPY .sqlx ./.sqlx

# Build for release with offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/backend /app/backend

# Copy migrations
COPY migrations ./migrations

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./backend"]
