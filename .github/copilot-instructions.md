When performing a code review, respond in Korean or English. If a PR is written in Korean, respond in Korean, otherwise English.

When performing a code review, follow the workflow in `.claude/rules/workflows/code-review.md` and apply the convention rules in `.claude/rules/conventions/`:

* **Server functions** (`.claude/rules/conventions/server-functions.md`): Use SubPartition types (`{Name}Partition`, `{Name}EntityType`) with `id` naming for path params and DTOs. Never expose raw `Partition` or `EntityType` in API interfaces.
* **Error handling** (`.claude/rules/conventions/error-handling.md`): Do not use `Error::BadRequest(String)`. Define typed error enums with `Translate` derive or unit variants on `common::Error`. Log details with `crate::error!` before converting to unit error types — users must never see internal details.
* **Styling** (`.claude/rules/conventions/styling.md`): Use semantic design tokens (`bg-card-bg`, `text-text-primary`), not raw Tailwind palette colors (`bg-neutral-800`, `text-gray-500`). Use primitive components (`Button`, `Input`, `Card`, `Row`, `Col`) — never raw HTML elements.
* **i18n** (`.claude/rules/conventions/i18n.md`): All user-facing strings must use the `translate!` macro. Enum values in UI must use `.translate()`, never `.to_string()`.
* **Dioxus app** (`.claude/rules/conventions/dioxus-app.md`): Follow component structure patterns, use `SeoMeta` on page views, guard `wasm_bindgen` calls with `#[cfg(not(feature = "server"))]`.
* **Lint & format** (`.claude/rules/conventions/lint-and-format.md`): Verify `rustywind` and `dx fmt` were applied to changed `.rs` files.
* **Tests** (`.claude/rules/conventions/server-function-tests.md`, `.claude/rules/conventions/playwright-tests.md`): Verify server function tests exist for new/changed endpoints. Verify e2e tests for user-facing changes.
* **MCP tools** (`.claude/rules/conventions/mcp-tools.md`): If MCP tools are added, verify `#[mcp_tool]` annotation, registration in `server.rs`, and integration tests in `mcp_tests.rs`.
* **Anti-patterns**: Check `/docs/troubleshooting.md` for known anti-patterns.
