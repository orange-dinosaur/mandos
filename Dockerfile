# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Install protobuf-compiler and other build dependencies
RUN apt-get update && apt-get install -y protobuf-compiler

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build the dependencies without the source code to cache dependencies
RUN mkdir src
RUN echo "fn main() {println!(\"dummy\")}" > src/main.rs
RUN echo "fn lib() {println!(\"dummy\")}" > src/lib.rs
RUN cargo build

# Copy the rest of the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release --bin mandos

# Create the final image
FROM debian:testing-slim

# Set the working directory inside the container
WORKDIR /app

# Install protobuf-compiler in the final image
RUN apt-get update && apt-get install -y protobuf-compiler

# Copy the compiled binary from the builder image
COPY --from=builder /app/target/release/mandos .

# Expose the gRPC port
EXPOSE 50051

# Start the gRPC server
CMD ["./mandos"]