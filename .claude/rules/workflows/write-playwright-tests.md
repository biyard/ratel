# Write Playwright Tests

Use this workflow for writing a new scenario spec file or adding steps to an existing scenario.

## Step 1: Understand the Feature Under Test

- Identify which pages and user flows are involved
- Determine if this is a new scenario or an extension of an existing one
- Check `playwright/tests/web/` for an existing spec to extend

## Step 2: Choose New or Extend

**New scenario** → create `playwright/tests/web/<feature-name>.spec.js`

**Extend existing** → add `test()` blocks inside the existing `test.describe.serial()` suite, preserving execution order

## Step 3: Identify Locators

Before writing, identify the correct locators for each UI element:
- Prefer `data-testid` — add to RSX if missing (`"data-testid": "my-element"`)
- Fall back to `label` → `role` → `placeholder` → `text`
- **Never** use raw CSS selectors or `.first()` on ambiguous elements

## Step 4: Write the Test

### File structure

```js
import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor, waitPopup } from "../utils";

test.describe.serial("My feature scenario", () => {
  let sharedState; // e.g. spaceUrl, postId

  test("Step 1: ...", async ({ page }) => {
    await goto(page, "/");
    // interactions
  });

  test("Step 2: ...", async ({ page }) => {
    // subsequent step, uses sharedState set in previous test
  });
});
```

### Navigation

```js
// Always use goto() — handles WASM hydration wait
await goto(page, "/spaces/abc/dashboard");

// After waitForURL(), verify hydration
await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, { waitUntil: "load" });
await page.waitForFunction(() => document.querySelector("[data-dioxus-id]") !== null);
```

### Interactions

```js
// Click (includes waitForLoadState("load") internally)
await click(page, { testId: "submit-btn" });
await click(page, { text: "Publish" });
await click(page, { label: "Confirm" });

// Click without navigation wait (toggles, panels, non-nav actions)
await clickNoNav(page, { testId: "toggle-switch" });

// Fill input
await fill(page, { placeholder: "Enter title..." }, "My Title");

// Rich text editor
const editor = await getEditor(page);
await editor.fill("Content here");

// Wait for popup
await waitPopup(page, { visible: true });
await waitPopup(page, { visible: false });
```

### Assertions

```js
// Wait for UI state change after async server call — never waitForLoadState("networkidle")
await expect(page.getByText("Send", { exact: true })).toBeHidden();
await expect(page.getByTestId("my-element")).toBeVisible();
await expect(page.getByTestId("my-element")).toContainText("Expected text");
```

### Multi-user flows

```js
// New browser context for a different user (no shared session)
const context = await browser.newContext({ storageState: { cookies: [], origins: [] } });
const page2 = await context.newPage();
try {
  await goto(page2, "/");
  // user2 actions
} finally {
  await context.close(); // always close in finally
}
```

## Step 5: Add `data-testid` Attributes if Needed

If a locator can't be expressed cleanly with existing attributes, add `data-testid` to the Dioxus component:

```rust
button {
    "data-testid": "publish-button",
    onclick: move |_| { ... },
    "Publish"
}
```

## Step 6: Run the Test

```bash
cd playwright && npx playwright test tests/web/<file>.spec.js --headed
```

Always run after writing to verify the test passes before claiming it is complete.

- **References**: conventions/playwright-tests.md, conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion
