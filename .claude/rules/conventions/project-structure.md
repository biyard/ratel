---
globs: ["app/ratel/Cargo.toml", "packages/*/Cargo.toml"]
---

# Project Structure

## Monorepo Layout

| Path | What | Notes |
|------|------|-------|
| `app/ratel/` | Dioxus fullstack app (`app-shell`) | Single package, feature-gated modules |
| `app/ratel/src/common/` | Shared foundation: types, models, components, utils, config | `use crate::common::*;` |
| `app/ratel/src/features/` | Feature modules: auth, posts, spaces, users, teams, membership, admin | Each gated by Cargo feature |
| `packages/by-macros/` | Proc macros (DynamoEntity derive) | |
| `packages/bdk/` | Biyard Dev Kit | |
| `packages/dioxus-translate/` | i18n framework | `translate!` macro |
| `packages/icons/` | Icon library | |

## Feature Flags

`full` (default) = `membership` + `users` + `teams` + `spaces_full`. Also: `web`, `server`, `lambda`, `bypass`.

- `bypass` skips auth verification (accepts `000000`). Must require explicit `--features bypass`, never included in `local-dev` or `full`.

## Feature Gating

- Server-only code: `#[cfg(feature = "server")]`
- Web-only code: `#[cfg(not(feature = "server"))]` or `#[cfg(feature = "web")]`
- Membership fields: `#[cfg(feature = "membership")]`

## Import Conventions

```rust
// Start with wildcard — brings in common items via re-export chain
use crate::features::<module>::*;
// Only add explicit imports for items NOT in the wildcard chain
use crate::common::hooks::use_infinite_query;
```

Check sibling files to see which imports are standard before adding new ones.

## Reference Docs

| Doc | When to read |
|-----|-------------|
| `docs/dioxus-convention.md` | Writing/reviewing Dioxus components |
| `docs/tailwindcss-convention.md` | Styling rules and token usage |
| `docs/playwright-testing.md` | Writing e2e tests |
| `docs/troubleshooting.md` | Debugging async/component issues |
| `docs/dynamo-prefix-convention.md` | Adding DynamoDB `#[dynamo(prefix)]` |
| `docs/entity-type-id-convention.md` | Converting EntityType to IDs |
