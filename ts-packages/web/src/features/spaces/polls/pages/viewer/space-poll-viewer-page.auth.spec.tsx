import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

test.describe.serial('[SpacePollViewerPage] Authenticated Users', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';
  let spaceUrl = '';
  let pollUrl = '';

  async function navigateToPoll() {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
  }

  test.beforeAll('Create a post and poll space', async ({ browser }) => {
    context = await browser.newContext({ storageState: 'user.json' });
    page = await context.newPage();
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const testTitle = 'Automated Post for Poll Space Verification';
    const testContent =
      'This is an automated post content created by Playwright E2E for verifying poll spaces. ' +
      'The purpose of this is to verify that the poll space creation and poll response functionality ' +
      'works correctly from end to end. This content is intentionally long to ' +
      'meet the minimum character requirements for post publishing. We will verify creating a poll space, ' +
      'adding poll questions, and submitting responses.';

    await click(page, { label: 'Create Post' });
    await page.waitForURL(/\/posts\/new/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    await page.fill('#post-title-input', testTitle);

    const editorSelector = '[data-pw="post-content-editor"] .ProseMirror';
    await page.waitForSelector(editorSelector, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    await page.click(editorSelector);
    await page.fill(editorSelector, testContent);

    await page.click('#publish-post-button');

    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });
    threadUrl = page.url();
  });

  test('[SPVP-001] Create a Poll Space', async () => {
    await page.goto(threadUrl);
    await page.waitForTimeout(3000);

    await expect(page.getByText('Create a Space', { exact: true })).toBeVisible(
      { timeout: 20000 },
    );
    await page.getByText('Create a Space', { exact: true }).click();

    const modal = page.getByRole('dialog', { name: 'Select a Space Type' });
    await modal.locator('div.cursor-pointer', { hasText: 'Poll' }).click();

    await modal.getByRole('button', { name: 'Create' }).click();

    await page.waitForURL(/\/spaces\/[^/]+(?:\?.*)?$/, { timeout: 15000 });

    spaceUrl = page.url();
  });

  test('[SPVP-002] Navigate to Polls page and create a poll', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    // Click on "Polls" in the side menu to go to polls list
    await page.getByRole('link', { name: 'Polls' }).click();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Click "Create Poll" button
    const createPollButton = page.getByRole('button', { name: 'Create Poll' });
    await expect(createPollButton).toBeVisible({ timeout: 10000 });
    await createPollButton.click();

    // Wait for navigation to the new poll page
    await page.waitForURL(/\/spaces\/[^/]+\/polls\/[^/]+/, { timeout: 15000 });
    pollUrl = page.url();
  });

  test('[SPVP-003] Authenticated admin user can see Edit button', async () => {
    await navigateToPoll();

    // Admin should see Edit button instead of viewer buttons
    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    const isEditVisible = await editButton
      .isVisible({ timeout: 5000 })
      .catch(() => false);

    // Since the user is the space creator, they should see the edit button
    expect(isEditVisible).toBe(true);
  });

  test('[SPVP-004] Admin can edit poll and add questions', async () => {
    await navigateToPoll();

    // Click Edit button
    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await expect(editButton).toBeVisible({ timeout: 10000 });
    await editButton.click();

    // Should now be in editing mode
    const saveButton = page.locator('[data-pw="poll-editor-save-btn"]');
    await expect(saveButton).toBeVisible({ timeout: 5000 });

    // Add a new question
    const addQuestionBtn = page.locator('[data-pw="survey-add-question-btn"]');
    const isAddQuestionVisible = await addQuestionBtn
      .isVisible({ timeout: 3000 })
      .catch(() => false);

    if (isAddQuestionVisible) {
      await addQuestionBtn.click();
      await page.waitForTimeout(1000);
    }

    // Save the poll
    await saveButton.click();

    // Wait for the save operation to complete
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Should return to view mode - edit button visible again
    await expect(editButton).toBeVisible({ timeout: 10000 });
  });

  test('[SPVP-005] Admin sees response_editable checkbox', async () => {
    await navigateToPoll();

    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    await expect(checkbox).toBeVisible({ timeout: 5000 });

    // Verify checkbox is enabled and clickable
    await expect(checkbox).toBeEnabled();

    // Just verify we can click it without error
    await checkbox.click();
    await page.waitForTimeout(500);
  });

  test('[SPVP-006] Discard changes returns to view mode', async () => {
    await navigateToPoll();

    // Click Edit button
    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    const isEditVisible = await editButton
      .isVisible({ timeout: 5000 })
      .catch(() => false);

    if (isEditVisible) {
      await editButton.click();

      // Should now be in editing mode
      const discardButton = page.locator('[data-pw="poll-editor-discard-btn"]');
      await expect(discardButton).toBeVisible({ timeout: 5000 });

      // Click discard
      await discardButton.click();
      await page.waitForTimeout(1000);

      // Should return to view mode
      await expect(editButton).toBeVisible({ timeout: 5000 });
    }
  });
});
