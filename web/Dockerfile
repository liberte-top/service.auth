FROM node:20-bookworm-slim AS builder
WORKDIR /app
RUN corepack enable
ARG VITE_ENV_LABEL=compose
ENV VITE_ENV_LABEL=$VITE_ENV_LABEL

COPY package.json pnpm-lock.yaml* ./
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store pnpm install --no-frozen-lockfile

COPY . ./
RUN pnpm build

FROM nginx:1.27-alpine AS runtime
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
