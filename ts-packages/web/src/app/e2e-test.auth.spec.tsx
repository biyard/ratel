import { test } from '@playwright/test';

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
  });
});
