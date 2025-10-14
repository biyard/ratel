import { test, expect, Locator } from '@playwright/test';
import { click, fill } from '@tests/utils';

test.describe.serial('[DeliberationPage] Authenticated Users ', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;

  let threadUrl = '';
  let deliberationUrl = '';

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

    await click(page, { text: 'Create a Space' });

    const modal = page.getByRole('dialog', { name: 'Select a Space Type' });
    await modal
      .locator('div.cursor-pointer', { hasText: 'Deliberation' })
      .click();

    await modal.getByRole('button', { name: 'Send' }).click();

    await page.waitForURL(/\/spaces\/[^/]+\/deliberation(?:\?.*)?$/, {
      timeout: 15000,
    });

    deliberationUrl = page.url();
  });

  test('Edit Summary', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);

    const newTitle = `E2E Edited Title ${Date.now()}`;
    const bodyL1 = 'This summary was edited by Playwright';
    const bodyL2 = 'Second line with more details.';

    await page.getByRole('button', { name: 'Edit' }).click();
    const editor = page.getByRole('textbox').first();
    await editor.waitFor();

    await editor.click();
    await editor.press(process.platform === 'darwin' ? 'Meta+A' : 'Control+A');
    await editor.press('Backspace');

    await editor.pressSequentially(newTitle, {
      delay: 10,
    });

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

    await textEditor.type(bodyL1);
    await textEditor.press('Enter');
    await textEditor.type(bodyL2);

    await page.getByRole('button', { name: 'Save' }).click();

    await expect(editor).toBeHidden({ timeout: 10000 });
    const viewBody = page.locator('.rich-content').first();
    await expect(viewBody).toBeVisible();

    await expect(page.getByText(newTitle, { exact: true })).toBeVisible();
    await expect(viewBody).toContainText(bodyL1);
    await expect(viewBody).toContainText(bodyL2);

    await page.reload();
    await expect(page.getByText(newTitle, { exact: true })).toBeVisible();
    await expect(page.locator('.rich-content').first()).toContainText(bodyL1);
    await expect(page.locator('.rich-content').first()).toContainText(bodyL2);
  });

  test('Edit Deliberation', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);

    const title = 'deliberation discussion title';
    const description = 'deliberation discussion description';

    await page
      .locator('div.cursor-pointer', { hasText: 'deliberation' })
      .click();

    await page.getByRole('button', { name: 'Edit' }).click();
    await page.locator('#add-discussion-btn').click();

    const modal = page.getByRole('dialog', { name: 'New Discussion' });
    await modal.waitFor();

    await modal.getByPlaceholder('Input your discussion name.').fill(title);
    await modal
      .getByPlaceholder('What is the purpose of your discussion?')
      .fill(description);
    await modal.getByRole('button', { name: 'continue' }).click();
    await modal.locator('div.cursor-pointer', { hasText: 'send' }).click();

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();

    await page.getByRole('button', { name: 'Save' }).click();

    await page
      .locator('div.cursor-pointer', { hasText: 'deliberation' })
      .click();

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();
  });

  test('Update Deliberation Discussion', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);

    const title = 'update deliberation discussion title';
    const description = 'update deliberation discussion description';

    await page
      .locator('div.cursor-pointer', { hasText: 'deliberation' })
      .click();

    await page.getByRole('button', { name: 'Edit' }).click();
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

    await modal.getByRole('button', { name: 'continue' }).click();
    await modal.locator('div.cursor-pointer', { hasText: 'send' }).click();

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();

    await page.getByRole('button', { name: 'Save' }).click();

    await page
      .locator('div.cursor-pointer', { hasText: 'deliberation' })
      .click();

    await expect(page.getByText(title, { exact: true })).toBeVisible();
    await expect(page.getByText(description, { exact: true })).toBeVisible();
  });

  test('Edit Poll', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);

    await page.locator('div.cursor-pointer', { hasText: 'Poll' }).click();

    const questionTitle1 = 'short answer title';
    const questionTitle2 = 'short answer title 2';
    const questionTitle3 = 'short answer title 3';

    const questionCards = page.locator(
      'div.bg-card-bg-secondary.border.rounded-\\[10px\\]',
    );

    const Q = (n: number) => {
      const root = questionCards.nth(n);

      const typeTrigger = root
        .getByRole('button', {
          name: 'Short Answer Question',
        })
        .or(root.locator('button:has-text("Short Answer Question")'));

      const title = root.getByPlaceholder('Title');

      const requiredRow = root
        .locator(':scope', { hasText: 'Required' })
        .first();
      const toggle = requiredRow
        .locator('div.w-11.h-5.flex.items-center.rounded-full')
        .first();

      return {
        root,
        typeTrigger,
        title,
        toggle,
      };
    };

    await page.getByRole('button', { name: 'Edit' }).click();
    await page.locator('#add-question-btn').click();

    const q0 = Q(0);
    await q0.typeTrigger.click();
    await page.getByText('Subjective Answer Question').click();
    await q0.title.fill(questionTitle1);

    await page.locator('#add-question-btn').click();
    const q1 = Q(1);
    await q1.title.fill(questionTitle2);

    await page.locator('#add-question-btn').click();
    const q2 = Q(2);
    await q2.title.fill(questionTitle3);

    await page.locator('#timeline-setting').click();

    const modal = page.locator('#popup-zone');
    await modal.waitFor();

    const pickerRow = modal.locator('div.flex.flex-wrap.items-center');
    const pickerButtons = pickerRow.locator('button[aria-haspopup="dialog"]');

    const endDateBtn = pickerButtons.nth(2);

    await endDateBtn.click();

    const calendar = page.locator(
      'div[role="dialog"][id^="radix-"][data-state="open"]',
    );
    await expect(calendar).toBeVisible();
    const caption = await calendar
      .locator('.rdp-month_caption, .rdp-caption_label')
      .first()
      .textContent();
    await calendar.locator('button.rdp-button_next').click();
    await expect(
      calendar.locator('.rdp-month_caption, .rdp-caption_label').first(),
    ).not.toHaveText(caption ?? '', { timeout: 3000 });
    const firstOfMonthBtn = calendar
      .locator(
        'td.rdp-day:not(.rdp-hidden):not(.rdp-outside)[data-day$="-01"] >> button.rdp-day_button',
      )
      .first();
    await firstOfMonthBtn.click();
    await modal.getByRole('button', { name: 'Confirm' }).click();

    await page.getByRole('button', { name: 'Save' }).click();
    await page.waitForTimeout(3000);

    await page.locator('div.cursor-pointer', { hasText: 'Poll' }).click();
    await expect(page.getByText(questionTitle1, { exact: true })).toBeVisible();
    await expect(page.getByText(questionTitle2, { exact: true })).toBeVisible();
    await expect(page.getByText(questionTitle3, { exact: true })).toBeVisible();
  });

  test('Edit Recommendation', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);

    await page
      .locator('div.cursor-pointer', { hasText: 'Recommendation' })
      .click();

    const newTitle = `E2E Edited Title ${Date.now()}`;
    const bodyL1 = 'This summary was edited by Playwright';
    const bodyL2 = 'Second line with more details.';

    await page.getByRole('button', { name: 'Edit' }).click();
    const editor = page.getByRole('textbox').first();
    await editor.waitFor();

    await editor.click();
    await editor.press(process.platform === 'darwin' ? 'Meta+A' : 'Control+A');
    await editor.press('Backspace');

    await editor.pressSequentially(newTitle, {
      delay: 10,
    });

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

    await textEditor.type(bodyL1);
    await textEditor.press('Enter');
    await textEditor.type(bodyL2);

    await page.getByRole('button', { name: 'Save' }).click();

    await expect(editor).toBeHidden({ timeout: 10000 });
    const viewBody = page.locator('.rich-content').first();
    await expect(viewBody).toBeVisible();

    await expect(page.getByText(newTitle, { exact: true })).toBeVisible();
    await expect(viewBody).toContainText(bodyL1);
    await expect(viewBody).toContainText(bodyL2);

    await page.reload();
    await expect(page.getByText(newTitle, { exact: true })).toBeVisible();
    await expect(page.locator('.rich-content').first()).toContainText(bodyL1);
    await expect(page.locator('.rich-content').first()).toContainText(bodyL2);
  });

  test('Publish', async () => {
    await page.goto(deliberationUrl);
    await page.waitForTimeout(3000);
    await page.getByRole('button', { name: 'Publish' }).click();

    const modal = page.locator('#popup-zone');
    await modal.waitFor();

    await modal.getByText('Private Publish').click();
    await modal.getByRole('button', { name: 'Publish' }).click();

    await page.reload();
    await expect(page.getByRole('button', { name: 'Publish' })).toHaveCount(0);
  });

  //TODO: add implement survey logic and public publish logic
});

async function clearAndType(input: Locator, text: string) {
  await input.click({ clickCount: 3 });
  const mod = process.platform === 'darwin' ? 'Meta' : 'Control';
  await input.press(`${mod}+KeyA`);
  await input.press('Backspace');
  await input.type(text);
}
