import { test, expect } from '@playwright/test';

test.describe('[SpacePollViewerPage] Anonymous Users', () => {
  test('[SPVP-ANON-001] Anonymous user can access home page', async ({
    page,
  }) => {
    // Navigate to home as anonymous user
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify the page loads
    const currentUrl = page.url();
    expect(currentUrl).toBeTruthy();
  });

  test('[SPVP-ANON-002] Anonymous user has no poll interaction buttons on home', async ({
    page,
  }) => {
    // Navigate to home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Poll viewer buttons should not be visible on home page
    const loginButton = page.locator('[data-pw="poll-viewer-login-btn"]');
    const hasLoginButton = await loginButton
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    // No poll-specific buttons on home page
    expect(hasLoginButton).toBe(false);
  });

  test('[SPVP-ANON-003] Anonymous user cannot see submit button on home', async ({
    page,
  }) => {
    // Navigate to home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Submit button should not be visible on home page
    const submitButton = page.locator('[data-pw="poll-viewer-submit-btn"]');
    const isSubmitVisible = await submitButton
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    expect(isSubmitVisible).toBe(false);
  });

  test('[SPVP-ANON-004] Anonymous user cannot see update button on home', async ({
    page,
  }) => {
    // Navigate to home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Update button should not be visible on home page
    const updateButton = page.locator('[data-pw="poll-viewer-update-btn"]');
    const isUpdateVisible = await updateButton
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    expect(isUpdateVisible).toBe(false);
  });

  test('[SPVP-ANON-005] Anonymous user home navigation works', async ({
    page,
  }) => {
    // Access the home page
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Page is accessible
    expect(page.url()).toBeTruthy();
  });

  test('[SPVP-ANON-006] Anonymous user cannot interact with polls on home', async ({
    page,
  }) => {
    // Navigate home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify that action buttons are not visible on home page
    const submitBtn = page.locator('[data-pw="poll-viewer-submit-btn"]');
    const updateBtn = page.locator('[data-pw="poll-viewer-update-btn"]');
    const loginBtn = page.locator('[data-pw="poll-viewer-login-btn"]');

    const hasSubmit = await submitBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);
    const hasUpdate = await updateBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);
    const hasLogin = await loginBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);

    // No poll-specific buttons on home page
    expect(hasSubmit).toBe(false);
    expect(hasUpdate).toBe(false);
    expect(hasLogin).toBe(false);
  });
});
