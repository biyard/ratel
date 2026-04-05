---
globs: ["playwright/**/*.js", "playwright/**/*.spec.js"]
---

# Playwright Test Rules

Rules for writing e2e tests in `playwright/`.

## Always Use Shared Helpers

Import from `tests/utils.js`: `goto`, `click`, `fill`, `getLocator`, `getEditor`, `waitPopup`.
**Never use raw Playwright APIs** like `page.getByRole().click()`.

## Locator Priority

`testId` > `label` > `role` > `placeholder` > `text`

**Avoid:**
- Raw CSS locators (`page.locator('label:has(...)')`, `page.locator("#id")`)
- `.first()` on order-dependent selectors — add `data-testid` instead
- Generic `h3`/heading selectors for blur — Dioxus dev toast interferes

## Navigation vs In-Page Interactions

- `goto()` helper handles navigation + hydration wait (`window.dioxus.send`)
- After `click()` helper — do NOT add manual `waitForLoadState("load")` (already internal)
- After non-navigation actions (autosave, tab switch, blur) — do NOT use `waitForLoadState("load")`. Wait for deterministic UI signals instead
- **Never use `waitForLoadState("networkidle")`** — causes CI failures
- After `waitForURL()` — add `waitForFunction` check for `window.dioxus.send`

## Async Server Responses

After triggering async server calls (e.g., "Verify" button), wait for visible UI state change, not `waitForLoadState("load")`:
```js
await expect(page.getByText("Send", { exact: true })).toBeHidden();
```

## Resource Cleanup

When using `browser.newContext()`, wrap in `try/finally` to guarantee `context.close()`.

## Environment Dependencies

- Verification code `"000000"` requires backend with `--features bypass`
- PR workflow Docker: use `make build-testing` (includes bypass), not `make build`

## Configuration

- `retries`: `process.env.CI ? 2 : 0` — never hardcode
- `workers`: `process.env.CI ? 1 : undefined`

## Test Files

- Location: `playwright/tests/web/*.spec.js` (plain JavaScript)
- Always run tests after writing/modifying to verify they pass
- Use keyboard Tab for blur instead of generic heading selectors

## Full Reference

See `docs/playwright-testing.md` for utility function signatures, app-specific selectors, and complete examples.
