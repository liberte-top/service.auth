# Base cargo-chef toolchain stage
FROM lukemathwalker/cargo-chef:latest-rust-1.91-bookworm AS chef
WORKDIR /app/api

# Plan dependency graph from manifests
FROM chef AS planner
WORKDIR /app
COPY api ./api
COPY proto ./proto
WORKDIR /app/api
RUN cargo chef prepare --recipe-path recipe.json

# Build dependency layers separately for better cache reuse
FROM chef AS builder
ARG APP_BUILD_SHA=unknown
ARG APP_BUILD_TIMESTAMP=unknown
ENV APP_BUILD_SHA=$APP_BUILD_SHA
ENV APP_BUILD_TIMESTAMP=$APP_BUILD_TIMESTAMP
WORKDIR /app/api
COPY --from=planner /app/api/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
WORKDIR /app
COPY api ./api
COPY proto ./proto
WORKDIR /app/api
RUN cargo build --release --locked

# Runtime stage (distroless for smaller footprint)
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/api/target/release/service-auth-api /app/service-auth-api

USER nonroot:nonroot
EXPOSE 3333
ENTRYPOINT ["/app/service-auth-api"]
