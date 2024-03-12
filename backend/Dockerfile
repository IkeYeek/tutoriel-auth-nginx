FROM rust:latest

WORKDIR /

# Install any necessary dependencies here
RUN apt-get update && \
    apt-get install -y build-essential

# Copy only the Cargo.toml and Cargo.lock files to cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./

# Build and cache dependencies
RUN cargo fetch

# Copy the rest of your source code
COPY . .

# Expose the port your Actix Rust app listens on
EXPOSE 8080

# Start the app using cargo run
CMD ["cargo", "run"]