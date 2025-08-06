import { test, expect } from '@playwright/test';

test.describe('Google OAuth Login Flow', () => {
  test('should login via Google and redirect to home', async ({ page, context }) => {
    await page.goto('/');

    // Click Google login button (assuming button has id="google-login")
    const [popup] = await Promise.all([
      context.waitForEvent('page'),
      page.click('#google-login'),
    ]);

    await popup.fill('input[type="email"]', 'testgoogleuser@gmail.com');
    await popup.click('button:has-text("Next")');
    await popup.fill('input[type="password"]', 'your-google-password');
    await popup.click('button:has-text("Next")');
    await popup.waitForLoadState('networkidle');

    await popup.close();
    await page.waitForURL('/home');
    await expect(page.locator('h1')).toContainText('home');
  });

  test('should persist session on reload', async ({ page }) => {
    await page.goto('/home');
    await page.reload();

    await expect(page).toHaveURL('/home');
    await expect(page.locator('h1')).toContainText('home');
  });

  test('should redirect to login when logged out', async ({ page }) => {
    await page.goto('/home');

    await page.click('#logout-button');
    await page.waitForURL('/login');
    await expect(page.locator('h1')).toContainText('Login');
  });
});
