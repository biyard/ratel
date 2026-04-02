# Playwright E2E Testing

This document provides comprehensive instructions for writing and maintaining Playwright E2E tests for the Ratel Dioxus application.

## Overview

Playwright tests verify the Dioxus fullstack app (served at `http://localhost:8080`) through browser automation. Tests are written in plain JavaScript and use a shared utility layer for consistent element interaction. All authenticated tests depend on a global auth setup that logs in once and saves browser storage state.

## Directory Structure

```
playwright/
  playwright.config.js            # Playwright config (ESM)
  package.json                    # type: commonjs, devDeps: @playwright/test
  user.json                       # Generated: saved storageState after auth setup
  tests/
    config.js                     # Shared constants (TIMEOUT, BASE_URL, ID)
    utils.js                      # Shared helpers (click, fill, goto, etc.)
    user.auth.setup.js            # Auth setup — logs in and saves storageState
    users/                        # Authenticated user test specs
      post.spec.js                # Example: post creation test
      *.spec.js                   # New test files go here
    spaces/                       # Space-related test specs
      *.spec.js
    components/                   # Component-level test specs (manage own auth context)
      mobile-bottom-nav.spec.js   # Mobile bottom navigation tests
      *.spec.js                   # New component tests go here
```

## Configuration

### `tests/config.js`

Exports a `CONFIGS` object used across all tests:

| Constant | Env Variable | Default | Description |
|----------|-------------|---------|-------------|
| `TIMEOUT` | `PLAYWRIGHT_TIMEOUT` | `30000` | Global timeout (ms) for test-level and navigation timeouts |
| `BASE_URL` | `PLAYWRIGHT_BASE_URL` | `http://localhost:8080` | Base URL of the Dioxus app |
| `ID` | `PLAYWRIGHT_ID` | `Date.now()` | Unique ID for test isolation |

**Timeout hierarchy:**

| Timeout | Value | Controls |
|---------|-------|----------|
| `test.timeout` (`CONFIGS.TIMEOUT`) | 30s | Maximum time a single test can run |
| `navigationTimeout` (`CONFIGS.TIMEOUT`) | 30s | Maximum time for `page.goto()`, `page.waitForURL()`, etc. |
| `expect.timeout` | 5s local / 10s CI | Maximum time for `expect(locator)` assertions (e.g., `toBeVisible()`) |

`CONFIGS.TIMEOUT` controls test-level and navigation timeouts. The `expect.timeout` is configured separately in `playwright.config.js` and is intentionally shorter — assertions should resolve quickly while tests and navigations get more headroom. Helpers like `getLocator()` perform visibility checks using the configured assertion timeout; see the `getLocator()` section below for details.

### `playwright.config.js`

Key settings:

- **testDir**: `.` (project root)
- **fullyParallel**: `true` (parallel file execution)
- **retries**: `2` on CI, `0` locally
- **workers**: `1` on CI, auto locally
- **reporter**: HTML (opens never, hosts on `0.0.0.0`)
- **trace/video/screenshot**: all `"on"`

### Projects

| Project | testMatch | Description |
|---------|-----------|-------------|
| `auth-setup` | `**/*.auth.setup.js` | Runs first; logs in and saves `user.json` |
| `Individual user tests` | `tests/users/**/*.spec.js`, `tests/spaces/**/*.spec.js` | Depends on `auth-setup`; uses saved `user.json` storageState |
| `Component tests` | `tests/components/**/*.spec.js` | Depends on `auth-setup`; no project-level device or storageState (tests manage their own contexts via `browser.newContext()`) |

Authenticated tests run on Desktop Chrome at a fixed **1440x950** viewport. Component tests have **no project-level device or viewport settings**; each test creates its own `browser.newContext()` with explicit viewport (e.g., mobile 375x667) and authentication state. Project-level `use` options are not applied to manual `browser.newContext()` calls, so tests must pass all desired options (viewport, storageState, userAgent, etc.) directly.

## Authentication Flow

The `tests/user.auth.setup.js` file runs before all authenticated tests:

1. Navigates to `/`
2. Clicks "Sign In" button
3. Enters email and clicks "Continue"
4. Enters password and clicks "Continue"
5. Waits for popup to close (login success)
6. Saves browser storage state to `user.json`

All tests in `tests/users/` automatically load this storage state, so they start as an authenticated user.

### Reference: `user.auth.setup.js`

