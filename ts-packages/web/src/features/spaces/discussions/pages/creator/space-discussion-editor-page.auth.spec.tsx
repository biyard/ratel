import { test, expect, Locator } from '@playwright/test';
import { click, fill } from '@tests/utils';

test.describe.serial('[SpaceDiscussionEditorPage] Authenticated Users ', () => {
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

  test('[SPEP-003] Update Discussion', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByText('Discussions', { exact: true }).click();

    const title = 'update deliberation discussion title';
    const description = 'update deliberation discussion description';

    await page.locator('#editable-discussion-option').click();

    const menu = page.locator(
      '#editable-discussion-option >> xpath=following-sibling::*[contains(@class,"absolute") and contains(@class,"z-50")]',
    );

    await expect(menu).toBeVisible();

    await expect(menu.getByText(/^Update$/)).toBeVisible();
    await expect(menu.getByText(/^Delete$/)).toBeVisible();

    await menu.getByText(/^Update$/).click();

    const modal = page.getByRole('dialog', {
      name: 'New Discussion',
    });
    await modal.waitFor();

    const titleInput = modal.getByPlaceholder('Input your discussion name.');
    await clearAndType(titleInput, title);

    const descInput = modal.getByPlaceholder(
      'What is the purpose of your discussion?',
    );
    await clearAndType(descInput, description);

    await modal.getByPlaceholder('Input your discussion name.').fill(title);
    await modal
      .getByPlaceholder('What is the purpose of your discussion?')
      .fill(description);

    await modal.getByRole('button', { name: 'Continue' }).click();
    await modal.locator('div.cursor-pointer', { hasText: 'send' }).click();
    await page.waitForTimeout(500);

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();
  });

  test('[SPEP-004] Delete Discussion', async () => {
    await page.goto(spaceUrl);
    await page.waitForTimeout(3000);

    await page.getByText('Discussions', { exact: true }).click();

    const title = 'update deliberation discussion title';
    const description = 'update deliberation discussion description';

    await page.locator('#editable-discussion-option').click();

    const menu = page.locator(
      '#editable-discussion-option >> xpath=following-sibling::*[contains(@class,"absolute") and contains(@class,"z-50")]',
    );

    await expect(menu).toBeVisible();

    await expect(menu.getByText(/^Update$/)).toBeVisible();
    await expect(menu.getByText(/^Delete$/)).toBeVisible();

    await menu.getByText(/^Delete$/).click();

    const ti = page.getByText(title, { exact: true });
    const desc = page.getByText(description, { exact: true });

    await expect(ti).toHaveCount(0);
    await expect(desc).toHaveCount(0);
  });
});

async function clearAndType(input: Locator, text: string) {
  await input.click({ clickCount: 3 });
  const mod = process.platform === 'darwin' ? 'Meta' : 'Control';
  await input.press(`${mod}+KeyA`);
  await input.press('Backspace');
  await input.type(text);
}
