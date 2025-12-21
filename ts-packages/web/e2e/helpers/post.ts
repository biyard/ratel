import { expect, Page } from '@playwright/test';
import { TIMEOUT } from './auth';
import { clickTeamSidebarMenu } from './team';

export async function createDeliberationPost(
  page: Page,
  title: string,
  content: string,
  youtubeLink?: string,
) {
  await clickTeamSidebarMenu(page, 'drafts');
  await page.getByTestId('create-post-button').click();

  const titleInput = page.getByPlaceholder('Title');
  await titleInput.fill(title);

  const editor = page.locator(
    '[data-pw="post-content-editor"] [contenteditable]',
  );
  await editor.waitFor({ state: 'visible' });
  await editor.click();
  await editor.fill(`${content}\n`);
  await page.keyboard.press('Enter');

  if (youtubeLink) {
    page.once('dialog', async (dialog) => {
      expect(dialog.type()).toBe('prompt');
      expect(dialog.message()).toBe('Input Link URL');
      await dialog.accept(youtubeLink);
    });
    await page.getByTestId('tiptap-toolbar-link').click();
    await page.waitForTimeout(1000);
  }

  // Enable deliberation space
  const skipSpaceCheckbox = page.locator('label[for="skip-space"]');
  const isChecked = await page.locator('#skip-space').isChecked();
  if (isChecked) {
    await skipSpaceCheckbox.click();
  }
  await page.waitForTimeout(100);

  await page
    .locator('[aria-label="space-setting-form-deliberation.label"]')
    .click();
  await page.waitForTimeout(100);

  await page.getByTestId('publish-post-button').click();
  await page.waitForLoadState('networkidle');
}

export async function createPollPost(
  page: Page,
  title: string,
  content: string,
) {
  await clickTeamSidebarMenu(page, 'drafts');
  await page.getByTestId('create-post-button').click();

  const titleInput = page.getByPlaceholder('Title');
  await titleInput.fill(title);

  const editor = page.locator(
    '[data-pw="post-content-editor"] [contenteditable]',
  );
  await editor.waitFor({ state: 'visible' });
  await editor.click();
  await editor.fill(`${content}\n`);
  await page.keyboard.press('Enter');

  // Enable poll space
  const skipSpaceCheckbox = page.locator('label[for="skip-space"]');
  const isChecked = await page.locator('#skip-space').isChecked();
  if (isChecked) {
    await skipSpaceCheckbox.click();
  }
  await page.waitForTimeout(100);

  await page.locator('[aria-label="space-setting-form-poll.label"]').click();
  await page.waitForTimeout(100);

  await page.getByTestId('publish-post-button').click();
  await page.waitForLoadState('networkidle');
}

export async function replyToPost(page: Page, replyContent: string) {
  // Wait for the boards menu item to be visible
  const boardsMenuItem = page.getByTestId('space-sidemenu-boards');
  await boardsMenuItem.waitFor({ state: 'visible', timeout: 15000 });
  await boardsMenuItem.click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  // Force a reload to ensure boards list loads fresh data
  await page.reload({ waitUntil: 'networkidle' });
  await page.waitForTimeout(1000);

  // Wait for board posts to load with a longer timeout
  const firstPost = page.getByTestId('board-post-item').first();
  await expect(firstPost).toBeVisible({ timeout: 15000 });

  // Add a small delay to ensure the post is fully interactive
  await page.waitForTimeout(500);

  await firstPost.click();
  await page.waitForLoadState('networkidle');

  // Write reply
  await page.getByTestId('open-new-comment-box-button').click();
  await page.waitForTimeout(300);

  const commentEditor = page.locator('[data-pw="comment-editor"]');
  await commentEditor
    .getByTestId('tiptap-editor-content')
    .first()
    .locator('[contenteditable="true"]')
    .first()
    .fill(replyContent);

  // Submit reply
  await page.getByLabel('Publish', { exact: true }).click();
  await page.waitForLoadState('networkidle');
}

export async function writeNewPost(
  page: Page,
  title: string,
  content: string,
  category: string,
) {
  await page.getByTestId('space-sidemenu-boards').click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  const createButton = page.getByTestId('board-btn-create-board');
  await createButton.click();

  await page.waitForURL(/.*\/create$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(1000);

  await setEndTimeOneHourLater(page);

  const titleInput = page.getByTestId('board-title-input');
  await titleInput.fill(title);

  const categoryInput = page.getByTestId('board-category-input');
  await categoryInput.fill(category);
  await page.keyboard.press('Enter');
  await page.keyboard.press('Enter');

  const editor = page.locator(
    '[data-pw="space-board-content-editor"] .ProseMirror',
  );
  await editor.click();
  await editor.fill(content);

  await page.getByTestId('board-btn-submit').click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
}

export async function setEndTimeOneHourLater(page: Page) {
  // Calculate current time and 1 hour later
  const now = new Date();
  const oneHourLater = new Date(now.getTime() + 60 * 60 * 1000);

  // Format time for selection (e.g., "02:00 PM")
  const endHour = oneHourLater.getHours();
  const endHour12 = endHour % 12 || 12;
  const endPeriod = endHour < 12 ? 'AM' : 'PM';
  const endTimeText = `${endHour12.toString().padStart(2, '0')}:00 ${endPeriod}`;

  // Click end time dropdown - wait with explicit timeout
  const endTimeButton = page.getByTestId('time-end-dropdown');
  await endTimeButton.waitFor({ state: 'visible', timeout: 15000 });
  await endTimeButton.click();
  await page.waitForTimeout(500);

  // Select the time option
  const timeOption = page.getByText(endTimeText, { exact: true });
  await timeOption.waitFor({ state: 'visible', timeout: 15000 });
  await timeOption.click();
  await page.waitForTimeout(500);

  // Verify the time was set
  await expect(endTimeButton).toContainText(endTimeText);
}
