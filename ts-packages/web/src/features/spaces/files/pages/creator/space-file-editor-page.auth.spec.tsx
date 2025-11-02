import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click, fill } from '@tests/utils';

test.describe.serial('[SpaceFileEditorPage] Authenticated Users ', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';

  test.beforeAll('Create post', async ({ browser }) => {
    context = await browser.newContext({ storageState: 'user.json' });
    page = await context.newPage();
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const testTitle = 'Automated Post Creation for Thread Page';
    const testContent =
      'This is an automated post content created by Playwright E2E. ' +
      'The purpose of this is to verify that the post creation functionality ' +
      'works correctly from end to end, including title input, content editing, ' +
      'auto-save, and final publication. This content is intentionally long to ' +
      'meet the minimum character requirements for post publishing.';

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

  test('[SPEP-001] Create a deliberation Space', async () => {
    await page.goto(threadUrl);
    await page.waitForTimeout(3000);

    await expect(page.getByText('Create a Space', { exact: true })).toBeVisible(
      { timeout: 20000 },
    );
    await page.getByText('Create a Space', { exact: true }).click();

    const modal = page.getByRole('dialog', { name: 'Select a Space Type' });
    await modal
      .locator('div.cursor-pointer', { hasText: 'Deliberation' })
      .click();

    await modal.getByRole('button', { name: 'Create' }).click();

    await page.waitForURL(/\/spaces\/[^/]+(?:\?.*)?$/, { timeout: 15000 });
  });

  test('[SPEP-002] Upload PDF file', async () => {
    await page.waitForTimeout(2000);

    await page.getByText('Files', { exact: true }).click();
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    await page.waitForSelector(fileInputSelector, { state: 'attached' });

    const buffer = Buffer.from('PDF test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-document.pdf',
      mimeType: 'application/pdf',
      buffer: buffer,
    });

    await expect(page.getByText('test-document.pdf')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-document.pdf')).toBeVisible();
  });

  test('[SPEP-003] Upload DOCX file', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    const buffer = Buffer.from('DOCX test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-document.docx',
      mimeType:
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      buffer: buffer,
    });

    await expect(page.getByText('test-document.docx')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-document.docx')).toBeVisible();
  });

  test('[SPEP-004] Upload XLSX file', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    const buffer = Buffer.from('XLSX test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-spreadsheet.xlsx',
      mimeType:
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      buffer: buffer,
    });

    await expect(page.getByText('test-spreadsheet.xlsx')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-spreadsheet.xlsx')).toBeVisible();
  });

  test('[SPEP-005] Upload JPG image', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    const buffer = Buffer.from('JPG test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-image.jpg',
      mimeType: 'image/jpeg',
      buffer: buffer,
    });

    await expect(page.getByText('test-image.jpg')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-image.jpg')).toBeVisible();
  });

  test('[SPEP-006] Upload PNG image', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    const buffer = Buffer.from('PNG test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-image.png',
      mimeType: 'image/png',
      buffer: buffer,
    });

    await expect(page.getByText('test-image.png')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-image.png')).toBeVisible();
  });

  test('[SPEP-007] Upload GIF image', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const fileInputSelector = 'input[type="file"]';
    const buffer = Buffer.from('GIF test content');
    await page.setInputFiles(fileInputSelector, {
      name: 'test-animation.gif',
      mimeType: 'image/gif',
      buffer: buffer,
    });

    await expect(page.getByText('test-animation.gif')).toBeVisible({
      timeout: 30000,
    });

    await page.getByText('Save', { exact: true }).click();
    await page.waitForTimeout(2000);

    await expect(page.getByText('test-animation.gif')).toBeVisible();
  });

  test('[SPEP-008] Remove uploaded file', async () => {
    await page.waitForTimeout(1000);

    await page.getByText('Edit', { exact: true }).click();
    await page.waitForTimeout(1000);

    const removeButtons = page.locator('button', { hasText: 'Remove' });
    const count = await removeButtons.count();

    if (count > 0) {
      await removeButtons.first().click();
      await page.waitForTimeout(1000);

      await page.getByText('Save', { exact: true }).click();
      await page.waitForTimeout(2000);
    }
  });
});
