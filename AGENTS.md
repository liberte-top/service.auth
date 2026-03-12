# Service.Auth AGENTS Guide

## Document Index
- `AGENTS.md`: service.auth collaboration and execution conventions.

## Current Flow
- This repository is the active product layer for the liberte.top experiment.
- Runtime stack is `docker-compose` with `db + api + web`.
- E2E verification is Playwright-based under `e2e/`.
- Web and API now run in same-origin deployment mode: the web nginx proxies `/api/` to the API service.

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
├── web/                      # SvelteKit SSR frontend
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
- API CI runs Rust tests, image build, GHCR publish, and opens an image-promotion PR to `kubernetes`.
- Web CI runs frontend build, image build, GHCR publish, and opens an image-promotion PR to `kubernetes`.
- Add dedicated E2E CI only when runtime and stability requirements are explicitly defined.

## Change Policy
- Keep env ownership explicit: root `.env` and `e2e/.env` must stay independent.
- Keep E2E layout explicit: `specs/` for tests, `lib/` for reusable helpers.
- Keep `docker-compose.yml` aligned with `.env.example` keys.
- Treat runtime data as disposable during the current experiment: it is acceptable to reset or clear environments to unblock end-to-end auth testing.
- Treat the delivery path as non-disposable: even for urgent fixes on experimental environments, code changes must still ship through the normal commit + CI + promotion flow instead of ad hoc manual image builds or direct cluster-only hotfixes.
