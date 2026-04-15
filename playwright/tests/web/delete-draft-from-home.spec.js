import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor } from "../utils";

/**
 * Delete Draft from Home Page
 *
 * After the home-ui renewal, the home page no longer inlines the drafts
 * timeline. Drafts now live on a dedicated `/{username}/drafts` page, reached
 * from the home arena top bar. Each draft card exposes a three-dot "More
 * options" menu whose "Delete draft" item deletes the draft immediately.
 *
 * Flow:
 *   1. Create a draft via the home "Create" button (autosave to draft)
 *   2. Click the home "Drafts" button → lands on /{username}/drafts
 *   3. Open the draft card's dots menu, click "Delete draft"
 *   4. Verify the card is removed from the list
 *
 * NOTE: The current UI deletes the draft immediately — there is no confirm
 *       popup. Re-add a confirm-and-cancel step once that popup ships.
 *
 * Requires the backend built with `--features bypass` so the authenticated
 * user session is available via user.json.
 */

test.describe.serial("Delete draft from home page", () => {
  let draftTitle;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;

  // ─── 1. Create a draft post ────────────────────────────────────────────────

  test("should create a draft post via the editor", async ({ page }) => {
    draftTitle = `E2E Draft ${uniqueId}`;

    await goto(page, "/");

    // Open the post editor from the home arena top bar
    await click(page, { testId: "home-btn-create" });

    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    await fill(page, { placeholder: "Title your post…" }, draftTitle);

    const editor = await getEditor(page);
    await editor.fill("Draft content for deletion test");

    // Trigger autosave by blurring the editor; editor debounces ~3s
    await page.keyboard.press("Tab");

    await expect(page.getByText("All changes saved")).toBeVisible({
      timeout: 10000,
    });
  });

  // ─── 2. Open the drafts page from home ─────────────────────────────────────

  test("should open the drafts page from the home Drafts button", async ({
    page,
  }) => {
    await goto(page, "/");

    await click(page, { testId: "home-btn-drafts" });

    // Drafts route is /{username}/drafts
    await page.waitForURL(/\/[^/]+\/drafts$/, { waitUntil: "load" });

    const draftCard = page.locator(".draft-card", { hasText: draftTitle });
    await expect(draftCard).toBeVisible();
  });

  // ─── 3. Delete the draft via the dots menu ─────────────────────────────────

  test("should delete the draft via the dots context menu", async ({
    page,
  }) => {
    await goto(page, "/");
    await click(page, { testId: "home-btn-drafts" });
    await page.waitForURL(/\/[^/]+\/drafts$/, { waitUntil: "load" });

    const draftCard = page.locator(".draft-card", { hasText: draftTitle });
    await expect(draftCard).toBeVisible();

    // Open the dots menu for this card
    await draftCard.getByLabel("More options", { exact: true }).click();

    // Menu-open state is reflected on the card via data-menu-open
    await expect(draftCard).toHaveAttribute("data-menu-open", "true");

    // Click the "Delete draft" action inside the menu (immediate delete)
    await draftCard
      .getByRole("button", { name: "Delete draft", exact: true })
      .click();

    // Card with our title should disappear from the page
    await expect(
      page.locator(".draft-card", { hasText: draftTitle }),
    ).toHaveCount(0, { timeout: 10000 });
  });
});
