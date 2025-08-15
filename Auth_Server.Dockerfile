# Stage 1: Build
FROM clux/muslrust:stable as builder

ENV SQLX_OFFLINE=true
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config musl-tools
RUN rustup target add x86_64-unknown-linux-musl
ENV CC_x86_64_unknown_linux_musl=musl-gcc

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx

RUN cargo build --release

# Stage 2: Runtime
FROM alpine:latest

# Set working directory
WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/auth-server .

USER 1000
# Copy .env
COPY .env ./

# Run the server
CMD ["./auth-server"]
