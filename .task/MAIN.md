# Task: service.auth 全面重构初始化（api/web/e2e + CI）

- **Branch:** feat/init-service-auth-refactor
- **Status:** Active
- **Last-Sync:** 2026-02-25 23:01:40 CST (on Perish)

## Phase Stack
- 2.1 API/Web/E2E/CI 实施闭环（Completed）
- 1.1 仓库初始化与重构规范冻结（Completed）

## Timeline
- [2026-02-25 21:51:53 CST] INITIALIZED on Perish.
- [2026-02-25 21:51:53 CST] BOOTSTRAP: public repo initialized with main guard first, then feature-branch task flow started.
- [2026-02-25 22:07:26 CST] IMPLEMENTED: API 裁剪为 health+accounts CRUD；Web 迁移到 Vite+Svelte；E2E Playwright 落地；CI api/web 从占位升级为 build/test/image。
- [2026-02-25 22:07:26 CST] VERIFIED: `cargo test`、`pnpm build`、`docker compose up -d db api web`、`docker compose --profile e2e up --build --abort-on-container-exit e2e` 全部通过。
- [2026-02-25 22:47:43 CST] REFINED: 清理 API 冗余模块文件（auth/session/verification 等残留）并删除 `api/docs`。
- [2026-02-25 22:47:43 CST] REFINED: E2E 从 compose 解耦，改为 `scripts/run-e2e.sh` 独立 runner 容器执行。
- [2026-02-25 22:47:43 CST] REFINED: Web 增加 MPA 示例页面（`health/showcase/notes`）和多入口构建。
- [2026-02-25 22:51:45 CST] REFINED: 按最新决策移除 `scripts/run-e2e.sh`，E2E 仅保留 `e2e` 目录直接执行模式。
- [2026-02-25 22:54:27 CST] REFINED: 引入根 `.env/.env.example` + e2e `dotenv` 自动加载，统一 E2E 环境变量入口并参数化 compose。
- [2026-02-25 22:56:18 CST] REFINED: 固化 web/e2e `package.json` 依赖版本号（移除范围符），并同步 lockfile 与构建校验。
- [2026-02-25 22:57:41 CST] REFINED: 根 `.env(.example)` 与 `e2e/.env(.example)` 解耦，Playwright 仅加载 `e2e/.env`。
- [2026-02-25 23:01:40 CST] REFINED: E2E 目录重构为 `specs/` + `lib/`，并更新 Playwright/TypeScript 配置路径。

## Global References
- **Docs:** .task/docs/REFAC_SPEC.md

---
*Managed via .task Convention*
