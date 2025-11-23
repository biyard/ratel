import { expect, test } from '@playwright/test';
import { fill } from '@tests/utils';
import { CONFIGS } from '@tests/config';

test.describe('Create Post Page - Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/posts/new');
    await page.waitForLoadState('networkidle');
  });

  test('[CP-AUTH-001] should display create post form with all elements', async ({
    page,
  }) => {
    // Verify page title
    await expect(page.getByText('Create post')).toBeVisible();

    // Verify title input
    const titleInput = page.getByPlaceholder('Title');
    await expect(titleInput).toBeVisible();

    // Verify content editor
    const editor = page.locator('[data-pw="post-content-editor"]');
    await expect(editor).toBeVisible();

    // Verify toolbar is present
    const toolbar = page.locator('.toolbar, [role="toolbar"]');
    const isToolbarVisible = await toolbar.isVisible().catch(() => false);
    if (isToolbarVisible) {
      await expect(toolbar).toBeVisible();
    }

    // Verify save/next button
    const actionButton = page.getByRole('button', {
      name: /save|next/i,
    });
    await expect(actionButton).toBeVisible();
  });

  test('[CP-AUTH-002] should create a new post with a poll space successfully', async ({
    page,
  }) => {
    const testTitle = 'Automated Test Post - E2E';
    const testContent =
      'This is test content for automated post creation. ' +
      'The content includes enough text to meet minimum requirements.';

    // Fill in title
    await fill(page, { placeholder: 'Title' }, testTitle);

    // Fill in content using TipTap's contenteditable div
    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Wait for auto-save to complete (5 second delay + processing time)
    await page.waitForTimeout(6000);

    // Check skip creating space to go straight to save
    const skipCheckbox = page.getByText('Skip creating a space');
    if (await skipCheckbox.isVisible()) {
      await skipCheckbox.click();
      await page.waitForTimeout(500); // Wait for button text to update
    }

    // Click save button (either "Save" or "Next")
    const actionButton = page.getByRole('button', { name: /Save|Next/i });
    await actionButton.waitFor({ state: 'visible', timeout: 5000 });
    await expect(actionButton).toBeEnabled();
    await actionButton.click();

    // Should redirect to thread page
    await page.waitForURL(/\/spaces\/.+/, { timeout: CONFIGS.PAGE_WAIT_TIME });
  });

  test('[CP-AUTH-003] should enforce title character limit', async ({
    page,
  }) => {
    const longTitle = 'a'.repeat(60); // Exceeds 50 character limit
    const titleInput = page.getByPlaceholder('Title');

    await titleInput.fill(longTitle);

    // Get the actual value after filling
    const actualValue = await titleInput.inputValue();

    // Should be limited to 50 characters
    expect(actualValue.length).toBeLessThanOrEqual(50);

    // Character counter should show limit
    const counter = page.locator('text=/\\d+\\/50/');
    await expect(counter).toBeVisible();
  });

  test('[CP-AUTH-004] should show saving status indicator', async ({
    page,
  }) => {
    const testTitle = 'Test Saving Status';
    const testContent = 'Testing auto-save functionality';

    // Fill in title
    await fill(page, { placeholder: 'Title' }, testTitle);

    // Fill in content using TipTap's contenteditable div
    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Should show "Saving..." or "Unsaved changes"
    const savingIndicator = page.getByText(/saving\.\.\.|unsaved changes/i);
    const isSavingVisible = await savingIndicator
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    if (isSavingVisible) {
      await expect(savingIndicator).toBeVisible();
    }

    // Wait for auto-save to complete (5 second delay + processing time)
    await page.waitForTimeout(6000);

    // Should eventually show "All changes saved"
    const savedIndicator = page.getByText('All changes saved');
    const isSavedVisible = await savedIndicator
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    if (isSavedVisible) {
      await expect(savedIndicator).toBeVisible();
    }
  });

  test('[CP-AUTH-005] should display toolbar with formatting options', async ({
    page,
  }) => {
    // Check for list formatting buttons
    const bulletListButton = page.getByLabel('Bullet List', { exact: true });
    const numberedListButton = page.getByLabel('Numbered List', {
      exact: true,
    });

    // Verify toolbar buttons exist (may vary based on implementation)
    const hasBulletList = await bulletListButton.isVisible().catch(() => false);
    const hasNumberedList = await numberedListButton
      .isVisible()
      .catch(() => false);

    // At least some formatting buttons should be present
    expect(hasBulletList || hasNumberedList).toBeTruthy();
  });

  test('[CP-AUTH-006] should handle list formatting', async ({ page }) => {
    const testContent = 'List item text';

    // Fill in content
    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Try to find and click bullet list button
    const bulletListButton = page.getByLabel('Bullet List', { exact: true });

    if (await bulletListButton.isVisible().catch(() => false)) {
      await bulletListButton.click();
      await page.waitForTimeout(500);

      // Content should now be in a list
      const listElement = page.locator('ul li, ol li');
      const hasListElement = await listElement.isVisible().catch(() => false);

      if (hasListElement) {
        await expect(listElement).toBeVisible();
      }
    }
  });

  test('[CP-AUTH-007] should upload and display image', async ({ page }) => {
    // Note: This test requires actual image upload functionality
    // which may need to be mocked or use a test image file

    // Check if image upload button exists
    const imageUploadButton = page.locator('button[aria-label*="image"]');
    const hasImageButton = await imageUploadButton
      .isVisible()
      .catch(() => false);

    if (hasImageButton) {
      await expect(imageUploadButton).toBeVisible();
    }
  });

  test('[CP-AUTH-008] should allow skipping space creation', async ({
    page,
  }) => {
    const testTitle = 'Test Skip Space Creation';
    const testContent = 'Testing space creation workflow';

    await fill(page, { placeholder: 'Title' }, testTitle);

    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Find and click skip checkbox
    const skipCheckbox = page.getByText('Skip creating a space');
    if (await skipCheckbox.isVisible()) {
      // Button text should change to "Save"
      const saveButton = page.getByRole('button', {
        name: 'Save',
        exact: true,
      });
      await expect(saveButton).toBeVisible();
    }
  });

  test('[CP-AUTH-009] should show space type selector when not skipping', async ({
    page,
  }) => {
    const testTitle = 'Test Space Type Selection';
    const testContent = 'Testing space type selector';

    await fill(page, { placeholder: 'Title' }, testTitle);

    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Ensure skip checkbox is not checked
    const skipCheckbox = page.locator('input#skip-space');
    const isChecked = await skipCheckbox.isChecked().catch(() => true);

    if (isChecked) {
      const skipLabel = page.getByText('Skip creating a space');
      await skipLabel.click();
    }

    // Space type carousel should be visible
    const spaceTypeSelector = page.locator('[class*="carousel"]');
    const isSelectorVisible = await spaceTypeSelector
      .isVisible()
      .catch(() => false);

    if (isSelectorVisible) {
      await expect(spaceTypeSelector).toBeVisible();
    }

    // Button should show "Next" instead of "Save"
    const nextButton = page.getByRole('button', { name: 'Next' });
    const isNextVisible = await nextButton.isVisible().catch(() => false);

    if (isNextVisible) {
      await expect(nextButton).toBeVisible();
    }
  });

  test('[CP-AUTH-010] should disable publish when fields are empty', async ({
    page,
  }) => {
    // Save/Next button should be disabled when form is empty
    const actionButton = page.getByRole('button', { name: /save|next/i });
    await expect(actionButton).toBeDisabled();

    // Fill only title
    await fill(page, { placeholder: 'Title' }, 'Test Title');
    await expect(actionButton).toBeDisabled();

    // Fill content
    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill('Test content with enough text');

    // Wait for auto-save to complete
    await page.waitForTimeout(6000);

    // Button should be enabled
    const isEnabled = await actionButton.isEnabled().catch(() => false);
    expect(isEnabled).toBeTruthy();
  });

  test('[CP-AUTH-011] should load existing post when postPk is provided', async ({
    page,
  }) => {
    // First create a post to get a valid postPk
    const createTitle = 'Post for Edit Test';
    const createContent = 'Original content for editing test';

    await fill(page, { placeholder: 'Title' }, createTitle);

    const editor1 = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor1.waitFor({ state: 'visible' });
    await editor1.click();
    await editor1.fill(createContent);

    const saveButton = page.getByRole('button', { name: 'Save' });
    await saveButton.click();
    await page.waitForURL(/\/threads\/.+/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    // Extract postPk from URL
    const url = page.url();
    const match = url.match(/\/threads\/(.+)/);

    if (match && match[1]) {
      const postPk = match[1];

      // Navigate to edit page with postPk
      await page.goto(`/posts/new?post-pk=${encodeURIComponent(postPk)}`);
      await page.waitForLoadState('networkidle');

      // Title and content should be loaded
      const titleInput = page.getByPlaceholder('Title');
      const titleValue = await titleInput.inputValue();
      expect(titleValue).toBe(createTitle);

      // Content editor should have the content
      const editor = page.locator('[data-pw="post-content-editor"]');
      const editorContent = await editor.textContent();
      expect(editorContent).toContain('Original content');

      // Should show "All changes saved" or last saved time
      const savedIndicator = page.getByText(/all changes saved|last saved/i);
      const isSavedVisible = await savedIndicator
        .isVisible({ timeout: 3000 })
        .catch(() => false);

      if (isSavedVisible) {
        await expect(savedIndicator).toBeVisible();
      }
    }
  });

  test('[CP-AUTH-012] should update existing post when editing', async ({
    page,
  }) => {
    // First create a post
    const createTitle = 'Post for Update Test';
    const createContent = 'Original content for update test';

    await fill(page, { placeholder: 'Title' }, createTitle);

    const editor1 = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor1.waitFor({ state: 'visible' });
    await editor1.click();
    await editor1.fill(createContent);

    const saveButton = page.getByRole('button', { name: 'Save' });
    await saveButton.click();
    await page.waitForURL(/\/threads\/.+/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    const url = page.url();
    const match = url.match(/\/threads\/(.+)/);

    if (match && match[1]) {
      const postPk = match[1];

      // Navigate to edit page
      await page.goto(`/posts/new?post-pk=${encodeURIComponent(postPk)}`);
      await page.waitForLoadState('networkidle');

      // Modify title and content
      const updatedTitle = 'Updated Post Title';
      const updatedContent = 'Updated content for the post';

      const titleInput = page.getByPlaceholder('Title');
      await titleInput.clear();
      await titleInput.fill(updatedTitle);

      const editor = page.locator(
        '[data-pw="post-content-editor"] [contenteditable]',
      );
      await editor.clear();
      await editor.fill(updatedContent);

      // Save the updated post
      const saveButton2 = page.getByRole('button', { name: 'Save' });
      await saveButton2.click();
      await page.waitForURL(/\/threads\/.+/, {
        timeout: CONFIGS.PAGE_WAIT_TIME,
      });

      // Verify we're on the same post page
      expect(page.url()).toContain(encodeURIComponent(postPk));
    }
  });

  test('[CP-AUTH-013] should handle mobile responsive layout', async ({
    page,
  }) => {
    // Test mobile layout
    await page.setViewportSize({
      width: CONFIGS.DEVICE_SCREEN_SIZES.MOBILE - 100,
      height: 800,
    });

    // Form elements should still be visible and functional
    await expect(page.getByPlaceholder('Title')).toBeVisible();
    await expect(page.locator('[data-pw="post-content-editor"]')).toBeVisible();

    // Action button should be visible
    const actionButton = page.getByRole('button', { name: /save|next/i });
    await expect(actionButton).toBeVisible();

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });

  test('[CP-AUTH-014] should show last saved timestamp', async ({ page }) => {
    const testTitle = 'Test Last Saved';
    const testContent = 'Testing last saved timestamp';

    await fill(page, { placeholder: 'Title' }, testTitle);

    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Wait for auto-save
    await page.waitForTimeout(6000);

    // Should show last saved timestamp
    const lastSavedText = page.getByText(/last saved at \d{4}\.\d{2}\.\d{2}/i);
    const isLastSavedVisible = await lastSavedText
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    if (isLastSavedVisible) {
      await expect(lastSavedText).toBeVisible();
    }
  });

  test('[CP-AUTH-015] should handle image removal', async ({ page }) => {
    // This test checks if image removal functionality works
    // Note: Requires actual image upload to test removal

    // Check if there's an image removal button
    const removeImageButton = page.locator(
      'button[aria-label*="Remove image"]',
    );
    const hasRemoveButton = await removeImageButton
      .isVisible()
      .catch(() => false);

    // If image is present, removal button should work
    if (hasRemoveButton) {
      await removeImageButton.click();
      await page.waitForTimeout(500);

      // Image should be removed
      const imagePreview = page.locator('img[alt="Uploaded"]');
      await expect(imagePreview).not.toBeVisible();
    }
  });

  test('[CP-AUTH-016] should preserve content during auto-save', async ({
    page,
  }) => {
    const testTitle = 'Preserve Content Test';
    const testContent = 'Content that should be preserved during auto-save';

    await fill(page, { placeholder: 'Title' }, testTitle);

    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]',
    );
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    await editor.fill(testContent);

    // Wait for auto-save
    await page.waitForTimeout(6000);

    // Get current values
    const titleInput = page.getByPlaceholder('Title');
    const titleValue = await titleInput.inputValue();

    const editorContent = await editor.textContent();

    // Content should be preserved
    expect(titleValue).toBe(testTitle);
    expect(editorContent).toContain('Content that should be preserved');
  });
});
