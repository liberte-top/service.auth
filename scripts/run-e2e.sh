#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
docker compose up -d --remove-orphans db api web

docker run --rm \
  --network serviceauth_default \
  --user "$(id -u):$(id -g)" \
  -e HOME=/tmp \
  -e CI=true \
  -e E2E_BASE_URL="${E2E_BASE_URL:-http://web}" \
  -v "$ROOT_DIR/e2e:/workspace" \
  -w /workspace \
  mcr.microsoft.com/playwright:v1.55.0-jammy \
  /bin/bash -lc "corepack pnpm install --no-frozen-lockfile && corepack pnpm test"
