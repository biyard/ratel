# CLAUDE.md

Ratel is a decentralized legislative platform. Monorepo: Dioxus 0.7 fullstack app + Rust backend + blockchain contracts.

## Tech Stack

- **Language**: Rust (edition 2024)
- **Frontend**: Dioxus 0.7 fullstack (RSX macro, TailwindCSS v4)
- **Backend**: Axum 0.8.1 REST API
- **Database**: DynamoDB (single-table design)
- **i18n**: `translate!` macro (`dioxus-translate`)
- **Testing**: Playwright (e2e), `cargo test` (unit/integration)

## Monorepo Layout

| Path | What |
|------|------|
| `app/ratel/` | Dioxus fullstack app (features gated by Cargo features) |
| `app/ratel/src/common/` | Shared types, models, components, utils |
| `app/ratel/src/features/` | Feature modules: auth, posts, spaces, users, teams, membership, admin |
| `packages/by-macros/` | Proc macros (DynamoEntity derive) |
| `packages/dioxus-translate/` | i18n framework |
| `packages/icons/` | Icon library |

## Task Workflows

Feature work flows through three ordered stages — PO → Design → Development. Each has its own workflow file; `feature-development.md` is the master index / decision doc.

- **Feature development (master)**: `.claude/rules/workflows/feature-development.md`
- **Stage 1 · Roadmap elaboration (PO)**: `.claude/rules/workflows/roadmap-elaboration.md`
- **Stage 2 · UI design**: `.claude/rules/workflows/ui-design-implementation.md`
- **Stage 3 · New feature implementation**: `.claude/rules/workflows/develop-a-new-feature.md`
- **Stage 3 · Feature enhancement**: `.claude/rules/workflows/improve-feature.md`
- **Bugfix**: `.claude/rules/workflows/bugfix.md`
- **MCP tools**: `.claude/rules/workflows/implement-mcp-tools.md`
- **Code review**: `.claude/rules/workflows/code-review.md`
- **Fix PR testing**: `.claude/rules/workflows/fix-pr-testing.md`
- **Fix PR Playwright (deep)**: `.claude/rules/workflows/fix-pr-playwright.md`
- **Write Playwright tests**: `.claude/rules/workflows/write-playwright-tests.md`
- **HTML to Dioxus**: `.claude/rules/workflows/html-to-dioxus.md`
