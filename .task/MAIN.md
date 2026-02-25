# Task: service.auth 全面重构初始化（api/web/e2e + CI）

- **Branch:** feat/init-service-auth-refactor
- **Status:** Active
- **Last-Sync:** 2026-02-25 22:47:43 CST (on Perish)

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

## Global References
- **Docs:** .task/docs/REFAC_SPEC.md
- **Scripts:** scripts/run-e2e.sh

---
*Managed via .task Convention*
