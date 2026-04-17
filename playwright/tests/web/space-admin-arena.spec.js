import { expect, test } from "@playwright/test";
import { click, fill, goto, getEditor } from "../utils";

test.describe.serial("Space admin arena", () => {
  let spaceUrl;
  const postTitle = "Admin Arena Playwright Space";
  const postContents =
    "This space is created by the admin-arena Playwright spec to exercise admin-only UI (overview edit, settings apps section, action creation, quest edit controls).";
  const uniqueAbout = `ADMIN ARENA TEST ${Date.now()}`;

  async function createSpaceFromPost(page) {
    await goto(page, "/");

    // Home → Create Post (creates a draft post, navigates to /posts/:id/edit)
    await click(page, { testId: "home-btn-create" });
    await page.waitForURL(/\/posts\/[^/]+\/edit/, { waitUntil: "load" });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null,
    );

    // Fill post metadata + body
    await fill(page, { placeholder: "Title your post…" }, postTitle);
    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Enable the Space toggle (switch with aria-label "Enable Space"),
    // then hit the primary action which is now labeled "Design Space".
    await click(page, { label: "Enable Space" });
    await click(page, { testId: "post-edit-publish-btn" });

    // Post-edit navigates to SpaceIndexPage (arena) after space creation.
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/?$/, { waitUntil: "load" });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null,
    );

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/$/, "");
  }

  async function hideFab(page) {
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
  }

  // Arena-style action creation: from SpaceIndexPage the admin clicks
  // `admin-add-action-card` to open TypePickerModal, then picks a type which
  // immediately creates the action and navigates to the creator page. There
  // is no intermediate "Create" confirmation.
  async function createAction(page, typeTestId, urlRegex) {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: typeTestId });
    // Per-step navigation can take >30s when the server creates the action
    // and routes to the creator page with a fresh SSR pass.
    await page.waitForURL(urlRegex, { waitUntil: "load", timeout: 60000 });
  }

  test.beforeAll(async ({ browser }) => {
    // beforeAll runs five navigations + four action creations; give it a
    // generous overall window so flaky server latency doesn't kill the run.
    test.setTimeout(300_000);

    const context = await browser.newContext({ storageState: "user.json" });
    const page = await context.newPage();

    try {
      await createSpaceFromPost(page);

      // Create 4 actions (Poll, Discussion, Quiz, Follow) via the new
      // type-picker testIds exposed on the arena dashboard.
      await createAction(page, "type-option-poll", /\/actions\/polls\//);
      await createAction(
        page,
        "type-option-discuss",
        /\/actions\/discussions\/[^/]+\/edit/,
      );
      await createAction(page, "type-option-quiz", /\/actions\/quizzes\//);
      await createAction(page, "type-option-follow", /\/actions\/follows\//);
    } finally {
      await context.close();
    }
  });

  test("Test 1: Admin badge visible in arena topbar", async ({ page }) => {
    await goto(page, spaceUrl);
    // Badge visibility depends on role hydration from the server; give it a
    // generous window before giving up.
    const badge = page.getByTestId("arena-topbar-admin-badge");
    await expect(badge).toBeVisible({ timeout: 15000 });
    await expect(badge).toContainText("Admin");
  });

  test("Test 2: Overview panel edit toggle and cancel", async ({ page }) => {
    await goto(page, spaceUrl);

    await click(page, { testId: "btn-overview" });
    const overview = page.getByTestId("overview-panel");
    await expect(overview).toHaveAttribute("data-open", "true");

    await click(page, { testId: "overview-edit-btn" });
    await expect(overview).toHaveAttribute("data-editing", "true");
    await expect(page.getByTestId("overview-about-input")).toBeVisible();

    await click(page, { testId: "overview-cancel-btn" });
    await expect(overview).toHaveAttribute("data-editing", "false");
  });

  test("Test 3: Overview About save persists", async ({ page }) => {
    await goto(page, spaceUrl);

    await click(page, { testId: "btn-overview" });
    await click(page, { testId: "overview-edit-btn" });

    await fill(page, { testId: "overview-about-input" }, uniqueAbout);
    await click(page, { testId: "overview-save-btn" });

    const overview = page.getByTestId("overview-panel");
    await expect(overview).toHaveAttribute("data-editing", "false", {
      timeout: 15000,
    });

    // Reload, reopen overview, check the saved text
    await goto(page, spaceUrl);
    await click(page, { testId: "btn-overview" });
    await expect(page.getByTestId("overview-panel")).toContainText(uniqueAbout);
  });

  test("Test 4: Settings panel shows Apps section for admin", async ({
    page,
  }) => {
    await goto(page, spaceUrl);

    await click(page, { testId: "btn-settings" });
    const settings = page.getByTestId("settings-panel");
    await expect(settings).toHaveAttribute("data-open", "true");

    const apps = page.getByTestId("apps-section");
    await expect(apps).toBeVisible();
    await expect(apps).toContainText("Installed Apps");
    await expect(apps).toContainText("Available Apps");
  });

  test("Test 5: Install an available app (Analyzes)", async ({ page }) => {
    await goto(page, spaceUrl);

    await click(page, { testId: "btn-settings" });
    await expect(page.getByTestId("apps-section")).toBeVisible();

    const installBtn = page.getByTestId("install-app-analyzes");
    await expect(installBtn).toBeVisible();
    await installBtn.click();

    // Installed row should appear with a Settings button
    await expect(page.getByTestId("settings-app-analyzes")).toBeVisible({
      timeout: 15000,
    });
    // Install button no longer visible (moved to installed)
    await expect(page.getByTestId("install-app-analyzes")).toBeHidden();
  });

  test("Test 6: Add-action card visible for admin", async ({ page }) => {
    await goto(page, spaceUrl);
    await expect(page.getByTestId("admin-add-action-card")).toBeVisible();
  });

  test("Test 7: Type picker modal open/close flows", async ({ page }) => {
    await goto(page, spaceUrl);

    const modal = page.getByTestId("type-picker-modal");

    // Open and verify options
    await click(page, { testId: "admin-add-action-card" });
    await expect(modal).toBeVisible();
    await expect(page.getByTestId("type-option-poll")).toBeVisible();
    await expect(page.getByTestId("type-option-discuss")).toBeVisible();
    await expect(page.getByTestId("type-option-quiz")).toBeVisible();
    await expect(page.getByTestId("type-option-follow")).toBeVisible();

    // Close via X button
    await page.locator(".type-sheet__close").click();
    await expect(modal).toBeHidden();

    // Reopen → close via scrim click
    await click(page, { testId: "admin-add-action-card" });
    await expect(modal).toBeVisible();
    await modal.click({ position: { x: 5, y: 5 } });
    await expect(modal).toBeHidden();
  });

  test("Test 8: Quest cards show edit buttons for admin", async ({ page }) => {
    await goto(page, spaceUrl);

    await expect(page.locator('[data-testid^="quest-edit-btn-"]')).toHaveCount(
      4,
      { timeout: 15000 },
    );
  });

  test("Test 9: Non-admin visitor sees no admin controls", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();
    try {
      await goto(page, spaceUrl);

      await expect(page.getByTestId("arena-topbar-admin-badge")).toHaveCount(0);
      await expect(page.getByTestId("admin-add-action-card")).toHaveCount(0);
      await expect(
        page.locator('[data-testid^="quest-edit-btn-"]'),
      ).toHaveCount(0);

      // Open overview if available, then check Edit button is not present
      const overviewToggle = page.getByTestId("btn-overview");
      if (await overviewToggle.isVisible().catch(() => false)) {
        await overviewToggle.click();
        await expect(page.getByTestId("overview-edit-btn")).toHaveCount(0);
      } else {
        await expect(page.getByTestId("overview-edit-btn")).toHaveCount(0);
      }
    } finally {
      await context.close();
    }
  });
});
