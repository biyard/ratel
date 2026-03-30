When Performing a code review, respond in Korean or English. If a PR is written by Korean, respond in Korean, otherwise English.

When performing a code review,
* Find out the anti-pattern in the `/docs/troubleshooting.md` file.
* For files under `app/`, apply the Dioxus convention checks defined in `/docs/dioxus-convention.md`. See `/docs/dioxus-convention-review-checklist.md` for the review checklist.
* Pay special attention to `toast.error()` usage: it must receive a typed `common::Error` variant, never a raw string. See section 12 of `/docs/dioxus-convention.md` for the full toast convention.
* **Do not use `Error::BadRequest(String)` for domain-specific errors.** Instead, define a specific error enum with `Translate` derive (e.g., `SpaceActionQuizError`, `SpaceRewardError`) for user-friendly, i18n-compatible error handling. See the `SpaceRewardError` pattern in `app/ratel/src/common/types/reward/error.rs` and the "Error Handling Convention" section of `CLAUDE.md`. When reviewing code that introduces new `Error::BadRequest(...)` calls, flag it and suggest a typed error variant instead. **Important:** `common::Error` uses `derive(Translate)` with the `#[translate(from)]` attribute on wrapper variants to delegate translation to the inner error's `translate()` method. When adding a new wrapper variant, annotate it with `#[translate(from)]` so the derive macro generates the delegation automatically.
* For TailwindCSS styling, apply the conventions defined in `/docs/tailwindcss-convention.md`. Key checks:
  - Use semantic design tokens (`bg-card`, `text-text-primary`), not raw color values (`bg-neutral-800`).
  - Do not use `light:` or `dark:` prefixes in component classes; define both light/dark values as semantic tokens in `app/ratel/tailwind.css`.
  - Prefer `aria-selected:` / `group-aria-selected:` variants over Rust if/else class selection for toggleable states.
  - Flag excessive `!important` overrides (`!bg-*`, `!text-*`) — components should accept a `class` prop instead.
  - Use `max-tablet:` / `max-mobile:` for responsive breakpoints.

## Dioxus RSX Conventions

### Signal Value Reading in RSX

* **Always call `signal()` explicitly** when using a `Signal<T>` value in RSX string interpolation — write `"{my_signal()}"`, not `"{my_signal}"`.
* Formatting a `Signal` handle directly (without `()`) serializes the Signal wrapper, not the inner value, producing incorrect output.
* This applies to all RSX attributes that use string interpolation: `key`, `class`, `id`, `src`, etc.
* Example: `key: "{input_key()}"` (correct) vs `key: "{input_key}"` (incorrect — formats the Signal handle).
## Server-Client Architecture

* **Centralize computed booleans on the server**: When a boolean decision (e.g., "can participate?") depends on multiple model fields, compute it once on the server and expose it as a field on the response DTO. Flag PRs that duplicate the same condition in both server controllers and client layout/view code.
* **Extract reusable conditions into model helper methods**: When the same boolean condition appears in multiple server-side locations, flag it and suggest extracting it into a method on the model struct (e.g., `SpaceCommon::is_participation_open()`).

## Component Accessibility

* **Switch component must have a `label` prop**: Every `Switch` usage must pass the `label` prop so that the rendered element includes a proper `aria-label` attribute. The `role="switch"` and `aria-checked` attributes are always rendered regardless of the label, but omitting `label` leaves the switch without an accessible name. Flag `Switch` usages that omit `label`.

## Playwright E2E Tests

When writing Playwright test code under `playwright/`, follow the conventions defined in `/docs/playwright-testing.md`.

Key rules:
* Test files go in `playwright/tests/users/*.spec.js` (authenticated user tests), `playwright/tests/spaces/*.spec.js` (space tests), or `playwright/tests/components/*.spec.js` (component tests that manage their own auth context). All plain JavaScript, not TypeScript.
* Always use shared helpers (`goto`, `click`, `fill`, `getLocator`, `getEditor`, `waitPopup`) from `tests/utils.js` instead of raw Playwright APIs.
* Locator options: `testId` > `label` > `role` > `placeholder` > `text` (priority order).
* See `/docs/playwright-testing.md` for utility function signatures, locator option details, app-specific selectors, and complete examples.


### In-Page Interactions vs Navigation

