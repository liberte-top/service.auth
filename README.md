# service.auth

Active product layer for the liberte.top experiment.

## Responsibilities

- `api/`: Rust API and OpenAPI source of truth.
- `web/`: Svelte web UI.
- `e2e/`: Playwright smoke coverage.
- `docker-compose.yml`: local full-stack development.

## Deployment Semantics

- `web` is deployed as the public same-origin entrypoint.
- `web` nginx proxies `/api/` to `auth-api`; the frontend does not carry an environment-specific API base URL.
- CI publishes `ghcr.io/liberte-top/service-auth-api` and `ghcr.io/liberte-top/service-auth-web`.

## Boundaries

- Own application code, tests, and local developer workflows.
- Do not own cluster provisioning.
- Do not own Kubernetes apply/release orchestration.

## Change Flow

1. Update API or web code.
2. Regenerate API client when the contract changes.
3. Verify locally with compose, unit tests, and E2E as needed.
4. Publish deployable images from `main`.
5. Hand deployment concerns off to `../kubernetes`.
