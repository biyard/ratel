import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor } from "../utils";

/**
 * Dungeon Hero Smoke Test — Phase 3
 *
 * Verifies the persistent Dungeon Hero band and the /spaces/{id} →
 * /spaces/{id}/actions redirect flip from Phase 3 of the gamification roadmap.
 *
 * Flow:
 *   1. Creator creates a team + post + space.
 *   2. Navigate to /spaces/{id}  (no trailing path) and assert the URL
 *      becomes /spaces/{id}/actions — the new default landing page.
 *   3. Assert data-testid="dungeon-hero" is visible.
 *   4. Assert data-testid="xp-hud-level" renders a number (the starting level).
 */

test.describe.serial("Dungeon Hero smoke", () => {
  let spaceUrl;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const teamNickname = `DH Team ${uniqueId}`;
  const teamUsername = `dh_${uniqueId}`;
  const postTitle = `Dungeon Hero Smoke ${uniqueId}`;
  const postContents =
    "Smoke test space for the Phase 3 Dungeon Hero band. " +
    "This space only exists to verify the persistent hero header and the " +
    "default landing redirect from /spaces/{id} to /spaces/{id}/actions.";

  test("Creator: create team + post + space", async ({ page }) => {
    await goto(page, "/");

    await click(page, { label: "User Profile" });
    await click(page, { text: "Create Team" });

    await page.locator('[data-testid="team-nickname-input"]').fill(teamNickname);
    await page.locator('[data-testid="team-username-input"]').fill(teamUsername);
    await page
      .locator('[data-testid="team-description-input"]')
      .fill("Phase 3 dungeon hero smoke team");
    await click(page, { text: "Create" });

    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "load",
    });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null
    );

    await click(page, { text: "Create" });
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    await fill(page, { placeholder: "Title" }, postTitle);
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(postContents);

    await click(page, { text: "Go to Space" });

    // Phase 3 flipped the default redirect: /spaces/{id} now lands on
    // /actions instead of /dashboard. The initial navigation from the
    // post editor still ends on an explicit sub-page, so capture the
    // base space URL here for the next step.
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/(dashboard|actions)/, {
      waitUntil: "load",
    });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/(dashboard|actions)$/, "");
  });

  test("Creator: /spaces/{id} redirects to /actions and renders Dungeon Hero", async ({
    page,
  }) => {
    await goto(page, spaceUrl);

    // Phase 3: the root space URL should redirect to the actions sub-page.
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/actions/, {
      waitUntil: "load",
    });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null
    );

    // The persistent Dungeon Hero band renders above the Outlet.
    const hero = page.getByTestId("dungeon-hero");
    await expect(hero).toBeVisible();

    // The XP HUD level tile renders a numeric level (starting users begin at 1).
    const levelTile = page.getByTestId("xp-hud-level");
    await expect(levelTile).toBeVisible();
    await expect(levelTile).toContainText(/\d+/);
  });
});
