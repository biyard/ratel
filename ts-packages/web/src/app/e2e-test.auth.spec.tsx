import { expect, test } from '@playwright/test';

test.describe.serial('E2E test', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('Create Team', async ({ page }) => {
    await page.locator('[data-pw="team-selector-trigger"]').click();
    await page.waitForTimeout(500);

    await page.getByText('Create a team', { exact: true }).click();
    await page.waitForTimeout(500);

    const timestamp = Date.now();

    await page
      .getByPlaceholder('Team display name')
      .fill('Display Name + ' + timestamp);
    await page
      .getByPlaceholder('Team ID (ex. ratel)')
      .fill('team_id' + timestamp);
    await page
      .getByPlaceholder('Please type description of your team.')
      .fill('Description + ' + timestamp);

    await page.getByText('Create', { exact: true }).click();
    await page.waitForTimeout(500);
    // const testContent =
    //   'This is an automated post content created by Playwright E2E. ' +
    //   'The purpose of this is to verify that the post creation functionality ' +
    //   'works correctly from end to end, including title input, content editing, ' +
    //   'auto-save, and final publication. This content is intentionally long to ' +
    //   'meet the minimum character requirements for post publishing.';

    // const editorSelector = '[data-pw="post-content-editor"] .ProseMirror';
    // await page.waitForSelector(editorSelector, {
    //   timeout: CONFIGS.PAGE_WAIT_TIME,
    // });
    // await page.click(editorSelector);
    // await page.fill(editorSelector, testContent);

    // await page.click('#publish-post-button');

    // await page.waitForURL(/\/threads\/.+/, { timeout: CONFIGS.PAGE_WAIT_TIME });
  });
});
