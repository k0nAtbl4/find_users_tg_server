# Stage 1: Build the application
FROM rust:1-slim as builder

WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && apt-get install -y libpq-dev pkg-config

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

WORKDIR /usr/src/app

# Install OpenSSL, which is a dependency of sqlx
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/bot_database .

# Expose the port the app runs on
EXPOSE 3000

# Set the command to run the application
CMD ["./bot_database"]
