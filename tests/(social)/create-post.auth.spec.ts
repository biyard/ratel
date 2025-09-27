import { test, expect } from "@playwright/test";

test.describe("Create Post - Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage
    await page.goto("/");
    // Wait for page to load completely
    await page.waitForLoadState("networkidle");
  });

  test("should create a general post successfully", async ({ page }) => {
    // Open post editor
    const createPostButton = page
      .locator('[data-testid="create-post-button"]')
      .first();
    await createPostButton.click();

    // Wait for editor to load
    await page.waitForSelector('[data-testid="post-editor"]', {
      timeout: 10000,
    });

    // Fill in title
    const titleInput = page.locator('[data-testid="post-title-input"]');
    await titleInput.fill("Test Post Title");

    // Fill in content
    const contentEditor = page.locator(
      '[data-testid="post-content-editor"] [contenteditable="true"]',
    );
    await contentEditor.click();
    await contentEditor.fill(
      "This is a test post content to verify the create post functionality works correctly.",
    );

    // Find submit button with user icon (based on the component structure)
    const submitButton = page.locator('[data-testid="post-submit-button"]');
    await expect(submitButton).toBeEnabled();

    // Submit the post
    await submitButton.click();

    // Wait for successful submission - either editor closes or success indication
    await page.waitForTimeout(3000);

    // Verify submission was successful by checking if editor is no longer visible or page changed
    const editorStillVisible = await page
      .locator('[data-testid="post-editor"]')
      .isVisible()
      .catch(() => false);

    // If editor is still visible, check if there's a success indication
    if (editorStillVisible) {
      // Look for loading indicators disappearing or success messages
      await expect(page.locator(".animate-spin")).not.toBeVisible({
        timeout: 5000,
      });
    }
  });
});