```js
import { test } from "@playwright/test";
import { waitPopup, click, fill, goto } from "./utils";

test("create storage state", async ({ page }) => {
  const email = `hi+user1@biyard.co`;
  const password = "admin!234";

  await goto(page, "/");

  await click(page, { role: "button", text: /sign in/i });
  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, password);
  await click(page, { text: "Continue" });

  await waitPopup(page, { visible: false });

  await page.context().storageState({ path: "user.json" });
});
```

## Utility Functions

All interaction helpers are defined in `tests/utils.js`. **Always use these instead of raw Playwright APIs.**

### `goto(page, path)`

Navigates to `BASE_URL + path` with `waitUntil: "load"`, then waits for the server-rendered Dioxus DOM to be present by checking for `[data-dioxus-id]` elements. It does not guarantee that hydration is fully complete; remaining hydration is handled by Playwright's built-in auto-waiting on subsequent interactions.

```js
await goto(page, "/");           // → http://localhost:8080/
await goto(page, "/spaces");     // → http://localhost:8080/spaces
```

### `click(page, opts)`

Finds an element using locator options, asserts it's visible, clicks it, and waits for `load` state. Returns the locator.

```js
await click(page, { role: "button", text: /sign in/i });
await click(page, { text: "Continue" });
await click(page, { label: "Create Post" });
```

### `clickNoNav(page, opts, clickOptions?)`

Like `click()`, but skips `waitForLoadState("load")` after clicking. Use this for non-navigation UI interactions (e.g., opening a sidebar sheet, toggling a panel) where `waitForLoadState` would resolve immediately or hang because no page navigation occurs.

The optional third parameter `clickOptions` is forwarded directly to Playwright's `locator.click(options)`. Use it when you need click modifiers like `{ force: true }` to bypass DOM-stability retry loops (e.g., when post-hydration effects keep mutating the DOM).

```js
await clickNoNav(page, { testId: "mobile-more-btn" });  // opens sidebar sheet, no navigation
await clickNoNav(page, { testId: "toggle-panel" });       // toggles a panel
await clickNoNav(page, { testId: "mobile-more-btn" }, { force: true });  // force-click, bypassing DOM stability checks
```

### `fill(page, opts, value)`

Finds an element using locator options, asserts it's visible, and fills it with the given value. Returns the locator.

```js
await fill(page, { placeholder: "Enter your email address" }, "user@example.com");
await fill(page, { placeholder: "Title" }, "My Post Title");
```

### `getLocator(page, opts)`

Finds an element using locator options, asserts it's visible (using the configured `expect.timeout` from `playwright.config.js`), and returns the locator. Use this for visibility assertions.

```js
// Assert that "Publish" button is visible
await getLocator(page, { text: "Publish" });

// Get locator for further interaction
const btn = await getLocator(page, { role: "button", text: "Submit" });
await expect(btn).toHaveText("Submit");
```

### `getEditor(page)`

Returns the `[contenteditable]` locator for rich text editor fields (e.g., post body). Asserts the editor is visible.

```js
const editor = await getEditor(page);
await editor.fill("Post body content here.");
```

### `waitPopup(page, { visible })`

Waits for the popup overlay to appear or disappear. The overlay is identified by `data-testid="popup-overlay"`.

- `{ visible: true }` — waits for popup to appear (default)
- `{ visible: false }` — waits for popup to close (removed from DOM)

```js
await waitPopup(page, { visible: true });   // popup opened
await waitPopup(page, { visible: false });  // popup closed
```

**How popups work in Ratel:** The Dioxus `PopupZone` component renders a full-screen overlay `div` with `data-testid="popup-overlay"` and `z-[101]`. When `popup.close()` is called, the internal signal is set to `None` and the entire overlay is **removed from the DOM** (not hidden with CSS).

### `wrap(page, project, baseDir)`

Adds ordered screenshot capture helpers to the page object. Useful for visual regression or documentation.

```js
const p = wrap(page, "my-project", "login-flow");
await p.capture("after-login");        // → screenshots/my-project/login-flow/001-after-login.png
await p.fullCapture("full-page");      // → full-page screenshot
await p.clickAndCapture("Continue");   // → clicks text, waits 500ms, captures
```

## Locator Options

`click`, `fill`, and `getLocator` accept the same options object. Provide exactly one locator strategy:

| Option | Playwright API | Example |
|--------|---------------|---------|
| `testId` | `page.getByTestId(id)` | `{ testId: "email-input" }` |
| `label` | `page.getByLabel(label, { exact: true })` | `{ label: "Create Post" }` |
| `role` + optional `text` | `page.getByRole(role, { name: text, exact: true })` | `{ role: "button", text: /sign in/i }` |
| `placeholder` | `page.getByPlaceholder(ph, { exact: true })` | `{ placeholder: "Enter your email address" }` |
| `text` | `page.getByText(text, { exact: true })` | `{ text: "Continue" }` |

