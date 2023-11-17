# Stage 1: Build the application
# Use the Rust official image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin web-server
WORKDIR /web-server

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree
COPY ./src ./src
COPY ./static ./static

# Build for release
RUN rm ./target/release/deps/web_server*
RUN cargo build --release

# Stage 2: Setup the runtime environment
# Use the same Rust image for runtime
FROM rust:latest

# Copy the build artifact from the builder stage and set permissions
COPY --from=builder /web-server/target/release/web-server .

# Set the binary as the entrypoint of the container
ENTRYPOINT ["./web-server"]
