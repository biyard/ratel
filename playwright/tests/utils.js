import { expect } from "@playwright/test";

export function wrap(page, project, baseDir) {
  const pageWithCapture = page;
  pageWithCapture.order = 1;

  pageWithCapture.fullCapture = async (name) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename, fullPage: true });
  };

  pageWithCapture.capture = async (name) => {
    const paddedOrder = String(pageWithCapture.order).padStart(3, "0");
    const filename = `screenshots/${project}/${baseDir}/${paddedOrder}-${name}.png`;
    pageWithCapture.order += 1;
    await page.screenshot({ path: filename });
  };

  pageWithCapture.clickAndCapture = async (name) => {
    await page.locator(`text=${name}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  pageWithCapture.clickXpathAndCapture = async (xpath, name) => {
    await page.locator(`xpath=${xpath}`).click();
    await page.waitForTimeout(500);
    await pageWithCapture.capture(name);
  };

  return pageWithCapture;
}

export async function click(page, opt) {
  const selected = await getLocator(page, opt);

  await selected.click();
  await page.waitForLoadState("load");

  return selected;
}

/**
 * Click an element without waiting for navigation (`waitForLoadState("load")`).
 * Use this for non-navigation UI interactions (e.g., opening a sidebar sheet,
 * toggling a panel) where `waitForLoadState` would resolve immediately or hang
 * because no page navigation occurs.
 */
export async function clickNoNav(page, opt) {
  const selected = await getLocator(page, opt);

  await selected.click();

  return selected;
}

export async function fill(page, opt, value) {
  const selected = await getLocator(page, opt);

  await selected.fill(value);

  return selected;
}

export async function getLocator(
  page,
  { placeholder, text, role, label, testId },
) {
  let selected;

  if (testId) {
    selected = page.getByTestId(testId);
  } else if (label) {
    selected = page.getByLabel(label, { exact: true });
  } else if (role) {
    const opt = { exact: true };
    if (text) {
      opt.name = text;
    }
    selected = page.getByRole(role, opt);
  } else if (placeholder) {
    selected = page.getByPlaceholder(placeholder, { exact: true });
  } else if (text) {
    selected = page.getByText(text, { exact: true });
  } else {
    throw new Error("Either text, label, or data-testid must be provided");
  }

  await expect(selected).toBeVisible();

  return selected;
}

export async function waitPopup(page, { visible = true }) {
  if (visible) {
    await expect(page.locator(".backdrop-blur-\\[10px\\]")).toBeVisible();
  } else {
    await expect(page.locator(".backdrop-blur-\\[10px\\]")).toBeHidden();
  }
}

export async function goto(page, url) {
  // Wait for the WASM response with a timeout fallback. When the browser
  // serves WASM from memory/disk cache on same-context navigations, the
  // response event may never fire, so we race against a 5-second timeout
  // and fall through to the hydration check below.
  await Promise.all([
    Promise.race([
      page.waitForResponse(
        (response) =>
          response.url().includes("app-shell") &&
          response.url().endsWith(".wasm") &&
          response.status() === 200,
      ),
      new Promise((resolve) => setTimeout(resolve, 5000)),
    ]),
    page.goto(url),
  ]);
  await page.waitForLoadState("domcontentloaded");
  await page.waitForTimeout(500);
}

/**
 * Wait until a specific element (by testId) has been hydrated by Dioxus —
 * i.e. it carries a `data-dioxus-id` attribute so its event listeners are
 * attached. Use this before clicking SSR-rendered interactive elements;
 * clicks on non-hydrated nodes are silently dropped, manifesting later as
 * "dropdown did not open / state did not toggle" failures.
 */
export async function waitForHydrated(page, testId, timeout = 15000) {
  await page.waitForFunction(
    (id) => {
      const el = document.querySelector(`[data-testid="${id}"]`);
      return !!el && el.hasAttribute("data-dioxus-id");
    },
    testId,
    { timeout },
  );
}

export async function getEditor(page) {
  const editor = page.locator("[contenteditable]");
  await expect(editor).toBeVisible();

  return editor;
}

/**
 * Create a team by driving the home → Teams HUD dropdown → "Create Team"
 * footer → ArenaTeamCreationPopup UI flow. After submit, Dioxus navigates
 * to `/{teamUsername}/home`, which the helper waits for.
 *
 * Requires: logged-in user.
 */
export async function createTeamFromHome(
  page,
  { username, nickname, description = "" },
) {
  await goto(page, "/");

  // Open the Teams dropdown (same trigger as openTeamFromHome).
  await expect(page.getByTestId("home-btn-teams")).toBeVisible({
    timeout: 15000,
  });
  // Wait until the button itself is hydrated — otherwise the click fires
  // before Dioxus attaches the `teams_open.toggle()` handler and the event
  // is silently dropped.
  await waitForHydrated(page, "home-btn-teams");
  await clickNoNav(page, { testId: "home-btn-teams" });
  // Dropdown is always rendered but toggled via aria-expanded + CSS
  // visibility, so bump the timeout past the CSS transition (0.18s) plus
  // a safety margin for Dioxus re-render.
  await expect(page.getByTestId("home-teams-dd")).toBeVisible({
    timeout: 10000,
  });

  // Click the "Create Team" footer — opens ArenaTeamCreationPopup.
  await clickNoNav(page, { testId: "home-btn-create-team" });
  await expect(page.getByTestId("arena-create-team-popup")).toBeVisible({
    timeout: 10000,
  });

  // Fill the TeamCreationForm fields.
  await fill(page, { testId: "team-nickname-input" }, nickname);
  await fill(page, { testId: "team-username-input" }, username);
  if (description) {
    await fill(page, { testId: "team-description-input" }, description);
  }

  // Submit — the arena popup handler performs the POST, closes the popup,
  // and navigates to the new team's home page.
  await click(page, { testId: "team-create-submit" });
  await page.waitForURL(new RegExp(`/${username}/home`), {
    waitUntil: "load",
  });
  await page.waitForFunction(
    () => document.querySelector("[data-dioxus-id]") !== null,
  );
}

/**
 * Navigate to a team home by clicking the Teams icon in the home page HUD
 * and picking the team from the dropdown. Mirrors the production flow a
 * logged-in user sees on the main arena (`/`).
 *
 * Requires: logged-in user who owns / belongs to the team at `teamUsername`.
 */
export async function openTeamFromHome(page, teamUsername) {
  await goto(page, "/");

  // Wait for the Teams HUD button (only renders when logged in + hydrated).
  await expect(page.getByTestId("home-btn-teams")).toBeVisible({
    timeout: 15000,
  });
  // Wait until the button itself is hydrated — clicks on SSR-rendered
  // elements that haven't received their Dioxus event handlers yet are
  // silently dropped.
  await waitForHydrated(page, "home-btn-teams");

  // Open the dropdown — non-navigation toggle, so clickNoNav.
  await clickNoNav(page, { testId: "home-btn-teams" });
  // Dropdown is always rendered but toggled via aria-expanded + CSS
  // visibility, so bump the timeout past the CSS transition (0.18s) plus
  // a safety margin for Dioxus re-render.
  await expect(page.getByTestId("home-teams-dd")).toBeVisible({
    timeout: 10000,
  });

  // CI PR runs start from a clean DB, so the freshly-created team is
  // guaranteed to be on the first page of the infinite-scroll dropdown.
  // Locally the dropdown can hold many older teams — scroll the inner
  // list (`#home-teams-dd-list`, the actual scrollable container that
  // triggers pagination via its onscroll handler) until the requested
  // team item appears. Bounded to 30 attempts to avoid infinite loops.
  const teamItem = page.getByTestId(`home-team-dd-item-${teamUsername}`);
  let visible = await teamItem.isVisible().catch(() => false);
  if (!visible) {
    for (let i = 0; i < 30; i++) {
      const reachedBottom = await page.evaluate(() => {
        const el = document.getElementById("home-teams-dd-list");
        if (!el) return true;
        const before = el.scrollTop;
        el.scrollTop = el.scrollHeight;
        // If scroll didn't move we're at the bottom and there is no more to load.
        return el.scrollTop === before;
      });
      // Give pagination + render a moment to catch up after scroll.
      await page.waitForTimeout(400);
      visible = await teamItem.isVisible().catch(() => false);
      if (visible) break;
      if (reachedBottom) break;
    }
  }
  await expect(teamItem).toBeVisible({ timeout: 15000 });
  await teamItem.click();

  await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
    waitUntil: "load",
  });
  await page.waitForFunction(
    () => document.querySelector("[data-dioxus-id]") !== null,
  );
}

