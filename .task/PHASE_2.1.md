# Phase: 2.1 - API/Web/E2E/CI 实施闭环

## Objective
按重构规范将 service.auth 落地到可运行、可验证状态：API 最小 CRUD + OpenAPI、Web Vite+Svelte、E2E 冒烟、CI 基础构建链路。

## Exit Criteria
- [x] API 移除 auth/session 逻辑，仅保留 health + accounts CRUD + OpenAPI。
- [x] Web 完成 Next.js -> Vite + Svelte 迁移，首页可读取 health 并触发 create+read 演示。
- [x] E2E 使用 Playwright 覆盖首页渲染、health、CRUD happy-path。
- [x] Compose 可启动 `db/api/web`，并可通过 `e2e` profile 运行测试。
- [x] CI `ci.deploy.api.yml`、`ci.deploy.web.yml` 从占位升级为构建验证流程。

## Work Log
- [2026-02-25 22:07:26 CST] STARTED: Phase initialized.
- [2026-02-25 22:07:26 CST] CHANGED: API module tree 裁剪，移除 auth/session 编译路径，重写 state/service/repo/schema/openapi/tests。
- [2026-02-25 22:07:26 CST] CHANGED: 增加 API CORS 层，修复 Web 浏览器上下文跨域调用。
- [2026-02-25 22:07:26 CST] CHANGED: Web 重构为 Vite + Svelte + axios openapi client，Docker runtime 改为 nginx 静态服务。
- [2026-02-25 22:07:26 CST] CHANGED: E2E 新增 Playwright 配置与 smoke 用例；修复 playwright 版本与镜像不匹配问题。
- [2026-02-25 22:07:26 CST] CHANGED: CI api/web workflow 增加 build/test/docker build 阶段。
- [2026-02-25 22:07:26 CST] VERIFIED: cargo/pnpm/compose/e2e 全链路通过。
- [2026-02-25 22:47:43 CST] CHANGED: 删除 API 未引用冗余文件（service/auth/session/verification、handler/auth/session、repo/entities/schema 相关）。
- [2026-02-25 22:47:43 CST] CHANGED: 删除 `api/docs`，完善根与 web 忽略规则，清理 Next 残留目录文件。
- [2026-02-25 22:47:43 CST] CHANGED: E2E 从 `docker-compose.yml` 移除，改为 `scripts/run-e2e.sh` 独立容器 runner。
- [2026-02-25 22:47:43 CST] CHANGED: Web 引入 Vite MPA 多入口示例页（`health.html`、`showcase.html`、`notes.html`）。
- [2026-02-25 22:47:43 CST] VERIFIED: `cargo test`、`pnpm build`、`./scripts/run-e2e.sh` 通过。
- [2026-02-25 22:51:45 CST] CHANGED: 删除 `scripts/run-e2e.sh` 与相关文档引用，E2E 回归目录内直接执行命令。
- [2026-02-25 22:54:27 CST] CHANGED: 新增根 `.env.example`，compose 参数化改为读取环境变量，e2e 引入 `dotenv` 统一加载根/局部 `.env`。
- [2026-02-25 22:54:27 CST] VERIFIED: `pnpm typecheck`、`pnpm exec playwright test --list` 通过。
- [2026-02-25 22:56:18 CST] CHANGED: 固化 `web/package.json` 与 `e2e/package.json` 依赖版本号，移除 `^` 范围写法。
- [2026-02-25 22:56:18 CST] VERIFIED: `pnpm install --no-frozen-lockfile`（web/e2e）、`pnpm typecheck`（e2e）、`pnpm build`（web）通过。
- [2026-02-25 22:57:41 CST] CHANGED: 根 `.env.example` 移除 E2E 配置，新增 `e2e/.env.example`，并将 Playwright 改为仅读取 `e2e/.env`。
- [2026-02-25 22:57:41 CST] VERIFIED: `pnpm typecheck`（e2e）、`pnpm exec playwright test --list` 通过。
- [2026-02-25 23:01:40 CST] CHANGED: `e2e/tests` 迁移为 `e2e/specs`，新增 `e2e/lib` 作为公共逻辑目录，并调整配置引用。
- [2026-02-25 23:01:40 CST] VERIFIED: `pnpm typecheck`（e2e）、`pnpm exec playwright test --list` 通过。
- [2026-02-26 22:21:18 CST] CHANGED: `ci.deploy.api.yml` 与 `ci.deploy.web.yml` 精简为最小占位（仅 `workflow_dispatch` + placeholder step）。

## Technical Notes
- **Files Touched:** `api/**`, `web/**`, `e2e/**`, `.github/workflows/ci.deploy.*.yml`, `docker-compose.yml`, `.task/*`
- **New Dependencies:** `api:tower-http(cors)`, `web:svelte/vite`, `e2e:@playwright/test`
- **Blockers:** None

---
*This phase is completed and ready for review/promotion.*
