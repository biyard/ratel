import { test, expect, Locator } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click, fill, waitForVisible } from '@tests/utils';

test.describe.serial('[ThreadPage] Authenticated Users ', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';

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

  test('Write a comment', async () => {
    await page.goto(threadUrl);

    const testComment = 'This is automated comment for automation';

    await click(page, { text: 'Share your thoughts...' });

    await fill(page, { label: 'Comment Editor' }, testComment);
    await click(page, { label: 'Publish' });

    await waitForVisible(page, { text: testComment });
  });

  test('Like the post', async () => {
    await page.goto(threadUrl);

    const selector: Locator = await click(page, { label: 'Like Post' });
    await expect(selector.locator('svg')).toHaveClass(/[&>path]:fill-primary/);

    // Unlike
    await click(page, { label: 'Like Post' });
    await expect(selector.locator('svg')).not.toHaveClass(
      /[&>path]:fill-primary/,
    );
  });

  test('Like a comment', async () => {
    await page.goto(threadUrl);

    const btn = await click(page, { label: 'Like Comment' });

    const thumbUpIcon = btn.locator('svg').first();

    // Click like button
    await btn.click();
    await expect(thumbUpIcon).toHaveClass(/fill-primary/);

    // Click unlike button
    await btn.click();
    await expect(thumbUpIcon).not.toHaveClass(/fill-primary/);
  });

  test('Reply to a comment', async () => {
    await page.goto(threadUrl);

    const testReply = 'This is an automated reply to a comment';

    // Find and click the first comment's reply button
    const replyButton = page.getByText('Reply', { exact: true }).first();
    await replyButton.waitFor({ state: 'visible' });
    await replyButton.click();

    // Wait for editor to appear and fill in the reply
    const editor = page
      .locator('div[contenteditable="true"][role="textbox"]')
      .last();
    await editor.waitFor({ state: 'visible' });
    await editor.fill(testReply);

    // Click the Publish button
    await click(page, { label: 'Publish' });

    // Verify the reply appears in the page
    await waitForVisible(page, { text: testReply });
  });

  test('Edit a post', async () => {
    await page.goto(threadUrl);

    // Click the Edit button
    await click(page, { text: 'Edit' });

    // Wait for editor to load
    await page.waitForLoadState('networkidle');

    // Modify the content - add additional text
    const additionalText = ' [EDITED by automation]';
    const editor = page.getByLabel('general-post-editor');
    await editor.waitFor({ state: 'visible' });

    // Get current content and append to it
    const currentText = await editor.textContent();
    await editor.fill(currentText + additionalText);

    // Click Update button
    await click(page, { text: 'Update' });

    // Wait to navigate back to the thread page
    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });

    // Verify the edited text appears
    await waitForVisible(page, { text: additionalText });
  });

  test('Delete a post', async () => {
    await page.goto(threadUrl);

    // Click the dropdown menu button (Extra icon)
    const menuButton = page.locator('button[aria-label="Post options"]');
    await menuButton.waitFor({ state: 'visible' });
    await menuButton.click();

    // Click the Delete button from the dropdown
    const deleteButton = page.getByText('Delete', { exact: true });
    await deleteButton.waitFor({ state: 'visible' });
    await deleteButton.click();

    // Wait for navigation away from the thread page (should redirect to home or posts list)
    await page.waitForURL(/^(?!.*\/threads\/)/, { timeout: 15000 });

    // Verify we're no longer on the thread page
    expect(page.url()).not.toMatch(/\/threads\/.+/);
  });
});
