import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

test.describe.serial('[SpacePollEditorPage] Authenticated Users', () => {
  let threadUrl = '';
  let spaceUrl = '';
  let pollUrl = '';

  test('[SPEP-001] Create a post for poll space', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // Wait for sidebar to load

    const testTitle = 'Automated Post for Poll Editor Testing';
    const testContent =
      'This is an automated post content created by Playwright E2E for verifying poll editor functionality. ' +
      'The purpose of this is to verify that the poll space creation, question editing, and admin features ' +
      'work correctly from end to end. This content is intentionally long to meet the minimum character ' +
      'requirements for post publishing. We will verify creating a poll space, editing questions, toggling ' +
      'response_editable, and saving changes.';

    // Try data-pw first, then fall back to label
    await click(page, { 'data-pw': 'create-post-button' });
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

  test('[SPEP-002] Create a Poll Space', async ({ page }) => {
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

  test('[SPEP-003] Navigate to Polls page and create a poll', async ({
    page,
  }) => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByRole('link', { name: 'Polls' }).click();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const createPollButton = page.getByRole('button', { name: 'Create Poll' });
    await expect(createPollButton).toBeVisible({ timeout: 10000 });
    await createPollButton.click();

    await page.waitForURL(/\/spaces\/[^/]+\/polls\/[^/]+/, { timeout: 15000 });
    pollUrl = page.url();
  });

  test('[SPEP-004] Response editable checkbox is visible for admin', async ({
    page,
  }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    await expect(checkbox).toBeVisible({ timeout: 10000 });
  });

  test('[SPEP-005] Edit button is visible for admin', async ({ page }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await expect(editButton).toBeVisible({ timeout: 10000 });
  });

  test('[SPEP-006] Clicking Edit shows Save and Discard buttons', async ({
    page,
  }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await editButton.click();
    await page.waitForTimeout(1000);

    const saveButton = page.locator('[data-pw="poll-editor-save-btn"]');
    const discardButton = page.locator('[data-pw="poll-editor-discard-btn"]');

    await expect(saveButton).toBeVisible({ timeout: 10000 });
    await expect(discardButton).toBeVisible({ timeout: 10000 });
  });

  test('[SPEP-007] Back button returns to polls list', async ({ page }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const backButton = page.getByRole('button', { name: /back/i });
    await expect(backButton).toBeVisible({ timeout: 10000 });
    await backButton.click();

    await page.waitForURL(/\/spaces\/[^/]+\/polls$/, { timeout: 10000 });
  });
});