* Do NOT use `waitForLoadState("load")` after non-navigation interactions (autosave, tab switch, blur). It resolves immediately and doesn't wait for the request. Use deterministic UI signals instead.
* Avoid `networkidle` for SPA navigation — use deterministic UI readiness assertions (e.g., `getLocator` for page-specific elements).

### Shared Helpers & Locators

* Always use shared helpers (`goto`, `click`, `fill`, `getLocator`) instead of raw Playwright APIs like `page.getByRole().click()`.
* Avoid raw CSS locators (`page.locator('label:has(...)')`, `page.locator("#id")`). Use semantic selectors via helpers.
* Avoid `.first()` on order-dependent selectors — add stable `data-testid` or `data-pw` attributes instead.
* Don't add manual `waitForLoadState("load")` after `click()` helper — it already waits internally.

### Resource Cleanup & Environment

* When using `browser.newContext()`, wrap in `try/finally` to guarantee `context.close()`.
* Tests using hardcoded verification codes (e.g., `000000`) require `--features bypass` on the backend. Document this dependency clearly.
* Use `make build-testing` (not `make build`) when building Docker images for Playwright tests. `build-testing` includes the `bypass` feature for signup/verification flows.
* After async server calls (e.g., clicking "Verify"), wait for deterministic UI signals instead of `waitForLoadState("load")` which resolves immediately for non-navigation interactions.

## FileUploader Component

* Do not nest `<label>` inside `FileUploader` children — `FileUploader` already renders a `<label>` wrapper. Using `label` as an inner container creates invalid nested `<label>` HTML that breaks click/drag behavior across browsers. Use `div` or `span` for inner containers instead.
* Do not introduce UI loading state without a cancel reset path — if there is no callback to detect file picker dialog cancellation (e.g., `oncancel`), omit loading state rather than risk a permanently stuck loading UI. Only add loading indicators when both success and failure/cancel paths reset the state.

## URL Parsing

* Always `trim_end_matches('/')` before `rsplit('/')` on URLs — trailing slashes produce empty segments that bypass fallback logic (e.g., `extract_filename_from_url` returning `""` instead of `"untitled"`).
* Filter empty segments after splitting — even after trimming, use `.filter(|s| !s.is_empty())` to handle edge cases like double slashes.

## Performance Patterns

* Use `HashMap` for O(1) lookups instead of linear scans when mapping between collections (e.g., post titles by key).
* Avoid redundant `.to_string()` calls in hot paths — store the result in a local variable when the same conversion is used multiple times (e.g., HashMap key lookup).
* Prefer `eq_ignore_ascii_case` over `to_lowercase()` for string matching — `to_lowercase()` allocates a new `String` on every call; `eq_ignore_ascii_case` compares in-place with zero allocation. Use it for case-insensitive matching in `match`-like chains (e.g., file extension detection).
* Avoid unnecessary `.clone()` on owned values — when a value will be moved into a struct or closure, compute derived values (e.g., file extension) from a borrow before the move, then use the original without `.clone()`.
* Destructure structs before partial moves — when a callback receives an owned struct and different fields will be moved at different points, destructure into local variables first (e.g., `let StructName { field1, field2, field3 } = value;`). This makes ownership explicit and avoids partial-move confusion.

## Spelling & Language Consistency

* Use American English spelling throughout the codebase — e.g., "unrecognized" not "unrecognised", "color" not "colour", "initialize" not "initialise". Applies to doc comments, string literals, error messages, and identifiers.

## Primary Actions in Conditional Sections

* **Never nest primary action buttons (e.g., "Create Post") inside conditionally rendered sections** — if a section (e.g., draft timeline) is hidden when its list is empty, any primary action button placed inside that section will also disappear. Keep primary action buttons outside conditional blocks so they remain visible regardless of content state.
* **Separate the action trigger from the content display** — the button that creates new items should always be accessible, even when the list of existing items is empty. Place it in a persistent container above or alongside the conditional section.

## Feature Flag Safety

* `bypass` must NOT be bundled into `local-dev` or other convenience feature groups. Keep it opt-in via explicit `--features bypass` only in test/local scripts.
* For Playwright CI testing, use the dedicated `build-testing` Makefile target which explicitly includes `bypass`. The production `build` target must never include `bypass`.
