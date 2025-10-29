import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click, fill } from '@tests/utils';
// import { fileURLToPath } from 'url';

test.describe
  .serial('[SpaceRecommendationEditorPage] Authenticated Users ', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';
  let spaceUrl = '';

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

    await click(page, { text: 'Create Post' });
    await page.waitForURL(/\/posts\/new/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    // Fill in the title using the ID selector
    await page.fill('#post-title-input', testTitle);

    // Fill in the content using the editor's ProseMirror element
    const editorSelector = '[data-pw="post-content-editor"] .ProseMirror';
    await page.waitForSelector(editorSelector, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    await page.click(editorSelector);
    await page.fill(editorSelector, testContent);

    // Click the publish button using ID selector
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

    spaceUrl = page.url();
  });

  test('[SPEP-002] Update Recommendation Contents', async () => {
    const testContent =
      'This is an automated post content created by Playwright E2E. ' +
      'The purpose of this is to verify that the post creation functionality ' +
      'works correctly from end to end, including title input, content editing, ' +
      'auto-save, and final publication. This content is intentionally long to ' +
      'meet the minimum character requirements for post publishing.';

    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');

    await page.getByText('Recommendations', { exact: true }).click();

    const editorSelector =
      '[data-pw="space-recommendation-editor"] .ProseMirror';

    // Wait for the editor to become editable (contenteditable="true")
    await page.waitForSelector(editorSelector, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    const editIcon = page.locator('svg[role="button"]');
    await expect(editIcon).toBeVisible({ timeout: CONFIGS.SELECTOR_WAIT_TIME });
    await editIcon.click();
    test.setTimeout(5000);
    await page.click(editorSelector);
    await page.fill(editorSelector, testContent);

    const saveButton = page.locator('svg[role="button"]');
    await saveButton.click();
  });

  // FIXME: fix to failed testcode
  //   test('Update Recommendation Files', async () => {
  //     await page.goto(spaceUrl);
  //     await page.waitForTimeout(3000);

  //     await page.getByText('Recommendations', { exact: true }).click();
  //     await page.getByText('Edit', { exact: true }).click();

  //     const [fileChooser] = await Promise.all([
  //       page.waitForEvent('filechooser'),
  //       page.getByText('Upload', { exact: true }).click(),
  //     ]);

  //     const filePath = fileURLToPath(
  //       new URL('../assets/sample.pdf', import.meta.url),
  //     );
  //     await fileChooser.setFiles(filePath);

  //     await expect(page.getByText('sample.pdf')).toBeVisible({ timeout: 50_000 });

  //     await page.getByText('Save', { exact: true }).click();

  //     await expect(page.getByText('sample.pdf')).toBeVisible();
  //   });
});
