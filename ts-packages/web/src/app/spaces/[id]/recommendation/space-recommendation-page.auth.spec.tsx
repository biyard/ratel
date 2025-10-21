import { test, expect } from '@playwright/test';
import { click, fill } from '@tests/utils';
import { fileURLToPath } from 'url';

test.describe.serial('[Space Recommendation] Authenticated Users ', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';
  let spaceUrl = '';

  test.beforeAll('Create a post', async ({ browser }) => {
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

    await click(page, { text: 'Create Post' });
    await fill(page, { placeholder: 'Write a title...' }, testTitle);
    await fill(page, { label: 'general-post-editor' }, testContent);

    await click(page, { label: 'Publish' });

    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });
    threadUrl = page.url();
  });

  test('Create a deliberation Space', async () => {
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

    spaceUrl = page.url();
  });

  test('Update Recommendation Contents', async () => {
    const body = 'This recommendation was edited by Playwright';

    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByText('Recommendations', { exact: true }).click();

    const editIcon = page.locator('svg[role="button"]');
    await expect(editIcon).toBeVisible({ timeout: 5000 });
    await editIcon.click();

    const textEditor = page
      .locator('.tiptap.ProseMirror[contenteditable="true"]')
      .last();

    await textEditor.waitFor();
    await expect(textEditor).toBeVisible();
    await expect(textEditor).toBeEditable();

    const mod = process.platform === 'darwin' ? 'Meta' : 'Control';
    await textEditor.click();
    await textEditor.press(`${mod}+KeyA`);
    await textEditor.press('Backspace');

    await textEditor.type(body);
    await textEditor.press('Enter');

    await expect(page.getByText(body)).toBeVisible();
  });

  test('Update Recommendation Files', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByText('Recommendations', { exact: true }).click();
    await page.getByText('Edit', { exact: true }).click();

    const [fileChooser] = await Promise.all([
      page.waitForEvent('filechooser'),
      page.getByText('Upload', { exact: true }).click(),
    ]);

    const filePath = fileURLToPath(
      new URL('../assets/sample.pdf', import.meta.url),
    );
    await fileChooser.setFiles(filePath);

    await expect(page.getByText('sample.pdf')).toBeVisible({ timeout: 50_000 });

    await page.getByText('Save', { exact: true }).click();

    await expect(page.getByText('sample.pdf')).toBeVisible();
  });
});
