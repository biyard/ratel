import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor, waitPopup } from "../utils";

/**
 * Delete Draft from Home Page
 *
 * This scenario tests the draft deletion feature on the home page timeline.
 * It exercises:
 *   1. Navigate to the home page (authenticated)
 *   2. Create a draft post (navigate to editor, fill title, go back)
 *   3. Verify the draft appears in the "Drafts" section on the home page
 *   4. Click the delete (trash) icon on the draft card
 *   5. Verify the confirmation popup appears with correct text
 *   6. Cancel the deletion and verify the draft is still present
 *   7. Click delete again and confirm the deletion
 *   8. Verify the draft is removed from the list
 *
 * NOTE: Requires backend built with `--features bypass` so that
 *       authenticated user session is available via user.json.
 */

test.describe.serial("Delete draft from home page", () => {
  let draftTitle;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;

  // ─── 1. Create a draft post ────────────────────────────────────────────────

  test("should create a draft post via the editor", async ({ page }) => {
    draftTitle = `E2E Draft ${uniqueId}`;

    await goto(page, "/");

    // Navigate to post creation — click "Create Post" button
    await click(page, { text: "Create Post" });

    // Wait for post editor to load
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    // Fill in the title to create a draft (autosave triggers on blur)
    await fill(page, { placeholder: "Title" }, draftTitle);

    // Type some content so the draft is meaningful
    const editor = await getEditor(page);
    await editor.fill("Draft content for deletion test");

    // Trigger autosave by blurring the editor
    await page.keyboard.press("Tab");

    // Wait briefly for autosave to complete
    await page.waitForTimeout(2000);
  });

  // ─── 2. Verify draft appears on home page ──────────────────────────────────

  test("should show the draft in the Drafts section on home page", async ({
    page,
  }) => {
    await goto(page, "/");

    // The Drafts section should be visible with aria-label
    const draftsSection = page.locator('[aria-label="Drafts section"]');
    await expect(draftsSection).toBeVisible();

    // The draft title should appear within the section
    await expect(draftsSection.getByText(draftTitle)).toBeVisible();
  });

  // ─── 3. Cancel draft deletion ──────────────────────────────────────────────

  test("should show confirmation popup and cancel deletion", async ({
    page,
  }) => {
    await goto(page, "/");

    const draftsSection = page.locator('[aria-label="Drafts section"]');
    await expect(draftsSection).toBeVisible();

    // Find the draft card containing our title
    const draftCard = draftsSection.locator(`text=${draftTitle}`).locator("../..");

    // Hover over the card to reveal the delete button
    await draftCard.hover();

    // Click the delete button (aria-label="Delete draft")
    const deleteButton = draftCard.getByLabel("Delete draft");
    await deleteButton.click();

    // Confirmation popup should appear
    await waitPopup(page, { visible: true });

    // Verify popup content
    await getLocator(page, { text: "Delete Draft" });
    await getLocator(page, {
      text: "Are you sure you want to delete this draft? This action cannot be undone.",
    });

    // Click Cancel
    await click(page, { text: "Cancel" });

    // Popup should close
    await waitPopup(page, { visible: false });

    // Draft should still be visible
    await expect(draftsSection.getByText(draftTitle)).toBeVisible();
  });

  // ─── 4. Confirm draft deletion ─────────────────────────────────────────────

  test("should delete the draft when confirmed", async ({ page }) => {
    await goto(page, "/");

    const draftsSection = page.locator('[aria-label="Drafts section"]');
    await expect(draftsSection).toBeVisible();

    // Find the draft card containing our title
    const draftCard = draftsSection.locator(`text=${draftTitle}`).locator("../..");

    // Hover over the card to reveal the delete button
    await draftCard.hover();

    // Click the delete button
    const deleteButton = draftCard.getByLabel("Delete draft");
    await deleteButton.click();

    // Confirmation popup should appear
    await waitPopup(page, { visible: true });

    // Click Confirm to delete
    await click(page, { text: "Confirm" });

    // Popup should close
    await waitPopup(page, { visible: false });

    // The draft title should no longer be visible
    await expect(page.getByText(draftTitle)).toBeHidden();
  });
});
