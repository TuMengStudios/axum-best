---
name: project-development
description: Mandatory development rules for the axum-best Rust/Axum service.
license: MIT
---

# axum-best Development Rules

Read this skill before inspecting or changing project code. Treat the existing
implementation as the source of truth. Make the smallest change that solves
the task and do not modify unrelated worktree changes.

## Architecture

Preserve this dependency direction:

```text
handlers -> services -> repos (traits) <- data implementations
```

- `handlers/`: HTTP extraction, validation, service invocation only.
- `services/`: business logic; depend on `Arc<dyn Repo>` traits.
- `repos/`: data-access contracts.
- `data/`: SeaORM/MySQL, Redis, and external API implementations.
- `models/`: persistence/domain models.
- `types/`: request/response DTOs, validation, OpenAPI schemas.
- `routers/`: route registration and route middleware.
- `errors/codes.rs`: stable application errors.
- `docs/openapi.rs`: central OpenAPI registration.
- `app/AppContext`: dependency assembly and lifecycle.

Do not put SQL, Redis commands, external calls, or business logic in handlers.
Create concrete dependencies in `AppContext`, inject them into services, and
expose services through `AppState`.

## New API Flow

For a new JSON API, normally implement in this order:

1. Add model and DTOs only when needed.
2. Add `Deserialize + Validate + ToSchema` request types and `Serialize + ToSchema` response types.
3. Add a repository trait and its data-layer implementation.
4. Add service business logic using repository traits.
5. Add a thin handler returning `crate::core::Result<T>`.
6. Register the route in `src/routers/`.
7. Add route middleware, authentication, and rate limiting.
8. Add `#[utoipa::path]`, then register paths and schemas in `src/docs/openapi.rs`.
9. Add focused tests and run verification commands.

Use the validation extractors for validated input:

- JSON: `ValidJson<T>`
- Query: `ValidQuery<T>`
- Path: `ValidPath<T>`
- Form: `ValidForm<T>`

Use plain Axum extractors only when validation is intentionally unnecessary.

## Response and Errors

Standard JSON API handlers use the unified response contract:

```json
{"err_no":10000,"err_msg":"success","data":{}}
```

Return `crate::core::Result<T>` and use `ok!(value)`. Errors are `AppError`:

```json
{"err_no":14000,"err_msg":"Bad Request Params"}
```

The HTTP status and numeric `err_no` are both contract fields. Internal causes
and details are server-side only and must not appear in responses.

- Reuse errors from `src/errors/codes.rs`; clone static values.
- Add new errors there with a unique code, correct status, and safe message.
- Preserve current code families: `10000` success, `14000` validation, `204xx` auth/user, `40400` not found, `40800` timeout, `42900` rate limit, `501xx` Redis, `502xx` database, `505xx` WeChat/JSON, `50000` not implemented.
- Use `with_cause` for safe server-side context and `covert_error` for SeaORM failures.
- Validation errors use `14000`.
- Test status, envelope, error code, and absence of internal details.

Non-JSON endpoints such as metrics, streams, files, or WebSockets may use their
required Axum response type, but must not silently change the standard API
contract.

## Auth, Limits, and Middleware

- Protect routes with the existing `auth::auth` route middleware; do not parse JWTs in handlers.
- `auth::auth` validates Bearer JWT, checks the active user, and inserts `Claims` into request extensions.
- Add `security(("bearerAuth" = []))` to protected OpenAPI paths.
- Add a route-level `RateLimitLayer` to every new endpoint. Use `with_login_quota` for authenticated routes and `with_quota` for public routes such as login and health checks.
- Ensure authenticated rate limiting runs with authenticated claims available; test that users do not share one fallback bucket.
- Preserve global middleware ordering in `src/routers/layers.rs`.
- Never log JWTs, authorization headers, passwords, salts, verification codes, WeChat codes/secrets, DSNs, or unnecessary personal data. Existing sensitive logs are technical debt; do not copy them.

## SeaORM, Redis, and Configuration

- Add schema changes as new files under `migrations/`; do not edit applied migrations. Apply the plain SQL files in timestamp order with the `mysql` client (no sqlx-cli / migrator framework).
- Define each table as a SeaORM entity under `src/models/entity/<table>.rs` (`DeriveEntityModel`, struct named `Model`) and re-export the public name from `src/models/mod.rs` (`UserInfo`, `OAuthAccount`); access it through `Entity`/`ActiveModel`/`Column` with bound values; use transactions for atomic multi-write operations.
- Preserve soft-delete and active-user behavior with explicit `deleted_at` filters; SeaORM has no built-in soft delete.
- Store timestamps as Unix epoch **milliseconds** (`Utc::now().timestamp_millis()`); entity `before_save` fills unset `created_at`/`updated_at`, and creates go through `ActiveModel::insert` so the hook runs.
- Convert SeaORM `DbErr` through the existing mapper (`src/data/db_error.rs`). Keep Redis keys and expiration behavior explicit and stable.
- Add configuration fields to the config type and `etc/config.toml`; keep production secrets in environment/secret management.

## OpenAPI / Swagger

Every endpoint must have:

- exact method and path in `#[utoipa::path]`;
- correct tag, summary, parameters/request body, security, and relevant statuses;
- `ToSchema` for exposed DTOs/models;
- path and schema registration in `src/docs/openapi.rs`.

Swagger is optional and controlled by `state.cfg.swagger.enabled`:

- UI: `/swagger-ui`
- JSON: `/api-docs/openapi.json`

Keep the documented response shape aligned with the runtime. Currently the
runtime wraps payloads in `err_no/err_msg/data`; check existing OpenAPI patterns
before changing response schemas.

## Verification

Add tests for changed behavior, especially validation, auth boundaries, rate
limits, service decisions, error mapping, response envelopes, and OpenAPI
registration. Run what the environment permits and report limitations honestly:

```text
cargo fmt --check
cargo check --all-targets
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Before finishing, confirm architecture, validation, response contract, error
code, auth/limits, database changes, OpenAPI, logging safety, tests, and command
results are all consistent.