/**
 * Create a draft post for a team. If the page is already on the team's home
 * (e.g. right after `createTeamFromHome`), clicks Create Post directly.
 * Otherwise drives the home → Teams dropdown → pick team flow first.
 *
 * The conditional navigation avoids an unnecessary `goto("/")` round-trip
 * that interrupts Dioxus hydration mid-flight (events on the freshly-
 * navigated home page fail to attach before the test tries to click the
 * Teams dropdown).
 *
 * Requires: logged-in user with TeamAdmin/Owner role for the given team.
 */
export async function createTeamPostFromHome(
  page,
  teamUsername,
  postTitle,
  postContents,
) {
  const teamHomeRe = new RegExp(`/${teamUsername}/home$`);
  if (!teamHomeRe.test(new URL(page.url()).pathname)) {
    await openTeamFromHome(page, teamUsername);
  }

  // The create button is gated by arena.can_edit — owners/admins see it after
  // the team-arena layout propagates the role into context (post-hydration).
  await expect(page.getByTestId("team-home-create-post")).toBeVisible({
    timeout: 15000,
  });
  await click(page, { testId: "team-home-create-post" });
  await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

  // Title + body
  await fill(page, { placeholder: "Title your post…" }, postTitle);
  const editor = await getEditor(page);
  await editor.fill(postContents);

  // Autosave confirmation — guards that the draft is persisted before the
  // caller layers on space creation, publishing, etc.
  await expect(page.getByText("All changes saved")).toBeVisible({
    timeout: 15000,
  });

  const match = page.url().match(/\/posts\/([^/]+)\/edit/);
  if (!match) {
    throw new Error(`could not extract post id from url: ${page.url()}`);
  }
  return match[1];
}

