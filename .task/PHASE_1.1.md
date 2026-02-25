# Phase: 1.1 - 仓库初始化与重构规范冻结

## Objective
在 service.auth 仓库中完成最小冷启动骨架与可执行重构规范，供独立 agent 按 spec 闭环实施。

## Exit Criteria
- [x] 仓库目录满足目标骨架：service.auth/{api,web,e2e,docker-compose.yml,.github/workflows/*}。
- [x] .task/docs 下提供完整重构 spec，覆盖 API/Web/E2E/CI 边界、验收标准、阶段拆分与风险项。

## Work Log
- [2026-02-25 21:51:53 CST] STARTED: Phase initialized.
- [2026-02-25 21:51:53 CST] CONTEXT: User requested repo-level bootstrap with guard-first ordering.
- [2026-02-25 21:51:53 CST] CHANGED: Created docker-compose.yml, ci.deploy.api.yml, ci.deploy.web.yml, and e2e placeholder.
- [2026-02-25 21:51:53 CST] CHANGED: Copied current auth-api and auth-web as baseline into api/ and web/.
- [2026-02-25 21:51:53 CST] CHANGED: Added .task/docs/REFAC_SPEC.md for independent-agent execution.

## Technical Notes
- **Files Touched:** .github/workflows/*, docker-compose.yml, e2e/README.md, .task/*, api/**, web/**
- **New Dependencies:** None (spec-only stage)
- **Blockers:** Pending implementation phase by separate agent.

---
*This phase will be popped/archived upon meeting exit criteria.*
