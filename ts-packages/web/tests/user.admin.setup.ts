import { test, expect } from '@playwright/test';
import { CONFIGS } from './config';
import { click, fill } from './utils';

test('create admin storage state', async ({ page }) => {
  await page.goto(CONFIGS.PLAYWRIGHT.BASE_URL!);
  await page.getByRole('button', { name: /sign in/i }).click();

  await fill(
    page,
    { placeholder: 'Enter your email address' },
    CONFIGS.ADMIN.id,
  );

  await click(page, { text: 'Continue' });

  await fill(
    page,
    { placeholder: 'Enter your password' },
    CONFIGS.ADMIN.password,
  );

  await click(page, { text: 'Sign in' });

  await expect(page.getByRole('link', { name: 'Admin' })).toBeVisible();

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: 'admin.json' });

  console.log('✅ Global authenticated admin setup completed');
  console.log(`🔐 Storage state saved to: admin.json`);
});
