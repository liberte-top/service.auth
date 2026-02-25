# service.auth e2e

Playwright smoke tests:

1. 首页渲染基础 UI
2. health 状态可读取
3. create + read account happy path

Run locally:

```bash
pnpm install
E2E_BASE_URL=http://localhost:5173 pnpm test
```

Or use the repo helper script:

```bash
./scripts/run-e2e.sh
```

Helper script details:

- `docker compose` only starts `db/api/web`
- E2E runs in a standalone Playwright container (not in compose)
- default `E2E_BASE_URL` is `http://web`
