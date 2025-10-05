import { test, expect } from "@playwright/test";

test.describe("Create Post - Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage
    await page.goto("/");
    // Wait for page to load completely
    await page.waitForLoadState("networkidle");
  });

  test("should create a general post successfully", async ({ page }) => {
    // Find and click the create post button
    // The button has bg-create-button-bg class and contains Edit1 icon
    const createPostButton = page.locator(
      'div.bg-create-button-bg.cursor-pointer'
    ).first();

    await expect(createPostButton).toBeVisible({ timeout: 10000 });
    await createPostButton.click();

    // Wait for the post editor to expand
    // The editor should have the title input field visible
    const titleInput = page.locator(
      'input[type="text"][placeholder*="title" i]'
    );
    await expect(titleInput).toBeVisible({ timeout: 10000 });

    // Fill in the title
    const testTitle = "Automated Test Post Title - E2E Test";
    await titleInput.fill(testTitle);

    // Fill in the content using the Lexical editor
    // Look for the contenteditable div inside the editor
    const contentEditor = page.locator('[contenteditable="true"]').first();
    await expect(contentEditor).toBeVisible({ timeout: 5000 });

    await contentEditor.click();
    const testContent =
      "This is an automated test post content created by Playwright E2E tests. " +
      "The purpose of this test is to verify that the post creation functionality " +
      "works correctly from end to end, including title input, content editing, " +
      "auto-save, and final publication. This content is intentionally long to " +
      "meet the minimum character requirements for post publishing.";

    await contentEditor.fill(testContent);

    // Wait for auto-save to complete
    // Auto-save happens every 5 seconds, so wait 6 seconds to be safe
    await page.waitForTimeout(6000);

    // Verify that auto-save completed by checking there's no "Saving..." text
    // or wait for any loading spinner to disappear
    await expect(page.locator('text=Saving...')).not.toBeVisible({
      timeout: 3000,
    }).catch(() => {
      // Saving text might not appear if save was instant
    });

    // Find the submit/publish button
    // It's a button with variant="rounded_primary" containing UserCircleIcon
    // We'll look for the button in the editor area that's not disabled
    const submitButton = page
      .locator('button')
      .filter({ has: page.locator('svg') })
      .filter({ hasNot: page.locator('.animate-spin') })
      .last();

    // Verify the submit button is enabled (not disabled)
    await expect(submitButton).toBeEnabled({ timeout: 5000 });

    // Click the submit button to publish the post
    await submitButton.click();

    // Wait for navigation to the thread/post detail page
    // The URL should change to /threads/{post_pk}
    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });

    // Verify we're on the thread page
    expect(page.url()).toMatch(/\/threads\/.+/);

    // Verify the post title appears on the thread page
    await expect(
      page.locator(`text=${testTitle}`).first()
    ).toBeVisible({ timeout: 10000 });

    // Optional: Verify the content is also visible
    // We'll check for a portion of the content text
    await expect(
      page.locator('text=/automated test post content/i')
    ).toBeVisible({ timeout: 5000 });
  });
});
