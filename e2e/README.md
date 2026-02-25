# service.auth e2e

Playwright smoke tests:

1. 首页渲染基础 UI
2. health 状态可读取
3. create + read account happy path

Layout:

- `specs/`: Playwright specs
- `lib/`: shared helpers/utilities for specs

Run locally:

```bash
cp .env.example .env
pnpm install
pnpm test
```

Playwright loads environment variables from `e2e/.env` only.
Required key:

- `E2E_BASE_URL` (example: `http://localhost:5173`)
