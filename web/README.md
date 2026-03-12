# service.auth web

SvelteKit SSR frontend for `service.auth`.

## Run

```bash
pnpm install
pnpm dev
```

## Environment

- `AUTH_API_INTERNAL_URL` - server-side origin used by the SvelteKit Node process to reach the auth API
- `WEB_VITE_AUTH_API_BASE_URL` - optional fallback for server-side API origin resolution
- `VITE_ENV_LABEL` - build label for metadata

## Build

```bash
pnpm build
pnpm start
```
