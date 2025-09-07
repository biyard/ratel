import { Page, Locator, expect } from '@playwright/test';
import { TestHelpers } from '../utils/test-helpers';

export class PoliticiansPage {
  private helpers: TestHelpers;

  // Locators
  readonly pageTitle: Locator;
  readonly politicianList: Locator;
  readonly politicianItems: Locator;
  readonly searchInput: Locator;
  readonly filterButtons: Locator;
  readonly backToHomeLink: Locator;

  constructor(private page: Page) {
    this.helpers = new TestHelpers(page);

    // Initialize locators
    this.pageTitle = page.locator('div:text("Politician Stance")');
    this.politicianList = page.locator('#politician-list');
    this.politicianItems = page.locator('#politician-list a');
    this.searchInput = page.locator('[data-testid="politician-search"]');
    this.filterButtons = page.locator('[data-testid="filter-button"]');
    this.backToHomeLink = page.locator('[data-testid="back-to-home"]');
  }

  async goto() {
    await this.page.goto('/politicians', { waitUntil: 'networkidle' });
  }

  async waitForPageLoad() {
    await this.pageTitle.waitFor({ state: 'visible' });
    await this.politicianList.waitFor({ state: 'visible' });
  }

  async selectFirstPolitician() {
    const firstPolitician = this.politicianItems.first();
    await firstPolitician.click();
  }

  async searchPolitician(query: string) {
    if (await this.searchInput.isVisible()) {
      await this.searchInput.fill(query);
      await this.page.keyboard.press('Enter');
    }
  }

  async getPoliticianCount(): Promise<number> {
    return this.politicianItems.count();
  }

  async backToHome() {
    if (await this.backToHomeLink.isVisible()) {
      await this.backToHomeLink.click();
      await this.page.waitForURL('**/');
    }
  }

  async takeScreenshot(name: string) {
    await this.page.screenshot({ 
      path: `test-results/screenshots/politicians-${name}.png`,
      fullPage: true 
    });
  }

  // Assertions
  async expectToBeOnPoliticiansPage() {
    await expect(this.page).toHaveURL(/.*politicians/);
  }

  async expectPoliticiansListVisible() {
    await expect(this.politicianList).toBeVisible();
  }

  async expectPageTitleVisible() {
    await expect(this.pageTitle).toBeVisible();
  }
}