# service.auth Next Phase Spec

## Goal
Build independent deploy pipelines for api/web with minimal complexity.

## Required Workflow Behavior
1. Trigger: workflow_dispatch.
2. Build image:
- api image: ghcr.io/liberte-top/service-auth-api:<sha>
- web image: ghcr.io/liberte-top/service-auth-web:<sha>
3. Push image to GHCR.
4. Deploy by kubectl patch on existing deployment (no manifest rewrite in this repo):
- kubectl -n service set image deployment/auth-api auth-api=<image>
- kubectl -n service set image deployment/auth-web auth-web=<image>
5. Wait rollout status.

## Image Tag Policy
- Publish immutable sha tag only.
- Do not publish or deploy latest tag.
- Stable is logical/manual: human updates image sha in kubernetes manifests when promoting.

## Secrets/Input Contract
- GHCR push permissions (GITHUB_TOKEN or PAT).
- SSH_HOST, SSH_USER, SSH_PRIVATE_KEY for remote kubectl host.
- Optional KUBE_NAMESPACE default service.

## Non-Goals
- No GitOps controller in this phase.
- No automatic PR creation to kubernetes repo in this phase.

## Acceptance
- Manual dispatch updates api/web deployments with sha image tag.
- Rollout success visible in workflow logs.
- Workflow summary prints promoted sha candidate and manual sync reminder for kubernetes repo.
