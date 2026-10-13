FROM lukemathwalker/cargo-chef:latest-rust-1.94.1-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release --locked

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin appuser
COPY data /app/data
COPY --from=builder /app/target/release/rust_autocomplete_api /usr/local/bin/rust_autocomplete_api
USER appuser
EXPOSE 8080
CMD ["/usr/local/bin/rust_autocomplete_api"]
