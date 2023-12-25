# Stage 1: Build the Rust WebAssembly app with Trunk
FROM rust:1.70 AS builder

# Install wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk
RUN cargo install trunk wasm-bindgen-cli

# Install additional dependencies required by Trunk
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Create a new empty shell project
RUN USER=root cargo new --lib client
WORKDIR /client

# Copy your source tree and Cargo.toml/Cargo.lock
COPY ./src ./src
COPY Cargo.toml Cargo.lock index.html ./

# Use Trunk to build the project for release
RUN trunk build --release

# Stage 2: Setup the runtime environment with a web server
FROM nginx:alpine

# Copy static assets from the builder stage
COPY --from=builder /client/dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start Nginx and serve the static files
CMD ["nginx", "-g", "daemon off;"]