# Base cargo-chef toolchain stage
FROM lukemathwalker/cargo-chef:latest-rust-1.91-bookworm AS chef
WORKDIR /app

# Plan dependency graph from manifests
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependency layers separately for better cache reuse
FROM chef AS builder
ARG APP_BUILD_SHA=unknown
ARG APP_BUILD_TIMESTAMP=unknown
ENV APP_BUILD_SHA=$APP_BUILD_SHA
ENV APP_BUILD_TIMESTAMP=$APP_BUILD_TIMESTAMP
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked

# Runtime stage (distroless for smaller footprint)
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/service-auth-api /app/service-auth-api

USER nonroot:nonroot
EXPOSE 3333
ENTRYPOINT ["/app/service-auth-api"]
