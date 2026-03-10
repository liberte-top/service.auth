# service.auth

Active product layer for the liberte.top experiment.

## Responsibilities

- `api/`: Rust API and OpenAPI source of truth.
- `web/`: Svelte web UI.
- `e2e/`: Playwright smoke coverage.
- `docker-compose.yml`: local full-stack development.

## Boundaries

- Own application code, tests, and local developer workflows.
- Do not own cluster provisioning.
- Do not own Kubernetes apply/release orchestration.

## Change Flow

1. Update API or web code.
2. Regenerate API client when the contract changes.
3. Verify locally with compose, unit tests, and E2E as needed.
4. Hand deployment concerns off to `../kubernetes`.