**Resolution priority**: `testId` > `label` > `role` > `placeholder` > `text`

The `text` option in `role` supports both strings and RegExp (e.g., `/sign in/i` for case-insensitive match).

## Writing a New Test

### Step 1: Create a file

Add a new `.spec.js` file under the appropriate directory:

- **`playwright/tests/users/`** — Authenticated user tests (auto-loaded `user.json` storageState)
- **`playwright/tests/spaces/`** — Space-related tests (auto-loaded `user.json` storageState)
- **`playwright/tests/components/`** — Component tests that manage their own auth context (no project-level storageState)

```
playwright/tests/users/my-feature.spec.js       # authenticated test
playwright/tests/components/my-component.spec.js  # component test with custom context
```

### Step 2: Write the test

```js
import { test } from "@playwright/test";
import { click, fill, goto, getLocator } from "../utils";

test("should do something after login", async ({ page }) => {
  // Navigate (storageState is auto-loaded, user is already authenticated)
  await goto(page, "/");

  // Interact
  await click(page, { label: "Create Post" });
  await fill(page, { placeholder: "Title" }, "Test Post");

  // Assert
  await getLocator(page, { text: "Expected outcome" });
});
```

### Step 3: Run the test

```bash
cd playwright
npm test                                         # Run all tests
npx playwright test tests/users/my-feature.spec.js  # Run specific file
```

## Complete Example: Post Creation

```js
import { test } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor } from "../utils";

test("Create a post", async ({ page }) => {
  await goto(page, "/");

  // Click "Create Post" button (identified by aria-label)
  await click(page, { label: "Create Post" });

  // Fill in the title
  await fill(page, { placeholder: "Title" }, "My Playwright Post");

  // Fill in the rich text body
  const editor = await getEditor(page);
  await editor.fill("This is a post created using Playwright.");

  // Publish the post
  await click(page, { text: "Publish" });

  // Verify navigation to the next page
  await getLocator(page, { label: "Create a Space" });
});
```

## App-Specific Reference

### Known `data-testid` Values

| testId | Location | Element |
|--------|----------|---------|
| `email-input` | Login modal | Email input field |
| `password-input` | Login modal | Password input field |
| `continue-button` | Login modal | Continue/submit button |

### Known `aria-label` Values

| aria-label | Location | Element |
|------------|----------|---------|
| `Create Post` | Home page sidebar | Create post button/link |
| `End of feed message` | Feed list | End-of-feed indicator |
| `Sidebar` | Home page | Right sidebar container |

### Login Modal (Two-Step Flow)

1. **Step 1**: Email only → placeholder `"Enter your email address"` → click `"Continue"`
2. **Step 2**: Password appears → placeholder `"Enter your password"` → click `"Continue"`
3. **Success**: Popup overlay is removed from DOM

### Key UI Elements

| Element | Locator Strategy | Value |
|---------|-----------------|-------|
| Sign In button | `{ role: "button", text: /sign in/i }` | Header (unauthenticated) |
| Home nav link | `{ role: "link", text: /home/i }` | Navigation menu |
| Membership nav link | `{ role: "link", text: /membership/i }` | Navigation menu |
| Create Post | `{ label: "Create Post" }` | Home sidebar |
| Log Out | `{ text: "Log Out" }` | Profile dropdown |
| Create Team | `{ text: "Create Team" }` | Profile dropdown |
| Post title input | `{ placeholder: "Title" }` | Post editor |
| Post editor body | `getEditor(page)` | `[contenteditable]` |
| Publish button | `{ text: "Publish" }` | Post editor |

### App Routes

| Route | Page |
|-------|------|
| `/` | Home (feed list) |
| `/auth` | Login page |
| `/auth/forgot-password` | Password reset |
| `/posts/:..rest` | Post feed/detail/editor |
| `/membership/:..rest` | Membership pages |
| `/spaces/:..rest` | Governance spaces |
| `/:username/:..rest` | User profile |
| `/teams/:teamname/:..rest` | Team pages |
| `/admin/:..rest` | Admin section |

## Running Tests

### Local Development

