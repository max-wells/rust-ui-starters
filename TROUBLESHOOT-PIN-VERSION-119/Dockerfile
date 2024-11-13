#/´°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
#/*                   ✨ 1. BUILD STAGE ✨                    */
#/*.•:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

# Build stage
FROM --platform=linux/amd64 debian:bullseye-slim AS builder

# TODO => WARN: FROM --platform flag should not use constant value "linux/amd64" (line 2)
# TODO => WARN: FROM --platform flag should not use constant value "linux/amd64" (line 50)  

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    tree \
    && rm -rf /var/lib/apt/lists/*

# * 💁 I've added package.json in .dockerignore since we only need to copy node_modules.
# RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
#     apt-get install -y nodejs

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install nightly toolchain and necessary components
RUN rustup default nightly && \
    rustup component add rust-src && \
    rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install --locked cargo-leptos

# Install specific version of wasm-bindgen-cli
# RUN cargo install -f wasm-bindgen-cli --version 0.2.95

# Set up the build directory
WORKDIR /app
COPY . .

# Build the application
RUN cargo leptos build --release -vv

# Debug: List contents of target directory
RUN echo "Contents of /app/target:" && \
    tree /app/target -L 3

# Find the built binary
RUN find /app/target -name start_axum -type f -executable -print0 | xargs -0 -I {} cp {} /app/start_axum

# Debug: Check if the binary was copied
RUN ls -l /app/start_axum || echo "Binary not found in /app/"


#/´°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
#/*                  ✨ 2. RUNTIME STAGE ✨                   */
#/*.•:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

# Runtime stage
FROM --platform=linux/amd64 debian:bullseye-slim AS runner

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built application and necessary files from the builder stage
COPY --from=builder /app/start_axum /app/start_axum
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/content-mdx /app/content-mdx
COPY --from=builder /app/public /app/public

# Set the working directory
WORKDIR /app

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="start_axum"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

# Expose the port the app runs on
EXPOSE 3000

# Run the application
CMD ["/app/start_axum"]