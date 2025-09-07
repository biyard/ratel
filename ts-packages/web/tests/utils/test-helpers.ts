import { Page, expect, Locator } from '@playwright/test';

export class TestHelpers {
  constructor(private page: Page) {}

  /**
   * Wait for an element to be visible and take a screenshot
   */
  async waitAndScreenshot(selector: string, name: string) {
    await this.page.waitForSelector(selector, { state: 'visible' });
    await this.page.screenshot({ path: `test-results/screenshots/${name}.png` });
  }

  /**
   * Fill form field with proper wait
   */
  async fillField(selector: string, value: string) {
    const field = this.page.locator(selector);
    await field.waitFor({ state: 'visible' });
    await field.fill(value);
  }

  /**
   * Click button with proper wait and error handling
   */
  async clickButton(selector: string) {
    const button = this.page.locator(selector);
    await button.waitFor({ state: 'visible' });
    await button.click();
  }

  /**
   * Wait for navigation after action
   */
  async waitForNavigation(action: () => Promise<void>, expectedUrl?: string) {
    await Promise.all([
      this.page.waitForLoadState('networkidle'),
      action(),
    ]);

    if (expectedUrl) {
      expect(this.page.url()).toContain(expectedUrl);
    }
  }

  /**
   * Check if element exists without failing
   */
  async elementExists(selector: string): Promise<boolean> {
    try {
      await this.page.waitForSelector(selector, { timeout: 5000 });
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get element text content safely
   */
  async getTextContent(selector: string): Promise<string | null> {
    const element = this.page.locator(selector);
    await element.waitFor({ state: 'attached' });
    return element.textContent();
  }

  /**
   * Scroll element into view and click
   */
  async scrollAndClick(selector: string) {
    const element = this.page.locator(selector);
    await element.scrollIntoViewIfNeeded();
    await element.click();
  }

  /**
   * Wait for API response
   */
  async waitForApiResponse(urlPattern: string | RegExp) {
    return this.page.waitForResponse(urlPattern);
  }

  /**
   * Mock API response for testing
   */
  async mockApiResponse(urlPattern: string | RegExp, response: any) {
    await this.page.route(urlPattern, route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(response),
      });
    });
  }
}

/**
 * Mobile-specific helpers
 */
export class MobileHelpers extends TestHelpers {
  /**
   * Open mobile menu if it exists
   */
  async openMobileMenu() {
    const menuButton = this.page.locator('[data-testid="mobile-menu-button"]');
    const isVisible = await menuButton.isVisible();
    
    if (isVisible) {
      await menuButton.click();
      await this.page.waitForSelector('[data-testid="mobile-menu"]', { state: 'visible' });
    }
  }

  /**
   * Close mobile menu if it exists
   */
  async closeMobileMenu() {
    const closeButton = this.page.locator('[data-testid="mobile-menu-close"]');
    const isVisible = await closeButton.isVisible();
    
    if (isVisible) {
      await closeButton.click();
      await this.page.waitForSelector('[data-testid="mobile-menu"]', { state: 'hidden' });
    }
  }
}

/**
 * Authentication helpers
 */
export class AuthHelpers extends TestHelpers {
  /**
   * Check if user is logged in
   */
  async isLoggedIn(): Promise<boolean> {
    try {
      await this.page.waitForSelector('[data-testid="user-menu"]', { timeout: 3000 });
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Sign out user
   */
  async signOut() {
    const userMenu = this.page.locator('[data-testid="user-menu"]');
    if (await userMenu.isVisible()) {
      await userMenu.click();
      await this.page.locator('[data-testid="sign-out-button"]').click();
      await this.page.waitForSelector('[data-testid="sign-in-button"]');
    }
  }

  /**
   * Open sign in modal
   */
  async openSignInModal() {
    await this.page.locator('[data-testid="sign-in-button"]').click();
    await this.page.waitForSelector('[data-testid="sign-in-modal"]', { state: 'visible' });
  }

  /**
   * Close any open modal
   */
  async closeModal() {
    const closeButton = this.page.locator('[data-testid="modal-close"]');
    if (await closeButton.isVisible()) {
      await closeButton.click();
    }
  }
}