// ── Arena action-editor helpers ─────────────────────────────────────────────
// Shared UI primitives for the poll/quiz/discussion/follow creator pages.
// When the arena UI evolves, update these helpers rather than every spec.

/**
 * Create a new action from the arena dashboard. Clicks the admin "add
 * action" card, picks a type from the TypePickerModal, and waits for the
 * creator page to load. `typeKey` is one of `"poll"`, `"quiz"`,
 * `"discuss"`, `"follow"`.
 *
 * Requires: the caller has already navigated to the space root URL and
 * the FAB has been hidden if it overlaps modal buttons.
 */
export async function createAction(page, spaceUrl, typeKey, urlRegex) {
  await goto(page, spaceUrl);
  // Hide FAB that may overlap the TypePicker buttons.
  await page.evaluate(() => {
    const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
    if (fab) fab.style.display = "none";
  });
  await click(page, { testId: "admin-add-action-card" });
  await click(page, { testId: `type-option-${typeKey}` });
  await page.waitForURL(urlRegex, { waitUntil: "load", timeout: 60000 });
}

/**
 * Blur the currently-focused field to commit an autosave (the new arena
 * editors persist on blur; there is no Save button).
 */
export async function commitAutosave(page) {
  await page.keyboard.press("Tab");
  await page.waitForLoadState("load");
}

/**
 * Add a new question on a poll-creator page and pick its type.
 * `type` is one of: `"single"`, `"multi"`.
 */
export async function addPollQuestion(page, type = "single") {
  await click(page, { testId: "poll-question-add" });
  const labels = { single: "Single", multi: "Multi" };
  const label = labels[type];
  if (!label) {
    throw new Error(`Unsupported poll question type: ${type}`);
  }
  // Type segment only needs to be (re-)clicked when it is not already selected.
  const segment = page.getByText(label, { exact: true });
  if ((await segment.count()) > 0) {
    const first = segment.first();
    if ((await first.getAttribute("aria-selected")) !== "true") {
      await first.click();
      await page.waitForLoadState("load");
    }
  }
}

/**
 * Fill the title + options of a poll question identified by its index. The
 * arena editor exposes two option inputs by default (no "Add Option" UI);
 * pass at most two option strings. Each field is blurred after fill so
 * the per-field onblur autosave commits before the next field is touched.
 */
export async function fillPollQuestion(page, idx, { title, options = [] }) {
  const block = page.getByTestId(`poll-question-${idx}`);
  await expect(block).toBeVisible();
  const titleInput = block.locator("input.input").first();
  await titleInput.fill(title);
  await titleInput.press("Tab");
  await page.waitForLoadState("load");

  for (let i = 0; i < options.length; i += 1) {
    const opt = page
      .getByTestId(`poll-question-${idx}-opt-${i}`)
      .locator("input");
    await opt.fill(options[i]);
    await opt.press("Tab");
    await page.waitForLoadState("load");
    // Small settle window so the onblur server round-trip completes before
    // the next option's focus races it.
    await page.waitForTimeout(200);
  }
}

/**
 * Toggle the "require prerequisite" tile on an action ConfigCard. The tile
 * replaces the legacy "Settings" tab + prerequisite card flow.
 */
export async function togglePrerequisite(page) {
  const tile = page.getByTestId("tile-prereq");
  await expect(tile).toBeVisible();
  await tile.locator('[role="switch"]').click();
  await page.waitForLoadState("load");
}

/**
 * Turn on the reward-setting toggle and set a credit amount on an action
 * ConfigCard. No-op if the toggle is not present (e.g. free membership
 * showing the Unlock button instead).
 */
export async function setReward(page, credits) {
  const toggle = page.getByTestId("reward-setting-toggle");
  if ((await toggle.count()) === 0) return;
  if ((await toggle.getAttribute("aria-checked")) !== "true") {
    await toggle.click();
    await page.waitForLoadState("load");
  }
  const creditInput = page.getByTestId("reward-credit-input");
  await expect(creditInput).toBeVisible();
  await creditInput.fill(String(credits));
  await commitAutosave(page);
}
