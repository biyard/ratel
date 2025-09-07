import { test, expect } from '@playwright/test';
import { HomePage } from './pages/HomePage';
import { PoliticiansPage } from './pages/PoliticiansPage';

test.describe('Navigation', () => {
  test('should navigate through main sections from home page', async ({ page }) => {
    const homePage = new HomePage(page);
    
    await homePage.goto();
    await homePage.expectToBeOnHomePage();
    await homePage.expectNavigationVisible();
    
    // Test About navigation
    await homePage.navigateToAbout();
    await homePage.takeScreenshot('about-page');
    
    // Go back to home
    await homePage.goto();
    
    // Test Politician Stance navigation
    await homePage.navigateToPoliticianStance();
    await homePage.takeScreenshot('politician-stance-page');
    
    // Go back to home
    await homePage.goto();
    
    // Test Community navigation
    await homePage.navigateToCommunity();
    await homePage.takeScreenshot('community-page');
    
    // Go back to home
    await homePage.goto();
    
    // Test Support navigation
    await homePage.navigateToSupport();
    await homePage.takeScreenshot('support-page');
  });

  test('should navigate to politicians page and back', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Start at home page
    await homePage.goto();
    await homePage.expectToBeOnHomePage();
    
    // Navigate to politicians via "View all" link
    await homePage.viewAllPoliticians();
    await politiciansPage.expectToBeOnPoliticiansPage();
    await politiciansPage.waitForPageLoad();
    await politiciansPage.takeScreenshot('politicians-list');
    
    // Navigate back to home
    await politiciansPage.backToHome();
    await homePage.expectToBeOnHomePage();
  });

  test('should show politician details when clicking on politician', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Navigate to politicians page
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    
    const politicianCount = await politiciansPage.getPoliticianCount();
    
    if (politicianCount > 0) {
      // Click on first politician
      await politiciansPage.selectFirstPolitician();
      
      // Should navigate to politician detail page
      await expect(page).toHaveURL(/.*politicians\/.+/);
      await page.screenshot({ path: 'test-results/screenshots/politician-detail.png' });
    }
  });

  test.describe('Mobile Navigation', () => {
    test.use({ viewport: { width: 375, height: 667 } });

    test('should work correctly on mobile devices', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      await homePage.expectToBeOnHomePage();
      
      // Mobile menu should be accessible
      const mobileMenuButton = page.locator('[data-testid="mobile-menu-button"]');
      if (await mobileMenuButton.isVisible()) {
        await mobileMenuButton.click();
        
        // Mobile menu should open
        const mobileMenu = page.locator('[data-testid="mobile-menu"]');
        await expect(mobileMenu).toBeVisible();
        
        await homePage.takeScreenshot('mobile-menu-open');
        
        // Should be able to navigate using mobile menu
        await homePage.navigateToAbout();
        await homePage.takeScreenshot('mobile-about-page');
      }
    });
  });

  test.describe('Responsive Navigation', () => {
    [
      { name: 'tablet', width: 768, height: 1024 },
      { name: 'desktop', width: 1280, height: 720 },
      { name: 'large', width: 1920, height: 1080 },
    ].forEach(({ name, width, height }) => {
      test(`should work correctly on ${name} viewport`, async ({ page }) => {
        await page.setViewportSize({ width, height });
        
        const homePage = new HomePage(page);
        await homePage.goto();
        await homePage.expectNavigationVisible();
        await homePage.takeScreenshot(`${name}-home-page`);
        
        // Test navigation to politicians page
        await homePage.viewAllPoliticians();
        const politiciansPage = new PoliticiansPage(page);
        await politiciansPage.expectToBeOnPoliticiansPage();
        await politiciansPage.takeScreenshot(`${name}-politicians-page`);
      });
    });
  });

  test('should handle page loading states', async ({ page }) => {
    const homePage = new HomePage(page);
    
    // Monitor network requests
    const apiRequests: string[] = [];
    page.on('request', request => {
      if (request.url().includes('/api/') || request.url().includes('/v1/')) {
        apiRequests.push(request.url());
      }
    });
    
    await homePage.goto();
    
    // Wait for any loading states to complete
    const loadingIndicator = page.locator('[data-testid="loading"]');
    if (await loadingIndicator.isVisible()) {
      await expect(loadingIndicator).toBeHidden();
    }
    
    await homePage.expectFeedVisible();
    await homePage.takeScreenshot('loaded-home-page');
    
    // Should have made some API requests
    expect(apiRequests.length).toBeGreaterThan(0);
  });

  test('should handle network errors gracefully', async ({ page }) => {
    // Simulate network failure for API calls
    await page.route('**/api/**', route => {
      route.abort('failed');
    });
    
    await page.route('**/v1/**', route => {
      route.abort('failed');
    });
    
    const homePage = new HomePage(page);
    await homePage.goto();
    
    // App should still load even with API failures
    await homePage.expectToBeOnHomePage();
    await homePage.takeScreenshot('network-error-state');
    
    // Check if error messages are displayed appropriately
    const errorMessage = page.locator('[data-testid="error-message"]');
    if (await errorMessage.isVisible()) {
      await expect(errorMessage).toContainText(/error|failed|unavailable/i);
    }
  });
});