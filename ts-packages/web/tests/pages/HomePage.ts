import { Page, Locator, expect } from '@playwright/test';
import { TestHelpers, MobileHelpers, AuthHelpers } from '../utils/test-helpers';

export class HomePage {
  private helpers: TestHelpers;
  private mobileHelpers: MobileHelpers;
  private authHelpers: AuthHelpers;

  // Locators
  readonly signInButton: Locator;
  readonly userMenu: Locator;
  readonly navigationMenu: Locator;
  readonly mobileMenuButton: Locator;
  readonly aboutLink: Locator;
  readonly politicianStanceLink: Locator;
  readonly communityLink: Locator;
  readonly supportLink: Locator;
  readonly viewAllLink: Locator;
  readonly logo: Locator;
  readonly feedContainer: Locator;
  readonly createPostButton: Locator;

  constructor(private page: Page) {
    this.helpers = new TestHelpers(page);
    this.mobileHelpers = new MobileHelpers(page);
    this.authHelpers = new AuthHelpers(page);

    // Initialize locators
    this.signInButton = page.locator('[data-testid="sign-in-button"]');
    this.userMenu = page.locator('[data-testid="user-menu"]');
    this.navigationMenu = page.locator('nav');
    this.mobileMenuButton = page.locator('[data-testid="mobile-menu-button"]');
    this.aboutLink = page.locator('nav a').filter({ hasText: 'About' });
    this.politicianStanceLink = page.locator('nav a').filter({ hasText: 'Politician stance' });
    this.communityLink = page.locator('nav a').filter({ hasText: 'Community' });
    this.supportLink = page.locator('nav a').filter({ hasText: 'Support' });
    this.viewAllLink = page.getByRole('link', { name: 'View all' });
    this.logo = page.locator('[data-testid="logo"]');
    this.feedContainer = page.locator('[data-testid="feed-container"]');
    this.createPostButton = page.locator('[data-testid="create-post-button"]');
  }

  async goto() {
    await this.page.goto('/', { waitUntil: 'networkidle' });
  }

  async navigateToAbout() {
    const viewport = this.page.viewportSize();
    if (viewport && viewport.width < 768) {
      await this.mobileHelpers.openMobileMenu();
    }
    await this.aboutLink.click();
  }

  async navigateToPoliticianStance() {
    const viewport = this.page.viewportSize();
    if (viewport && viewport.width < 768) {
      await this.mobileHelpers.openMobileMenu();
    }
    await this.politicianStanceLink.click();
  }

  async navigateToCommunity() {
    const viewport = this.page.viewportSize();
    if (viewport && viewport.width < 768) {
      await this.mobileHelpers.openMobileMenu();
    }
    await this.communityLink.click();
  }

  async navigateToSupport() {
    const viewport = this.page.viewportSize();
    if (viewport && viewport.width < 768) {
      await this.mobileHelpers.openMobileMenu();
    }
    await this.supportLink.click();
  }

  async openSignInModal() {
    await this.authHelpers.openSignInModal();
  }

  async viewAllPoliticians() {
    await this.viewAllLink.click();
    await this.page.waitForURL('**/politicians');
  }

  async waitForFeedToLoad() {
    await this.feedContainer.waitFor({ state: 'visible' });
  }

  async isUserLoggedIn(): Promise<boolean> {
    return this.authHelpers.isLoggedIn();
  }

  async getPageTitle(): Promise<string> {
    return this.page.title();
  }

  async takeScreenshot(name: string) {
    await this.page.screenshot({ 
      path: `test-results/screenshots/home-${name}.png`,
      fullPage: true 
    });
  }

  // Assertions
  async expectToBeOnHomePage() {
    await expect(this.page).toHaveURL(/.*\/(en|ko)?\/?$/);
  }

  async expectSignInButtonVisible() {
    await expect(this.signInButton).toBeVisible();
  }

  async expectUserMenuVisible() {
    await expect(this.userMenu).toBeVisible();
  }

  async expectNavigationVisible() {
    await expect(this.navigationMenu).toBeVisible();
  }

  async expectFeedVisible() {
    await expect(this.feedContainer).toBeVisible();
  }
}