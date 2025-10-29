// FIXME: remove comment when pr testing completed
// import { test, expect } from '@playwright/test';
// import { click, fill } from '@tests/utils';
// // import { fileURLToPath } from 'url';

// test.describe.serial('[SpacePanelEditorPage] Authenticated Users ', () => {
//   let context: import('@playwright/test').BrowserContext;
//   let page: import('@playwright/test').Page;

//   let threadUrl = '';
//   let spaceUrl = '';

//   test.beforeAll('Create a post', async ({ browser }) => {
//     context = await browser.newContext({ storageState: 'user.json' });
//     page = await context.newPage();
//     await page.goto('/');
//     await page.waitForLoadState('networkidle');

//     const testTitle = 'Automated Post Creation for Thread Page';
//     const testContent =
//       'This is an automated post content created by Playwright E2E. ' +
//       'The purpose of this is to verify that the post creation functionality ' +
//       'works correctly from end to end, including title input, content editing, ' +
//       'auto-save, and final publication. This content is intentionally long to ' +
//       'meet the minimum character requirements for post publishing.';

//     await click(page, { text: 'Create Post' });
//     await fill(page, { placeholder: 'Write a title...' }, testTitle);
//     await fill(page, { label: 'general-post-editor' }, testContent);

//     await click(page, { label: 'Publish' });

//     await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });
//     threadUrl = page.url();
//   });

//   test('[SPEP-001] Create a deliberation Space', async () => {
//     await page.goto(threadUrl);
//     await page.waitForTimeout(3000);

//     await expect(page.getByText('Create a Space', { exact: true })).toBeVisible(
//       { timeout: 20000 },
//     );
//     await page.getByText('Create a Space', { exact: true }).click();

//     const modal = page.getByRole('dialog', { name: 'Select a Space Type' });
//     await modal
//       .locator('div.cursor-pointer', { hasText: 'Deliberation' })
//       .click();

//     await modal.getByRole('button', { name: 'Create' }).click();

//     await page.waitForURL(/\/spaces\/[^/]+(?:\?.*)?$/, { timeout: 15000 });

//     spaceUrl = page.url();
//   });

//   test('[SPEP-002] Create Panel', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.locator('#add-panel-button').click();

//     await expect(page.getByText('Enter panel name')).toBeVisible();
//   });

//   test('[SPEP-003] Input Panel Name', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.getByText('Enter panel name', { exact: true }).click();
//     const input = page.getByPlaceholder('Enter panel name');
//     await input.click();
//     await input.fill('Panel Label 1');
//     await input.press('Enter');
//     await page.waitForTimeout(1000);
//     await expect(
//       page.getByText('Panel Label 1', { exact: true }),
//     ).toBeVisible();
//   });

//   test('[SPEP-004] Select Age', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.locator('#age-td').click();

//     const modal = page.getByRole('dialog', { name: 'Set Age Attributes' });
//     await modal.getByText('17 and under').click();
//     await modal.getByRole('button', { name: 'Save' }).click();

//     await page.waitForTimeout(1000);
//     await expect(page.getByText('17 and under', { exact: true })).toBeVisible();
//   });

//   test('[SPEP-005] Select Gender', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.locator('#gender-td').click();

//     const modal = page.getByRole('dialog', { name: 'Set Gender Attributes' });
//     await modal.locator('label[for="gender-male"]').click();
//     await modal.getByRole('button', { name: /save/i }).click();

//     await page.waitForTimeout(1000);
//     await expect(page.getByText('Male', { exact: true })).toBeVisible();
//   });

//   test('[SPEP-006] Input Panel Quotas', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.locator('#quotas-td').click();
//     const input = page.getByPlaceholder('0');
//     await input.click();
//     await input.fill('200');
//     await input.press('Enter');
//     await page.waitForTimeout(1000);
//     await expect(page.getByText('200', { exact: true })).toBeVisible();
//   });

//   test('[SPEP-007] Delete Panel', async () => {
//     await page.goto(spaceUrl);
//     await page.waitForTimeout(3000);

//     await page.getByText('Panels', { exact: true }).click();
//     await page.locator('#menu-option').click();
//     await page.getByText('Delete', { exact: true }).click();
//     await page.waitForTimeout(1000);
//     const table = page.locator('table');
//     await expect(table.getByText('Panel Label 1', { exact: true })).toHaveCount(
//       0,
//     );
//   });
// });
