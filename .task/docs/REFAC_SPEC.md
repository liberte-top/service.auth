# service.auth Refactor Spec (v1)

## 1. 背景与目标
当前 `auth-api` 与 `auth-web` 的实现与部署路径偏重，需统一到一个新的 `service.auth` 仓库，并先收敛成「可部署、可验证、可迭代」的最小闭环。

核心目标：
- 单仓库承载 `api + web + e2e + CI`。
- API 去复杂认证流程，仅保留最小 CRUD + OpenAPI 冒烟。
- Web 从 Next.js 迁移到 Vite + Svelte，保留 OpenAPI 客户端调用能力。
- 保持 CI 可分模块独立演进（guard/api/web 分离）。

## 2. 目标目录（强约束）
```text
service.auth/
├── api/                     # 基于当前 auth-api 结构演进
├── web/                     # 迁移为 Vite + Svelte
├── e2e/                     # E2E 冒烟测试目录
├── docker-compose.yml       # 本地联调编排（api/web/db/e2e profile）
└── .github/workflows/
    ├── ci.guard.yml
    ├── ci.deploy.api.yml
    └── ci.deploy.web.yml
```

说明：GitHub 工作流目录实际必须为 `.github/workflows/`（`workflow` 单数无效）。

## 3. API 重构范围

### 3.1 保留项
- Rust + Axum + SeaORM 技术栈保持不变（降低迁移成本）。
- OpenAPI 产出链路保留：
  - `GET /api/openapi.json`
  - Swagger UI（如 `/api/docs`）
- 健康检查保留：`GET /api/v1/health`

### 3.2 删除项（必须）
- 全部登录注册相关逻辑：
  - password register/login/logout
  - email verification
  - github oauth
- 会话/redis 依赖链路（session service、cookie auth flow）
- 与认证行为强绑定的 schema/repo/service

### 3.3 最小 CRUD（必须）
建议仅保留一个资源：`accounts`。
- `POST /api/v1/accounts`
- `GET /api/v1/accounts/{uid}`
- `PATCH /api/v1/accounts/{uid}`
- `DELETE /api/v1/accounts/{uid}`

数据模型最小字段建议：
- `uid` (uuid)
- `account_type` (string)
- `username` (nullable)
- `email` (nullable)
- `phone` (nullable)
- `created_at/updated_at/deleted_at`

### 3.4 验收标准
- `cargo test` 通过。
- `docker build ./api` 成功。
- `kubectl/apply` 前置 smoke（本地 compose）可完成 CRUD + health。
- OpenAPI 文档能覆盖健康和 CRUD。

## 4. Web 重构范围

### 4.1 迁移目标
从 Next.js 切换到 Vite + Svelte 的最小静态站点。

### 4.2 最小功能
- 首页展示：
  - 环境标记
  - API 健康状态（通过 OpenAPI 客户端调用）
  - 一个最小 CRUD 演示（如创建并读取 account）
- 无 SSR 诉求，无 middleware 诉求。

### 4.3 OpenAPI 保留策略
- 保留 `openapi/` 目录作为 SDK 输出目录。
- 允许继续使用 `orval`（或同类生成器），但必须保证：
  - 代码生成命令固定（如 `pnpm openapi:gen`）
  - baseURL 由 `VITE_AUTH_API_BASE_URL` 控制。

### 4.4 验收标准
- `pnpm build` 成功。
- 产物可由静态容器提供服务。
- Web 页面可实测调用 API health。

## 5. E2E 规范
- 推荐 Playwright。
- 最小用例：
  1. 打开首页，显示基础 UI。
  2. 成功读取 health 状态。
  3. 触发一次 CRUD happy-path（create + read）。
- E2E 目录需可在 compose profile `e2e` 下执行。

## 6. Docker Compose 规范
- 至少包含：`db`, `api`, `web`。
- `e2e` 使用 profile 按需启动。
- 目标是一条命令可起联调：
  - `docker compose up -d db api web`

## 7. CI 规范

### 7.1 ci.guard.yml
- 禁止 `.task/` 进入 `main`。
- job 名保持：`forbid-task-dir-on-main`。

### 7.2 ci.deploy.api.yml
阶段 1（当前）：占位 workflow_dispatch。
阶段 2（实施时）：
- 构建/测试 API
- 构建并发布镜像
- 部署（后续由平台策略决定）

### 7.3 ci.deploy.web.yml
阶段 1（当前）：占位 workflow_dispatch。
阶段 2（实施时）：
- 构建 Web 静态产物
- 构建并发布镜像
- 部署

## 8. 分阶段实施建议（供独立 agent）
- Phase A: API 裁剪（移除 auth/session，保留 CRUD + OpenAPI）
- Phase B: Web 迁移（Next -> Vite+Svelte，打通 health）
- Phase C: E2E 落地（Playwright happy-path）
- Phase D: CI 完整化（api/web deploy pipeline）

## 9. 风险与回退
- 风险 1：API 裁剪过程中 repo/service 依赖残留导致编译失败。
  - 回退：先保留接口签名，再逐层删除实现。
- 风险 2：Web 迁移导致 OpenAPI 调用路径不一致。
  - 回退：保留统一 `openapi/http` 适配层。
- 风险 3：CI 提前复杂化影响节奏。
  - 回退：先占位 workflow，确保主干 guard 和本地 compose 可用。
