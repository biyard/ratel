import { test, expect } from '@playwright/test';
import { HomePage } from './pages/HomePage';
import { SignInModal } from './pages/SignInModal';

test.describe('Authentication', () => {
  test.beforeEach(async ({ page }) => {
    // Start fresh on each test
    await page.context().clearCookies();
  });

  test('should display sign in button when user is not authenticated', async ({ page }) => {
    const homePage = new HomePage(page);
    
    await homePage.goto();
    await homePage.expectSignInButtonVisible();
    await homePage.takeScreenshot('unauthenticated-home');
  });

  test('should open sign in modal when sign in button is clicked', async ({ page }) => {
    const homePage = new HomePage(page);
    const signInModal = new SignInModal(page);
    
    await homePage.goto();
    await homePage.openSignInModal();
    await signInModal.expectModalVisible();
    await signInModal.expectSignInOptionsVisible();
    await signInModal.takeScreenshot('opened-modal');
  });

  test('should close sign in modal when close button is clicked', async ({ page }) => {
    const homePage = new HomePage(page);
    const signInModal = new SignInModal(page);
    
    await homePage.goto();
    await homePage.openSignInModal();
    await signInModal.waitForModal();
    await signInModal.close();
    await signInModal.expectModalHidden();
  });

  test('should show different sign-in options in modal', async ({ page }) => {
    const homePage = new HomePage(page);
    const signInModal = new SignInModal(page);
    
    await homePage.goto();
    await homePage.openSignInModal();
    await signInModal.waitForModal();
    
    // Check that at least one sign-in method is available
    const hasGoogleSignIn = await signInModal.googleSignInButton.isVisible();
    const hasTelegramSignIn = await signInModal.telegramSignInButton.isVisible();
    const hasEmailSignIn = await signInModal.emailInput.isVisible();

    // At least one sign-in method should be visible
    expect(hasGoogleSignIn || hasTelegramSignIn || hasEmailSignIn).toBeTruthy();
    
    await signInModal.takeScreenshot('signin-options');
  });

  test.describe('Mobile Authentication', () => {
    test.use({ viewport: { width: 375, height: 667 } });

    test('should work correctly on mobile devices', async ({ page }) => {
      const homePage = new HomePage(page);
      const signInModal = new SignInModal(page);
      
      await homePage.goto();
      await homePage.expectSignInButtonVisible();
      await homePage.openSignInModal();
      await signInModal.expectModalVisible();
      await signInModal.takeScreenshot('mobile-signin-modal');
    });
  });

  // Skip OAuth tests in CI as they require external services
  test.skip('OAuth sign in flows', () => {
    test('should handle Google OAuth sign in', async ({ page }) => {
      const homePage = new HomePage(page);
      const signInModal = new SignInModal(page);
      
      await homePage.goto();
      await homePage.openSignInModal();
      await signInModal.waitForModal();
      
      if (await signInModal.googleSignInButton.isVisible()) {
        await signInModal.signInWithGoogle();
        // This would normally redirect to Google OAuth
        // In a real test, you'd mock this or use a test OAuth provider
      }
    });

    test('should handle Telegram sign in', async ({ page }) => {
      const homePage = new HomePage(page);
      const signInModal = new SignInModal(page);
      
      await homePage.goto();
      await homePage.openSignInModal();
      await signInModal.waitForModal();
      
      if (await signInModal.telegramSignInButton.isVisible()) {
        await signInModal.signInWithTelegram();
        // This would normally redirect to Telegram auth
      }
    });
  });
});