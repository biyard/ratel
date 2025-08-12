// import { test, expect } from '@playwright/test';

// test.describe('Negative Scenarios & Edge Cases', () => {
//   test('should reject SQL injection attempts in login', async ({ page }) => {
//     await page.goto('/');
//     await page.fill('#email', "' OR 1=1 --");
//     await page.fill('#password', 'any');
//     await page.click('button[type="submit"]');

//     // Check for specific security error or that we don't get unexpected access
//     await expect(page).not.toHaveURL('/');
//     await expect(page.locator('.error')).toContainText('Invalid credentials');
//     // Verify no SQL error is exposed to the client
//     await expect(page.locator('body')).not.toContainText(/sql|database|syntax/i);
//   });


//   test('should redirect to login on session expiry', async ({ page }) => {
//     await page.goto('/');

//     // session expiry
//     await page.context().clearCookies();
//     await page.reload();

//     await expect(page).toHaveURL('/');
//     await expect(page.locator('h1')).toHaveText('Login');
//   });

//   test('should prevent unauthorized dashboard access', async ({ page }) => {
//     await page.goto('/');
//     await page.goto('/');

//     await expect(page).toHaveURL("/");
//     await expect(page.locator('h1')).toContainText('Login');
//   });

//   test('should handle multiple tab sessions', async ({ browser }) => {
//     const context = await browser.newContext();
//     const page1 = await context.newPage();
//     const page2 = await context.newPage();

//     await page1.goto('/login');
//     await page1.fill('#email', 'testuser@example.com');
//     await page1.fill('#password', 'correct-password');
//     await page1.click('button[type="submit"]');
//     await page1.waitForURL('/dashboard');

//     await page2.goto('/dashboard');
//     await expect(page2.locator('h1')).toContainText('Dashboard');

//     await page1.click('#logout-button');
//     await page2.reload();
//     await expect(page2).toHaveURL('/login');
//   });

// });