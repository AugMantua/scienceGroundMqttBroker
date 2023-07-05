# Step 1: Build the Rust application
FROM rust:1.68.2-alpine AS builder

# Adding necessary packages
RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev

RUN rustup target add aarch64-unknown-linux-musl
RUN rustup toolchain install stable-aarch64-unknown-linux-musl

WORKDIR /app

RUN rustup target add aarch64-unknown-linux-musl

# Copy the cargo manifest files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (cached)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the source code and build the application
COPY src/ ./src/
RUN cargo build --target aarch64-unknown-linux-musl --release


# Step 2: Create the lightweight runtime image
FROM shinsenter/scratch
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/scienceGroundMqttClient .

# Set the entrypoint command for the container
CMD ["./scienceGroundMqttClient"]
# CMD ["tail", "-f", "/dev/null"]