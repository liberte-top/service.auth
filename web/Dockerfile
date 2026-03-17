FROM node:20-bookworm-slim AS builder
WORKDIR /app/service.auth/web
RUN corepack enable
ARG GITHUB_PACKAGES_TOKEN=
ARG APP_BUILD_SHA=unknown
ARG APP_BUILD_TIMESTAMP=unknown
ARG VITE_ENV_LABEL=compose
ARG AUTH_API_INTERNAL_URL=http://auth-api:3333
ENV APP_BUILD_SHA=$APP_BUILD_SHA
ENV APP_BUILD_TIMESTAMP=$APP_BUILD_TIMESTAMP
ENV VITE_ENV_LABEL=$VITE_ENV_LABEL
ENV AUTH_API_INTERNAL_URL=$AUTH_API_INTERNAL_URL

COPY service.auth/web/package.json service.auth/web/pnpm-lock.yaml* ./
RUN if [ -n "$GITHUB_PACKAGES_TOKEN" ]; then printf '%s\n%s\n' "@liberte-top:registry=https://npm.pkg.github.com" "//npm.pkg.github.com/:_authToken=$GITHUB_PACKAGES_TOKEN" > /root/.npmrc; fi
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store pnpm install --no-frozen-lockfile

COPY service.auth/web .
RUN pnpm build

FROM node:20-bookworm-slim AS runtime
WORKDIR /app/service.auth/web
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=80
ENV AUTH_API_INTERNAL_URL=http://auth-api:3333
COPY --from=builder /app/service.auth/web/package.json ./package.json
COPY --from=builder /app/service.auth/web/node_modules ./node_modules
COPY --from=builder /app/service.auth/web/build ./build
COPY --from=builder /app/service.auth/web/public ./public

 EXPOSE 80
 CMD ["node", "build"]
