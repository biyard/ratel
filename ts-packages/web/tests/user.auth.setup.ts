import { test, expect } from '@playwright/test';
import { CONFIGS } from './config';
import { click } from './utils';

test('create storage state', async ({ page }) => {
  const id = CONFIGS.PLAYWRIGHT.ID;
  const email = `playwright+${id}@ratel.foundation`;
  const password = 'password1234!@#$';
  const displayName = `Playwright User ${id}`;
  const userName = `pw-${id}`;
  console.log(`🆕 Creating new user: ${email} / ${password}`);

  await page.goto(CONFIGS.PLAYWRIGHT.BASE_URL!);
  await page.getByRole('button', { name: /sign in/i }).click();
  await page.getByText('Create an account').click();

  await page.getByPlaceholder('Email', { exact: true }).fill(email);
  await page.getByText('Send', { exact: true }).click();

  const codeInput = page.getByPlaceholder('Verify code in your email.', {
    exact: true,
  });
  await expect(codeInput).toBeVisible({ timeout: CONFIGS.PAGE_WAIT_TIME });
  await codeInput.fill('000000');
  await click(page, { text: 'Verify' });

  await page.getByPlaceholder(/password/i).fill(password);
  await page.getByPlaceholder(/display name/i).fill(displayName);
  await page.getByPlaceholder(/user name/i).fill(userName);

  // Accept terms by clicking the label (checkbox is hidden)
  const tosCheckbox = page.locator('label[for="agree_checkbox"]');
  await tosCheckbox.click();
  await page.getByRole('button', { name: /finished sign-up/i }).click();
  await expect(page.getByText(/start/i)).toBeVisible();

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: 'user.json' });

  console.log('✅ Global authenticated user setup completed');
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: user.json`);
});
