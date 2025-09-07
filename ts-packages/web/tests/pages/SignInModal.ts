import { Page, Locator, expect } from '@playwright/test';
import { TestHelpers } from '../utils/test-helpers';

export class SignInModal {
  private helpers: TestHelpers;

  // Locators
  readonly modal: Locator;
  readonly closeButton: Locator;
  readonly googleSignInButton: Locator;
  readonly telegramSignInButton: Locator;
  readonly emailInput: Locator;
  readonly passwordInput: Locator;
  readonly signInButton: Locator;
  readonly signUpLink: Locator;
  readonly forgotPasswordLink: Locator;
  readonly modalTitle: Locator;

  constructor(private page: Page) {
    this.helpers = new TestHelpers(page);

    // Initialize locators
    this.modal = page.locator('[data-testid="sign-in-modal"]');
    this.closeButton = page.locator('[data-testid="modal-close"]');
    this.googleSignInButton = page.locator('[data-testid="google-sign-in"]');
    this.telegramSignInButton = page.locator('[data-testid="telegram-sign-in"]');
    this.emailInput = page.locator('[data-testid="email-input"]');
    this.passwordInput = page.locator('[data-testid="password-input"]');
    this.signInButton = page.locator('[data-testid="sign-in-submit"]');
    this.signUpLink = page.locator('[data-testid="sign-up-link"]');
    this.forgotPasswordLink = page.locator('[data-testid="forgot-password-link"]');
    this.modalTitle = page.locator('[data-testid="modal-title"]');
  }

  async waitForModal() {
    await this.modal.waitFor({ state: 'visible' });
  }

  async close() {
    await this.closeButton.click();
    await this.modal.waitFor({ state: 'hidden' });
  }

  async signInWithEmail(email: string, password: string) {
    if (await this.emailInput.isVisible()) {
      await this.helpers.fillField('[data-testid="email-input"]', email);
      await this.helpers.fillField('[data-testid="password-input"]', password);
      await this.signInButton.click();
    }
  }

  async signInWithGoogle() {
    if (await this.googleSignInButton.isVisible()) {
      await this.googleSignInButton.click();
      // Handle OAuth redirect
      await this.page.waitForURL('**/oauth/**', { timeout: 10000 });
    }
  }

  async signInWithTelegram() {
    if (await this.telegramSignInButton.isVisible()) {
      await this.telegramSignInButton.click();
      // Handle Telegram auth
      await this.page.waitForURL('**/telegram/**', { timeout: 10000 });
    }
  }

  async goToSignUp() {
    if (await this.signUpLink.isVisible()) {
      await this.signUpLink.click();
    }
  }

  async goToForgotPassword() {
    if (await this.forgotPasswordLink.isVisible()) {
      await this.forgotPasswordLink.click();
    }
  }

  async takeScreenshot(name: string) {
    await this.page.screenshot({ 
      path: `test-results/screenshots/signin-modal-${name}.png`
    });
  }

  // Assertions
  async expectModalVisible() {
    await expect(this.modal).toBeVisible();
  }

  async expectModalHidden() {
    await expect(this.modal).toBeHidden();
  }

  async expectSignInOptionsVisible() {
    // Check that at least one sign-in method is available
    const hasGoogleSignIn = await this.googleSignInButton.isVisible();
    const hasTelegramSignIn = await this.telegramSignInButton.isVisible();
    const hasEmailSignIn = await this.emailInput.isVisible();

    expect(hasGoogleSignIn || hasTelegramSignIn || hasEmailSignIn).toBeTruthy();
  }
}