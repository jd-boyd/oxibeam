# Use the official Rust image as the base
FROM rust:1.76 as builder

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build && \
    rm -rf src/

# Now copy the actual source code
COPY . .

# Build the project and run tests
RUN cargo build --release && \
    cargo test

# If you want a smaller final image, you can use a multi-stage build
FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/your-app-name /usr/local/bin/

# Set the startup command
CMD ["your-app-name"]
