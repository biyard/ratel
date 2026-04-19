import { expect, test } from "@playwright/test";
import {
  click,
  createTeamFromHome,
  createTeamPostFromHome,
  fill,
  getLocator,
  goto,
} from "../utils";

// Mirrors the reward-anonymous-space-with-collective-panel-by-user spec's
// proven setup path: team via UI → team post via UI → space via REST, then
// each action creation is its own test so every step starts from a fresh
// page context (avoids any cross-step UI state bleed that broke the previous
// single-test setup).
test.describe.serial("Space admin arena", () => {
  let spaceUrl;

  const teamNickname = "Admin Arena Team";
  const teamUsername = `e2e_aa_${Date.now()}`;
  const postTitle = "Admin Arena Playwright Space";
  const postContents =
    "This space is created by the admin-arena Playwright spec to exercise admin-only UI (overview edit, settings apps section, action creation, quest edit controls).";
  const uniqueAbout = `ADMIN ARENA TEST ${Date.now()}`;

  // Hide the floating action button (DevTools FAB) that may overlap modal
  // buttons and steal clicks.
  async function hideFab(page) {
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
  }

  test("Setup: create team, post, and space", async ({ page }) => {
    // Drive team + post creation through production UI, then create the
    // space via REST — the same approach used by the reward-panel spec.
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "E2E test team for admin arena",
    });

    const postId = await createTeamPostFromHome(
      page,
      teamUsername,
      postTitle,
      postContents,
    );

    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(
      spaceRes.ok(),
      `create space: ${await spaceRes.text()}`,
    ).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;

    spaceUrl = `/spaces/${spaceId}`;

    // Sanity-check the creator dashboard is reachable before proceeding.
    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
  });

  test("Setup: create poll action", async ({ page }) => {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-poll" });
    await page.waitForURL(/\/actions\/polls\//, {
      waitUntil: "load",
      timeout: 60000,
    });
  });

  test("Setup: create discussion action", async ({ page }) => {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-discuss" });
    await page.waitForURL(/\/actions\/discussions\/[^/]+\/edit/, {
      waitUntil: "load",
      timeout: 60000,
    });
  });

  test("Setup: create quiz action", async ({ page }) => {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-quiz" });
    await page.waitForURL(/\/actions\/quizzes\//, {
      waitUntil: "load",
      timeout: 60000,
    });
  });

  test("Setup: create follow action", async ({ page }) => {
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-follow" });
    await page.waitForURL(/\/actions\/follows\//, {
      waitUntil: "load",
      timeout: 60000,
    });
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
