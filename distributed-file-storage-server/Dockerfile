# Use a Debian-based Rust image for ARM64 as the base image to ensure apt-get is available
FROM debian:bullseye AS builder

# Install Rust and necessary system dependencies
RUN apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Install system dependencies for Diesel (PostgreSQL)
RUN apt-get update && apt-get install -y libpq-dev

# Install diesel CLI with PostgreSQL support
RUN /root/.cargo/bin/cargo install diesel_cli --no-default-features --features postgres

# Build the application in release mode
RUN /root/.cargo/bin/cargo build --release

# Use Ubuntu as the base image for the final runtime stage
FROM ubuntu:20.04

# Install necessary runtime system dependencies for Diesel (PostgreSQL)
RUN apt-get update && apt-get install -y \
    libpq-dev \
    libssl-dev \
    libc6

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/distributed-file-storage-server /usr/local/bin/distributed-file-storage-server

# Copy the Diesel CLI binary from the builder image
COPY --from=builder /root/.cargo/bin/diesel /usr/local/bin/diesel

# Copy the migration script
COPY migrate.sh /usr/local/bin/migrate.sh

# Copy the migrations folder into the final image
COPY ./migrations /usr/src/app/migrations

# Give execution permissions to the script
RUN chmod +x /usr/local/bin/migrate.sh

# Expose the port that the application will run on
EXPOSE 8080

# Set the command to run the application
CMD ["/usr/local/bin/migrate.sh"]
