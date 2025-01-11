FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /rust-deploy

# Add the suggested packages, including perl
RUN apt-get update && apt-get install -y clang lld git perl

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /rust-deploy/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder /rust-deploy/target/x86_64-unknown-linux-musl/release/rust-deploy /app/rust-deploy
# Copy all files from the templates directory
COPY src/mail/templates /app/src/mail/templates
# If you have other static files or templates, copy them here as well
# COPY static /app/static

ENTRYPOINT ["/app/rust-deploy"]
EXPOSE 8000