# Chef stage - prepare recipe
FROM rust:1.88-slim AS chef
RUN cargo install cargo-chef
WORKDIR /usr/src/app

# Planner stage - analyze dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage - build dependencies and application
FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Now copy source code and build application
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/assets /
# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/grocy /usr/local/bin/

LABEL org.opencontainers.image.source=https://github.com/LunchTimeCode/grocy

# Set the startup command
CMD ["grocy"]
