# Phase: 1.2 - CI 部署链路与镜像版本策略

## Objective
实现 service.auth 的最小可用 CI 部署链路：构建并推送 ghcr 镜像，使用 kubectl 命令局部更新在线 deployment 镜像；稳定版本镜像由人工同步到 kubernetes 仓库清单。

## Exit Criteria
- [x] ci.deploy.api.yml 与 ci.deploy.web.yml 从占位升级为可执行流程（build, push, kubectl set image, rollout status）。
- [x] 明确并文档化镜像标签策略（仅 sha）与人工同步稳定版本步骤（kubernetes 仓库手动更新）。

## Work Log
- [2026-02-26 22:34:49 CST] STARTED: Phase initialized for CI implementation planning.
- [2026-02-26 22:47:54 CST] IMPLEMENTED: Replaced placeholder workflows with executable API/Web deployment pipelines.
- [2026-02-26 22:47:54 CST] VALIDATED: Verified workflow definitions include test/build, GHCR push, kubectl set image, rollout status, and summary reminders.
- [2026-02-26 22:47:54 CST] COMPLETED: Exit criteria satisfied.

## Technical Notes
- **Files Touched:** .github/workflows/ci.deploy.api.yml, .github/workflows/ci.deploy.web.yml
- **New Dependencies:** ghcr login, kubectl in GitHub Actions runner, cluster access secrets
- **Blockers:** none

---
*This phase will be popped/archived upon meeting exit criteria.*
