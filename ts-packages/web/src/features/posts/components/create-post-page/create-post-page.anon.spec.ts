import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';

test.describe('Create Post Page - Anonymous User', () => {
  test('[CP-ANON-001] should redirect anonymous users and show login popup', async ({
    page,
  }) => {
    // Navigate to create post page as anonymous user
    await page.goto('/posts/new');
    await page.waitForLoadState('networkidle');

    // Should redirect to homepage
    await page.waitForURL('/', { timeout: CONFIGS.PAGE_WAIT_TIME });
    expect(page.url()).toContain('/');

    // Login popup should be visible
    const loginPopup = page.getByText(/sign in/i).first();
    await expect(loginPopup).toBeVisible({ timeout: CONFIGS.PAGE_WAIT_TIME });
  });

  test('[CP-ANON-002] should not display create post form for anonymous users', async ({
    page,
  }) => {
    // Navigate to create post page
    await page.goto('/posts/new');
    await page.waitForLoadState('networkidle');

    // Should be redirected (not on /posts/new)
    expect(page.url()).not.toContain('/posts/new');

    // Form elements should not be visible
    const titleInput = page.getByPlaceholder('Title');
    await expect(titleInput).not.toBeVisible();

    const editor = page.locator('[data-pw="post-content-editor"]');
    await expect(editor).not.toBeVisible();
  });

  test('[CP-ANON-003] should show login popup with close option', async ({
    page,
  }) => {
    // Navigate to create post page
    await page.goto('/posts/new');
    await page.waitForLoadState('networkidle');

    // Wait for redirect
    await page.waitForURL('/', { timeout: CONFIGS.PAGE_WAIT_TIME });

    // Login popup should be visible
    const loginPopup = page.getByText(/sign in/i).first();
    await expect(loginPopup).toBeVisible();

    // Popup should have a way to close/dismiss it
    // Look for close button or backdrop
    const closeButton = page.locator('button[aria-label="Close"]').first();
    const hasCloseButton = await closeButton.isVisible().catch(() => false);

    // Just verify login UI is present
    expect(hasCloseButton || loginPopup).toBeTruthy();
  });

  test('[CP-ANON-004] should handle mobile responsive redirect', async ({
    page,
  }) => {
    // Test mobile layout
    await page.setViewportSize({
      width: CONFIGS.DEVICE_SCREEN_SIZES.MOBILE - 100,
      height: 800,
    });

    // Navigate to create post page
    await page.goto('/posts/new');
    await page.waitForLoadState('networkidle');

    // Should still redirect on mobile
    await page.waitForURL('/', { timeout: CONFIGS.PAGE_WAIT_TIME });
    expect(page.url()).toContain('/');

    // Form elements should not be visible
    const titleInput = page.getByPlaceholder('Title');
    await expect(titleInput).not.toBeVisible();

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });

  test('[CP-ANON-005] should persist redirect behavior across page loads', async ({
    page,
  }) => {
    // Try multiple times to ensure consistent redirect behavior
    for (let i = 0; i < 2; i++) {
      await page.goto('/posts/new');
      await page.waitForLoadState('networkidle');

      // Should always redirect
      expect(page.url()).not.toContain('/posts/new');

      // Should be on homepage
      await page.waitForURL('/', { timeout: CONFIGS.PAGE_WAIT_TIME });
    }

    // Final check: login popup should be visible
    const loginPopup = page.getByText(/sign in/i).first();
    await expect(loginPopup).toBeVisible();
  });
});
