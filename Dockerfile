# Rust
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Build the dependencies separately to take advantage of Docker layer caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the source code to the working directory
COPY . .

# Build the application
RUN cargo build --release

# Expose the port on which the application will run
EXPOSE 3000

# Run the application
CMD ["cargo", "run", "--release"]
