FROM node:20-bookworm-slim AS builder
WORKDIR /app/service.auth/web
RUN corepack enable
ARG APP_BUILD_SHA=unknown
ARG APP_BUILD_TIMESTAMP=unknown
ARG VITE_ENV_LABEL=compose
ENV APP_BUILD_SHA=$APP_BUILD_SHA
ENV APP_BUILD_TIMESTAMP=$APP_BUILD_TIMESTAMP
ENV VITE_ENV_LABEL=$VITE_ENV_LABEL

COPY service.auth/web/package.json service.auth/web/pnpm-lock.yaml* ./
COPY packages /app/packages
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store pnpm install --no-frozen-lockfile

COPY service.auth/web .
RUN pnpm build

FROM nginx:1.27-alpine AS runtime
COPY service.auth/web/docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/service.auth/web/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
