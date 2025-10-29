import { test, expect } from '@playwright/test';
import { click, fill } from '@tests/utils';

test.describe
  .serial('[SpaceDiscussionMeetingViewerPage] Authenticated Users ', () => {
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
    await fill(page, { placeholder: 'Write a title...' }, testTitle);
    await fill(page, { label: 'general-post-editor' }, testContent);

    await click(page, { label: 'Publish' });

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

  test('[SPEP-002] Create Discussion', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByText('Discussions', { exact: true }).click();
    await page.getByText('Add Discussion', { exact: true }).click();

    const title = 'deliberation discussion title';
    const description = 'deliberation discussion description';

    const modal = page.getByRole('dialog', { name: 'New Discussion' });
    await modal.waitFor();

    await modal.getByPlaceholder('Input your discussion name.').fill(title);
    await modal
      .getByPlaceholder('What is the purpose of your discussion?')
      .fill(description);
    await modal.getByRole('button', { name: 'Continue' }).click();
    await modal.locator('div.cursor-pointer', { hasText: 'send' }).click();

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();
  });

  test('[SPEP-003] Publish Space', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(1000);

    await page.getByText('Publish', { exact: true }).click();

    const modal = page.getByRole('dialog', { name: 'Publish Space' });
    await page.waitForTimeout(1000);
    await modal.waitFor();

    await modal.getByText('Public Publish', { exact: true }).click();
    await modal.getByText('Publish', { exact: true }).click();
    await page.waitForTimeout(1000);

    await page.goto(spaceUrl + '/discussions');
    await page.waitForTimeout(1000);
    await expect(page.getByText('Join', { exact: true })).toBeVisible();
  });

  // FIXME: checking failed testing logic..
  //   test('[SPEP-004] Participate Meeting (check participate members)', async () => {
  //     await page.goto(spaceUrl + '/discussions');
  //     await page.waitForTimeout(100);

  //     await page.getByText('Join', { exact: true }).click();
  //     await page.waitForTimeout(1000);

  //     await expect(
  //       page.getByText('deliberation discussion title', { exact: true }),
  //     ).toBeVisible();
  //     await page.getByText('Participants', { exact: true }).click();
  //     const rows = page.locator('div').filter({
  //       has: page.getByRole('img', { name: /'s profile$/ }),
  //     });
  //     await expect(rows.first()).toBeVisible();
  //     await page.locator('#participant-close-button').click();

  //     await page.getByText('End', { exact: true }).click();

  //     await page.waitForTimeout(100);
  //     await expect(
  //       page.getByText('deliberation discussion title', { exact: true }),
  //     ).toBeVisible();
  //   });

  //   test('[SPEP-005] Participate Meeting (send messages)', async () => {
  //     const message = 'meeting message';

  //     await page.goto(spaceUrl + '/discussions');
  //     await page.waitForTimeout(100);

  //     await page.getByText('Join', { exact: true }).click();
  //     await page.waitForTimeout(1000);

  //     await expect(
  //       page.getByText('deliberation discussion title', { exact: true }),
  //     ).toBeVisible();

  //     await page.getByText('Chat', { exact: true }).click();
  //     await page.waitForTimeout(100);

  //     const input = page.getByPlaceholder('Type message here');
  //     await input.fill(message);
  //     await input.press('Enter');
  //     await page.waitForTimeout(100);

  //     await expect(page.getByText(message, { exact: true })).toBeVisible();
  //     await page.locator('#chat-close-button').click();
  //     await page.getByText('End', { exact: true }).click();

  //     await page.waitForTimeout(100);
  //     await expect(
  //       page.getByText('deliberation discussion title', { exact: true }),
  //     ).toBeVisible();
  //   });
});
