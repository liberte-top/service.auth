# Auth Foundation

## Goal
- Expand `service.auth` into a stronger first-party auth authority without turning it into a generic IAM platform.
- Keep `@liberte-top/shared` business-agnostic and limited to reusable auth mechanics and contracts.
- Prove the architecture with a full vertical slice from API to web to published shared package usage.

## Core Constraints
- Keep the design simple and explicit.
- Do not add speculative fallback layers or defensive branches without a current need.
- Keep policy authority in `service.auth/api`.
- Keep `ts-packages/packages/shared` focused on generic auth nouns, thin helpers, and client-side mechanics.

## Domain Boundary

### In Scope
- `Principal`: the stable authenticated subject
- `Profile`: self-managed identity metadata attached to a principal
- `Scope catalog`: canonical scope names plus human-readable metadata
- `Grant`: which scopes are active for a principal or narrowed credential
- `API token`: create, list, reveal once, revoke, expire, and observe last-used state
- `Actor context`: the resolved identity returned for session or token auth
- `Revocation`: explicit invalidation semantics for sessions and API tokens
- One real protected consumer flow that proves scope enforcement outside pure auth UI

### Out of Scope
- OAuth/OIDC provider features
- MFA, passkeys, WebAuthn
- org/team/role hierarchy
- generic policy engine or wildcard scope model
- broad admin-platform surfaces
- compliance-grade audit platform

## Ownership

### service.auth/api
- Owns principal resolution, profile writes, token lifecycle, scope enforcement, route authorization, and revocation.
- Owns the canonical scope catalog and auth-context semantics.
- Must not depend on web-only convenience logic.

### service.auth/web
- Owns user workflows: profile UI, token management UI, scope visibility, and self-service flows.
- May hide or disable controls based on auth context, but enforcement remains server-side.
- Must consume released package contracts rather than repo-local auth special cases.

### ts-packages/shared
- Owns business-agnostic auth contracts and mechanics only.
- Good shared exports: auth context types, scope predicate helpers, redirect helpers, auth snapshot refresh helpers, auth-aware fetch classification.
- Bad shared exports: concrete scope names, service-specific profile fields, route names, or page workflow logic.

## Planned Vertical Slices

### Slice 1: Auth Contract Backbone
- Unify actor context semantics for session and API token auth.
- Introduce an explicit scope catalog contract.
- Remove demo-only assumptions from API key resolution.
- Publish shared auth contract additions needed by consumers.

### Slice 2: Self Profile
- Add a real self profile read/update surface.
- Separate principal identity from editable profile fields.
- Expose the profile in web settings without conflating it with preferences.

### Slice 3: Personal API Tokens
- Create token with selected scopes and optional expiry.
- Return the secret once, then store only hashed material.
- List active/revoked tokens and revoke them from the UI.

### Slice 4: Scope-Protected Flow
- Drive one real protected resource path with both session auth and API token auth.
- Validate positive authorization and explicit `403` denial on missing scopes.

## Acceptance Criteria
- A signed-in user can view and update their profile.
- A signed-in user can create an API token, copy it once, and later revoke it.
- A token can access a protected flow only when it has the required scopes.
- Revocation takes effect on subsequent requests.
- Session and token auth produce the same core actor context, differing only where intentionally modeled such as `auth_type`.
- Shared-package changes ship as released versions and are validated in a real consumer.
