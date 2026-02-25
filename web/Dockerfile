# Build stage
FROM node:20-bookworm-slim AS builder
ARG BUILD_COMMIT=unknown
WORKDIR /app
RUN corepack enable

COPY package.json pnpm-lock.yaml ./
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store pnpm install --frozen-lockfile

COPY . ./
RUN printf '%s\n' "$BUILD_COMMIT" > public/version.txt
RUN --mount=type=cache,id=next-cache,target=/app/.next/cache pnpm build && CI=true pnpm prune --prod

# Runtime stage
FROM gcr.io/distroless/nodejs24-debian12
WORKDIR /app

COPY --from=builder --chown=65532:65532 /app/.next/standalone ./
COPY --from=builder --chown=65532:65532 /app/.next/static ./.next/static
COPY --from=builder --chown=65532:65532 /app/public ./public

USER 65532:65532

ENV NODE_ENV=production
ENV PORT=8888

# No EXPOSE: do not publish ports by default
CMD ["server.js"]
