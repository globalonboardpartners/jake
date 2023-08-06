# # Use the official Rust image as the base image
# FROM rust:latest

# # Set the working directory inside the container
# WORKDIR /usr/src/app

# # Copy the Cargo.toml and Cargo.lock files to the container
# COPY Cargo.toml Cargo.lock .env ./

# # # Create a dummy source file to help build dependencies without overwriting the real source code
# RUN mkdir src \
#     && echo "fn main() {println!(\"Placeholder for Docker build\")}" > src/main.rs \
#     && cargo build --release

# # # Remove the dummy source file
# RUN rm -f src/main.rs

# # # Copy the rest of the source code
# COPY src ./src

# # Build your Rust application
# # RUN cargo build --release

# # run it
# RUN cargo run --release

# # Expose the port your web server listens on
# EXPOSE 8081

# # Set the command to run your application
# CMD ["./target/release/jake"]

####################################################
# FROM rust:1.67

# WORKDIR /usr/src/jake
# COPY . .

# RUN cargo install --path .

# CMD ["jake"]
######################################################
# Use a lightweight base image that supports your binary
# FROM rust:1.67

# # Set the working directory inside the image
# WORKDIR /usr/src/
# # usr/src/app
# # Copy the binary from your local machine to the image
# COPY target/release/jake /usr/src/jake
# COPY .env /usr/src/
# RUN whoami
# RUN pwd
# RUN chmod +wrx /usr/src/jake
# RUN chmod +wrx /usr/src/.env

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

# # (Optional) Set the binary as the default command to run when the container starts
# ENTRYPOINT ["./usr/src/app/jake"]
###########################################
FROM rust:1.67

WORKDIR /usr/src/jake
COPY . .

RUN cargo install --path .

CMD ["jake"]
