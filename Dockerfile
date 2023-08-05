# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the Rust dependencies without building the application itself
RUN mkdir src \
    && echo "fn main() {println!(\"Placeholder for Docker build\")}" > src/main.rs \
    && cargo build --release

# Copy the rest of the source code
COPY src ./src

# Build your Rust application
RUN cargo build --release

# Expose the port your web server listens on
EXPOSE 8081

# Set the command to run your application
CMD ["./target/release/jake"]
