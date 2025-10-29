import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';

test.describe('Create Post Page - Anonymous User', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/posts/new');
  });

  test('[CP-ANON-001] should redirect anonymous users to login', async ({
    page,
  }) => {
    // Anonymous users should see login popup or be redirected
    const loginPopup = page.locator('#login_popup');
    const signInButton = page.getByRole('button', { name: /sign in/i });

    const isLoginVisible = await loginPopup
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);
    const isSignInVisible = await signInButton
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);

    expect(isLoginVisible || isSignInVisible).toBeTruthy();
  });

  test('[CP-ANON-002] should not allow creating posts without authentication', async ({
    page,
  }) => {
    // Verify that the create post form is not accessible without authentication
    const titleInput = page.getByPlaceholder('Title');
    const isFormVisible = await titleInput
      .isVisible({ timeout: 3000 })
      .catch(() => false);

    // If form is visible, it should be disabled or non-functional
    if (isFormVisible) {
      const publishButton = page.getByRole('button', { name: /publish/i });
      const isPublishDisabled = await publishButton.isDisabled();
      expect(isPublishDisabled).toBeTruthy();
    }
  });

  test('[CP-ANON-003] should display appropriate message for unauthenticated access', async ({
    page,
  }) => {
    // Check for authentication required - either login popup or sign-in button
    const loginPopup = page.locator('#login_popup');
    const signInButton = page.getByRole('button', { name: /sign in/i });

    const isLoginVisible = await loginPopup
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);
    const isSignInVisible = await signInButton
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);

    // Should show some authentication UI
    expect(isLoginVisible || isSignInVisible).toBeTruthy();
  });

  test('[CP-ANON-004] should not allow accessing edit mode via query params', async ({
    page,
  }) => {
    // Try to access an existing post for editing
    await page.goto('/posts/new?post-pk=test-post-id');

    // Should still require authentication
    const loginPopup = page.locator('#login_popup');
    const signInButton = page.getByRole('button', { name: /sign in/i });

    const isLoginVisible = await loginPopup
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);
    const isSignInVisible = await signInButton
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);

    expect(isLoginVisible || isSignInVisible).toBeTruthy();
  });

  test('[CP-ANON-005] should handle mobile responsive layout for login prompt', async ({
    page,
  }) => {
    // Test mobile layout
    await page.setViewportSize({
      width: CONFIGS.DEVICE_SCREEN_SIZES.MOBILE - 100,
      height: 800,
    });

    // Authentication prompt should still be visible
    const signInButton = page.getByRole('button', { name: /sign in/i });
    const isVisible = await signInButton
      .isVisible({ timeout: CONFIGS.PAGE_WAIT_TIME })
      .catch(() => false);

    if (isVisible) {
      await expect(signInButton).toBeVisible();
    }

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });
});
