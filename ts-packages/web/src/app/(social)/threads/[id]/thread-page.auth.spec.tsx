import { test, expect, Locator } from '@playwright/test';
import { CONFIGS } from '@tests/config';

import { click, waitForVisible } from '@tests/utils';

test.describe.serial('[ThreadPage] Authenticated Users ', () => {
  let threadUrl = '';

  test('[TP-001] Create post', async ({ page }) => {
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

  test('[TP-002] Write a comment', async ({ page }) => {
    await page.goto(threadUrl);

    const testComment = 'This is automated comment for automation';

    await click(page, { text: 'Share your thoughts...' });

    // Fill in the comment using data-pw selector
    const commentEditor = '[data-pw="comment-editor"] .ProseMirror';
    await page.waitForSelector(commentEditor, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    await page.click(commentEditor);
    await page.fill(commentEditor, testComment);

    // Click publish button using ID selector
    await page.click('#publish-comment-button');

    await waitForVisible(page, { text: testComment });
  });

  test('[TP-003] Like the post', async ({ page }) => {
    await page.goto(threadUrl);

    const selector: Locator = await click(page, { label: 'Like Post' });
    await expect(selector.locator('svg')).toHaveClass(/fill-primary/);

    // Unlike
    await click(page, { label: 'Like Post' });
    await expect(selector.locator('svg')).not.toHaveClass(/fill-primary/);
  });

  test('[TP-004] Like a comment', async ({ page }) => {
    await page.goto(threadUrl);

    const btn = await click(page, { label: 'Like Comment' });

    const thumbUpIcon = btn.locator('svg').first();

    // Click like button
    await expect(thumbUpIcon).toHaveClass(/fill-primary/);

    // Click unlike button
    await btn.click();
    await expect(thumbUpIcon).not.toHaveClass(/fill-primary/);
  });

  test('[TP-005] Reply to a comment', async ({ page }) => {
    await page.goto(threadUrl);

    const testReply = 'This is an automated reply to a comment';

    // Find and click the first comment's reply button
    await click(page, { label: 'Reply to Comment' });

    // Wait for editor to appear and fill in the reply
    const editor = page
      .locator('div[contenteditable="true"][role="textbox"]')
      .filter({ visible: true })
      .first();
    await editor.fill(testReply);

    // Click the Publish button
    await click(page, { label: 'Publish' });

    // Verify the reply appears in the page
    await waitForVisible(page, { text: testReply });
  });

  // FIXME: Implement Published Post Edit Page
  // test('[TP-006] Edit a post', async ({ page }) => {
  //   await page.goto(threadUrl);

  //   // Click the Edit button
  //   await click(page, { label: 'Edit Post' });

  //   // Modify the content - add additional text
  //   const additionalText = ' [EDITED by automation]';
  //   const editor = page.getByLabel('general-post-editor');
  //   await editor.waitFor({ state: 'visible' });

  //   // Get current content and append to it
  //   const currentText = await editor.textContent();
  //   await editor.fill(currentText + additionalText);

  //   // Click Update button
  //   await click(page, { text: 'Publish' });

  //   // Wait to navigate back to the thread page
  //   await page.waitForLoadState('networkidle');

  //   // Verify the edited text appears
  //   await waitForVisible(page, {
  //     text: 'This is an automated post content created by Playwright E2E. The purpose of this is to verify that the post creation functionality works correctly from end to end, including title input, content editing, auto-save, and final publication. This content is intentionally long to meet the minimum character requirements for post publishing. [EDITED by automation]',
  //   });
  // });

  test('[TP-007] Delete a post', async ({ page }) => {
    await page.goto(threadUrl);

    await click(page, { label: 'Post options for desktop' });
    await click(page, { label: 'Delete Post' });

    await page.waitForURL('/', { timeout: 15000 });
  });
});