```bash
cd playwright

# Install dependencies (first time)
npm install
npx playwright install chromium

# Run all tests (auth-setup → authenticated tests)
npm test

# Run only authenticated tests (skips auth-setup if user.json exists)
npm run test:auth

# Run only auth setup
npm run test:setup

# Run specific test file
npx playwright test tests/users/post.spec.js

# Run in headed mode (see the browser)
npx playwright test --headed

# Run in debug mode (step through)
npx playwright test --debug

# Open interactive UI mode
npm run ui

# View HTML report after test run
npm run report
```

### Environment Variables

```bash
# Override base URL (e.g., for staging)
PLAYWRIGHT_BASE_URL=https://dev.ratel.foundation npx playwright test

# Override timeout (ms)
PLAYWRIGHT_TIMEOUT=10000 npx playwright test

# Set unique test ID for isolation
PLAYWRIGHT_ID=my-test-run npx playwright test
```

## Troubleshooting

### Test Times Out

- Default test timeout is 30000ms (30s). Increase with `PLAYWRIGHT_TIMEOUT` env var.
- Ensure the app is running at `http://localhost:8080` (or set `PLAYWRIGHT_BASE_URL`).
- Check that the Dioxus WASM app is loading correctly (the `goto()` helper waits for `window.__dioxus_hydrated` which is set in `run.rs` as soon as WASM starts executing).

### Auth Setup Fails

- Verify the test user credentials in `user.auth.setup.js` match a real account.
- Ensure the login modal elements haven't changed (check `data-testid` and placeholder values).
- Check that the popup overlay `data-testid="popup-overlay"` is present on the overlay element.

### Element Not Found

- Use `npx playwright test --debug` to step through and inspect the DOM.
- Verify the locator matches by checking the app in a browser at `http://localhost:8080`.
- Ensure you're using the correct locator option (`testId`, `label`, `role`, `placeholder`, `text`).

### Stale `user.json`

- Delete `playwright/user.json` and re-run to regenerate.
- The auth-setup project will run again automatically.

## Best Practices

1. **Always use utility helpers** — `goto`, `click`, `fill`, `getLocator` instead of raw Playwright APIs. This keeps tests consistent and handles waits automatically.
2. **One assertion per test** — Keep tests focused on a single behavior for clear failure messages.
3. **Use `getLocator` for assertions** — It internally calls `toBeVisible()` with the configured timeout.
4. **Prefer `label` and `testId`** — These are stable selectors. Avoid CSS class selectors.
5. **Don't use `page.waitForTimeout()`** — Use `waitPopup`, `getLocator`, or Playwright's built-in auto-waiting instead.
6. **Plain JavaScript** — All test files use `.js`, not TypeScript.
7. **No `test.describe` needed** — Individual `test()` calls are fine for simple specs.
8. **Avoid raw CSS locators** — Don't use `page.locator('label:has(...)')` or `page.locator("#some-id")`. Use semantic selectors via helpers: `testId` > `label` > `role` > `placeholder` > `text`.
9. **Avoid `.first()` on order-dependent selectors** — Add stable `data-testid` or `data-testid` attributes to the UI and target them specifically, instead of relying on DOM order.
10. **Don't add manual waits after `click()` helper** — `click()` already calls `waitForLoadState("load")` internally. Adding another wait is redundant and slows tests.
11. **Use `try/finally` for browser contexts** — When manually creating `browser.newContext()`, wrap the test body in `try/finally` to guarantee `context.close()` runs.
12. **Document `bypass` feature dependency** — Tests using hardcoded verification codes (e.g., `000000`) only work with `--features bypass`. Note this requirement in the test file header.
13. **Use `build-testing` for Playwright Docker images** — The PR workflow must use `make build-testing` (not `make build`) when building Docker images for Playwright tests. `build-testing` includes the `bypass` feature so signup/verification flows with code `"000000"` work. The production `build` target excludes `bypass` for security.
14. **Wait for async server responses with deterministic UI signals** — After triggering async server calls (e.g., clicking "Verify"), don't rely on `waitForLoadState("load")` which resolves immediately for non-navigation interactions. Wait for a visible UI state change (e.g., `expect(page.getByText("Send", { exact: true })).toBeHidden()`).

## In-Page Interactions vs Navigation

After UI interactions that do **not** cause a page navigation (e.g., autosave on blur, tab switch, selecting options):

- **Do NOT use `waitForLoadState("load")`** — it resolves immediately since no navigation occurred, so it doesn't actually wait for any request to complete
- **Use deterministic UI signals** — wait for a "Saved" indicator, a specific element to appear, or a network request to complete
- **Avoid `networkidle`** — in SPAs, the app may continue fetching after the load event. Use `getLocator` for page-specific elements instead
