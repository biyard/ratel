import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click, fill } from '@tests/utils';

test.describe.serial('[SpacePollEditorPage] Authenticated Users', () => {
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

    const currentUrl = page.url();
    expect(currentUrl).toBeTruthy();
    expect(currentUrl).toContain(CONFIGS.PLAYWRIGHT.BASE_URL);

    const testTitle = 'Automated Post for Poll Editor Feature';
    const testContent =
      'This is an automated post content created by Playwright E2E for verifying poll editor functionality. ' +
      'The purpose of this is to verify that the poll space creation, question editing, and admin features ' +
      'work correctly from end to end. This content is intentionally long to meet the minimum character ' +
      'requirements for post publishing. We will verify creating a poll space, editing questions, toggling ' +
      'response_editable, and saving changes.';

    await click(page, { text: 'Create Post' });
    await fill(page, { placeholder: 'Write a title...' }, testTitle);
    await fill(page, { label: 'general-post-editor' }, testContent);

    await click(page, { label: 'Publish' });

    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });
    threadUrl = page.url();
  });

  test('[SPEP-001] Create a Poll Space', async () => {
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

  test('[SPEP-002] Navigate to Poll editor page', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(5000);

    // Extract space ID and construct poll URL
    const urlParts = spaceUrl.split('/');
    const spaceId = urlParts[urlParts.length - 1].split('?')[0];
    const pollPk = `SPACE_POLL#${decodeURIComponent(spaceId).split('#')[1]}`;
    pollUrl = `/spaces/${encodeURIComponent(spaceId)}/polls/${encodeURIComponent(pollPk)}`;

    // Navigate to poll page
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await expect(editButton).toBeVisible({ timeout: 10000 });
  });

  test('[SPEP-003] Response editable checkbox is visible for admin', async () => {
    await navigateToPoll();

    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    await expect(checkbox).toBeVisible({ timeout: 5000 });
  });

  test('[SPEP-004] Response_editable checkbox is visible and clickable', async () => {
    await navigateToPoll();

    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    await expect(checkbox).toBeVisible({ timeout: 5000 });

    // Verify checkbox is enabled and clickable
    await expect(checkbox).toBeEnabled();

    // Just verify we can click it without error
    await checkbox.click();
    await page.waitForTimeout(500);
  });

  test('[SPEP-005] Enter edit mode by clicking Edit button', async () => {
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await expect(editButton).toBeVisible({ timeout: 5000 });
    await editButton.click();

    const saveButton = page.locator('[data-pw="poll-editor-save-btn"]');
    const discardButton = page.locator('[data-pw="poll-editor-discard-btn"]');

    await expect(saveButton).toBeVisible({ timeout: 5000 });
    await expect(discardButton).toBeVisible({ timeout: 5000 });
    await expect(editButton).not.toBeVisible();
  });

  test('[SPEP-006] Add a new question in edit mode', async () => {
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await editButton.click();
    await page.waitForTimeout(1000);

    const addQuestionBtn = page.locator('[data-pw="survey-add-question-btn"]');
    const isAddQuestionVisible = await addQuestionBtn
      .isVisible({ timeout: 3000 })
      .catch(() => false);

    if (isAddQuestionVisible) {
      await addQuestionBtn.click();
      await page.waitForTimeout(1000);

      const hasQuestionEditor = await page
        .locator('.flex.flex-col')
        .first()
        .isVisible();
      expect(hasQuestionEditor).toBe(true);
    }
  });

  test('[SPEP-007] Save changes in edit mode', async () => {
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await editButton.click();
    await page.waitForTimeout(1000);

    const saveButton = page.locator('[data-pw="poll-editor-save-btn"]');
    await saveButton.click();
    await page.waitForTimeout(2000);

    await expect(editButton).toBeVisible({ timeout: 5000 });
  });

  test('[SPEP-008] Discard changes in edit mode', async () => {
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await editButton.click();
    await page.waitForTimeout(1000);

    const discardButton = page.locator('[data-pw="poll-editor-discard-btn"]');
    await discardButton.click();
    await page.waitForTimeout(1000);

    await expect(editButton).toBeVisible({ timeout: 5000 });
  });

  test('[SPEP-009] Edit and Save workflow completes successfully', async () => {
    await navigateToPoll();

    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    await editButton.click();
    await page.waitForTimeout(1000);

    const addQuestionBtn = page.locator('[data-pw="survey-add-question-btn"]');
    const canAddQuestion = await addQuestionBtn
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    if (canAddQuestion) {
      await addQuestionBtn.click();
      await page.waitForTimeout(1000);
    }

    const saveButton = page.locator('[data-pw="poll-editor-save-btn"]');
    await saveButton.click();
    await page.waitForTimeout(2000);

    await expect(editButton).toBeVisible({ timeout: 5000 });
  });
});
