# Service.Auth AGENTS Guide

## Document Index
- `AGENTS.md`: service.auth collaboration and execution conventions.

## Current Flow
- This repository is in active refactor mode on `feat/init-service-auth-refactor`.
- Runtime stack is `docker-compose` with `db + api + web`.
- E2E verification is Playwright-based under `e2e/`.

## Single Source of Truth
- Container runtime parameters live in root `.env` (see `.env.example`).
- E2E runtime parameters live in `e2e/.env` (see `e2e/.env.example`) and are intentionally isolated from root env.
- Service implementation lives in `api/` and `web/`; smoke validation lives in `e2e/specs/`.

## Repository Structure (Refactor Map)
Use this as the baseline module map before iterative refactor.

```text
service.auth/
├── .github/workflows/        # CI entrypoints
├── api/                      # Rust API (health + accounts CRUD + OpenAPI)
│   ├── src/
│   ├── tests/
│   ├── Dockerfile
│   └── Cargo.toml
├── web/                      # Vite + Svelte frontend (MPA)
│   ├── src/
│   ├── openapi/
│   ├── docker/
│   ├── Dockerfile
│   └── package.json
├── e2e/                      # Playwright smoke tests
│   ├── specs/                # E2E specs
│   ├── lib/                  # Shared helpers/utilities for specs
│   ├── .env(.example)        # E2E-only runtime params
│   └── package.json
├── scripts/                  # Local helper scripts (currently empty)
├── docker-compose.yml        # Local stack orchestration
├── .env(.example)            # Compose runtime parameters
└── AGENTS.md                 # Collaboration and execution conventions
```

## Runtime Parameters
- Root `.env` (`.env.example`):
  - `POSTGRES_DB`, `POSTGRES_USER`, `POSTGRES_PASSWORD`
  - `DB_PUBLIC_PORT`
  - `DATABASE_URL`
  - `API_PORT`
  - `WEB_PUBLIC_PORT`
  - `WEB_VITE_AUTH_API_BASE_URL`
  - `WEB_VITE_ENV_LABEL`
- E2E `.env` (`e2e/.env.example`):
  - `E2E_BASE_URL`

## Execution Entry
- Stack bootstrap: `docker compose up -d db api web`
- API tests: `cd api && cargo test --locked`
- Web build: `cd web && pnpm install --frozen-lockfile && pnpm build`
- E2E checks: `cd e2e && pnpm install --frozen-lockfile && pnpm test`

## Common Commands
- `docker compose up -d db api web`
- `docker compose ps`
- `docker compose logs -f api`
- `cd api && cargo test --locked`
- `cd web && pnpm build`
- `cd e2e && pnpm typecheck`
- `cd e2e && pnpm exec playwright test --list`

## E2E Workflow
- Playwright loads env variables from `e2e/.env` only.
- Keep shared E2E logic in `e2e/lib/`, keep assertions and flows in `e2e/specs/`.
- E2E should target the running `web` service exposed by `E2E_BASE_URL`.

## Minimal Baseline Regression Checklist
- `docker compose up -d db api web`
- `cd api && cargo test --locked`
- `cd web && pnpm build`
- `cd e2e && pnpm typecheck`
- `cd e2e && pnpm test`

## CI Strategy
- Keep API and Web workflows decoupled in `.github/workflows/ci.deploy.api.yml` and `.github/workflows/ci.deploy.web.yml`.
- API CI should at least run Rust tests and image build.
- Web CI should at least run frontend build and image build.
- Add dedicated E2E CI only when runtime and stability requirements are explicitly defined.

## Change Policy
- Keep env ownership explicit: root `.env` and `e2e/.env` must stay independent.
- Keep E2E layout explicit: `specs/` for tests, `lib/` for reusable helpers.
- Keep `docker-compose.yml` aligned with `.env.example` keys.
