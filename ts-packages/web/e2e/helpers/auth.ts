import { Page } from '@playwright/test';

export const TIMEOUT = 10000;

export async function login(
  page: Page,
  { email, password }: { email: string; password: string },
) {
  // Clear cookies to ensure a fresh session
  await page.context().clearCookies();

  // Navigate to the page first, then clear storage
  await page.goto('/');
  await page.waitForLoadState('networkidle');

  // Clear storage after navigation
  await page.evaluate(() => {
    localStorage.clear();
    sessionStorage.clear();
  });

  // Reload page to apply cleared storage
  await page.reload();
  await page.waitForLoadState('networkidle');

  // Now sign in
  const signInButton = page.getByRole('button', { name: /sign in/i });
  await signInButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await signInButton.click();
  await page.getByTestId('email-input').fill(email);
  await page.getByTestId('continue-button').click();
  await page.getByTestId('password-input').fill(password);
  await page.getByTestId('continue-button').click();

  await page.getByRole('dialog').waitFor({
    state: 'detached',
    timeout: TIMEOUT,
  });

  await page.waitForURL('/', {
    waitUntil: 'networkidle',
    timeout: TIMEOUT,
  });
}

export async function logout(page: Page) {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  // Click user menu and logout
  const userMenu = page.getByTestId('user-menu-trigger');
  if (await userMenu.isVisible()) {
    await userMenu.click();
    await page.getByTestId('logout-button').click();
    await page.waitForLoadState('networkidle');
  }
}

export async function mobileLogin(page: Page, email: string, password: string) {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  const menuButton = page.getByTestId('mobile-menu-toggle');
  await menuButton.waitFor({ state: 'visible' });
  await menuButton.click();

  const signInButton = page.getByRole('button', { name: /sign in/i });
  await signInButton.waitFor({ state: 'visible' });
  await signInButton.click();

  await page.getByTestId('email-input').fill(email);
  await page.getByTestId('continue-button').click();
  await page.getByTestId('password-input').fill(password);
  await page.getByTestId('continue-button').click();

  // Wait for login dialog to be completely removed from DOM
  await page.getByRole('dialog').waitFor({
    state: 'detached',
    timeout: TIMEOUT,
  });

  // Wait for navigation to complete with network idle
  await page.waitForURL('/', {
    waitUntil: 'networkidle',
    timeout: TIMEOUT,
  });
}
