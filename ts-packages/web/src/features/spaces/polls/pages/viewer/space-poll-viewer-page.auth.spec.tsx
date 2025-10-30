import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

test.describe.serial('[SpacePollViewerPage] Authenticated Users', () => {
  let threadUrl = '';
  let spaceUrl = '';
  let pollUrl = '';

  test('[SPVP-001] Create a post for poll space', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // Wait for sidebar to load

    const testTitle = 'Automated Post for Poll Viewer Testing';
    const testContent =
      'This is an automated post content created by Playwright E2E for verifying poll viewer functionality. ' +
      'The purpose of this is to verify that the poll space creation and poll response functionality ' +
      'works correctly from end to end. This content is intentionally long to ' +
      'meet the minimum character requirements for post publishing. We will verify creating a poll space, ' +
      'adding poll questions, and submitting responses.';

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

  test('[SPVP-002] Create a Poll Space', async ({ page }) => {
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

  test('[SPVP-003] Navigate to Polls page and create a poll', async ({
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

  test('[SPVP-004] Admin user sees Edit button initially', async ({ page }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await expect(editButton).toBeVisible({ timeout: 10000 });
  });

  test('[SPVP-005] Poll page loads correctly', async ({ page }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    expect(page.url()).toBe(pollUrl);
  });

  test('[SPVP-006] Back button returns to polls list', async ({ page }) => {
    await page.goto(pollUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const backButton = page.getByRole('button', { name: /back/i });
    await expect(backButton).toBeVisible({ timeout: 10000 });
    await backButton.click();

    await page.waitForURL(/\/spaces\/[^/]+\/polls$/, { timeout: 10000 });
  });
});
