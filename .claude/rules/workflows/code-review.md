# Code Review

## Step 1: Understand the Change
- Read the PR description and linked issue
- Review the full diff to understand scope
- **Skills**: code-review:code-review

## Step 2: Check Server Functions
- Verify SubPartition types (`id` naming) in path params and DTOs
- Verify typed error enums (no `Error::BadRequest(String)`)
- Verify proper `#[cfg(feature = "server")]` gating
- **References**: conventions/server-functions.md, conventions/error-handling.md

## Step 3: Check Frontend
- Verify semantic color tokens (no raw Tailwind palette colors)
- Verify primitive components (`Button`, `Input`, `Card`, `Row`, `Col` — no raw HTML)
- Verify `translate!` macro for all user-facing strings
- Verify `SeoMeta` on page views
- **References**: conventions/styling.md, conventions/dioxus-app.md, conventions/i18n.md, conventions/design-system-guide.md

## Step 4: Check MCP Tools (if applicable)
- Verify `#[mcp_tool]` annotation and `#[mcp(description)]` on params
- Verify registration in `server.rs`
- Verify integration tests in `mcp_tests.rs`
- **References**: conventions/mcp-tools.md

## Step 5: Check Tests
- Verify server function tests exist for new/changed endpoints
- Verify e2e tests for user-facing changes
- **References**: conventions/server-function-tests.md, conventions/playwright-tests.md

## Step 6: Check Lint & Format
- Verify `rustywind` and `dx fmt` were applied to changed `.rs` files
- **References**: conventions/lint-and-format.md

## Step 7: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion
