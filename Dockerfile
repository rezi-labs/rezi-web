# Build stage
FROM rust:1.85-slim AS builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build your program for release
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/grocy /usr/local/bin/
LABEL org.opencontainers.image.source https://github.com/LunchTimeCode/grocy

# Set the startup command
CMD ["grocy"]