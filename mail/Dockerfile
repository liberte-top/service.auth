FROM rust:1.91-bookworm AS builder
WORKDIR /app

COPY mail/Cargo.toml mail/build.rs ./
COPY mail/src ./src
COPY mail/templates ./templates
COPY proto ../proto
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/service-mail /app/service-mail

USER nonroot:nonroot
EXPOSE 50051
ENTRYPOINT ["/app/service-mail"]
