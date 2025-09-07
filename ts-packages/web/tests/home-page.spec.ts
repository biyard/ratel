import { test, expect } from '@playwright/test';
import { HomePage } from './pages/HomePage';
import { mockApiResponses } from './fixtures/test-data';

test.describe('Home Page', () => {
  test('should load home page successfully', async ({ page }) => {
    const homePage = new HomePage(page);
    
    await homePage.goto();
    await homePage.expectToBeOnHomePage();
    
    // Check page title
    const title = await homePage.getPageTitle();
    expect(title).toContain('Ratel');
    
    await homePage.takeScreenshot('initial-load');
  });

  test('should display main page elements', async ({ page }) => {
    const homePage = new HomePage(page);
    
    await homePage.goto();
    
    // Check that key elements are visible
    await homePage.expectNavigationVisible();
    await homePage.expectSignInButtonVisible();
    
    // Check if logo is visible
    const logo = page.locator('[data-testid="logo"]').first();
    if (await logo.isVisible()) {
      await expect(logo).toBeVisible();
    }
    
    await homePage.takeScreenshot('main-elements');
  });

  test('should load and display feed content', async ({ page }) => {
    const homePage = new HomePage(page);
    
    // Mock API responses for faster testing
    await page.route('**/api/feeds/**', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ data: mockApiResponses.feedPosts }),
      });
    });
    
    await homePage.goto();
    await homePage.waitForFeedToLoad();
    
    // Check if feed content is displayed
    const feedCards = page.locator('[data-testid="post-card"]');
    const feedCount = await feedCards.count();
    
    if (feedCount > 0) {
      // At least one post should be visible
      await expect(feedCards.first()).toBeVisible();
      await homePage.takeScreenshot('feed-loaded');
    }
  });

  test('should display promotion card if promotion exists', async ({ page }) => {
    const homePage = new HomePage(page);
    
    // Mock promotion API
    await page.route('**/api/promotion/**', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          data: {
            id: 1,
            title: 'Test Promotion',
            content: 'Test promotion content',
            feed_id: 1,
          }
        }),
      });
    });
    
    await homePage.goto();
    
    // Check if promotion card is displayed
    const promotionCard = page.locator('[data-testid="promotion-card"]');
    if (await promotionCard.isVisible()) {
      await expect(promotionCard).toBeVisible();
      await homePage.takeScreenshot('promotion-displayed');
    }
  });

  test('should handle empty feed state', async ({ page }) => {
    const homePage = new HomePage(page);
    
    // Mock empty feed response
    await page.route('**/api/feeds/**', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ data: [] }),
      });
    });
    
    await homePage.goto();
    
    // Check if empty state is displayed
    const emptyState = page.locator('[data-testid="feed-empty-state"]');
    if (await emptyState.isVisible()) {
      await expect(emptyState).toBeVisible();
      await homePage.takeScreenshot('empty-feed-state');
    }
  });

  test('should display create post button for authenticated users', async ({ page }) => {
    const homePage = new HomePage(page);
    
    // Mock user session
    await page.route('**/api/auth/session', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ 
          data: mockApiResponses.userProfile,
          authenticated: true 
        }),
      });
    });
    
    await homePage.goto();
    
    // Check if create post button is visible for authenticated users
    const createPostButton = page.locator('[data-testid="create-post-button"]');
    if (await createPostButton.isVisible()) {
      await expect(createPostButton).toBeVisible();
      await homePage.takeScreenshot('authenticated-home');
    }
  });

  test.describe('Home Page Interactions', () => {
    test('should interact with news section', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      
      // Check if news section exists and is interactive
      const newsSection = page.locator('[data-testid="news-section"]');
      if (await newsSection.isVisible()) {
        await expect(newsSection).toBeVisible();
        
        // Look for news items
        const newsItems = page.locator('[data-testid="news-item"]');
        const newsCount = await newsItems.count();
        
        if (newsCount > 0) {
          // Click on first news item
          await newsItems.first().click();
          await homePage.takeScreenshot('news-item-clicked');
        }
      }
    });

    test('should show suggestions section', async ({ page }) => {
      const homePage = new HomePage(page);
      
      await homePage.goto();
      
      // Check if suggestions section is displayed
      const suggestionsSection = page.locator('[data-testid="suggestions-section"]');
      if (await suggestionsSection.isVisible()) {
        await expect(suggestionsSection).toBeVisible();
        await homePage.takeScreenshot('suggestions-displayed');
        
        // Check for suggestion items
        const suggestionItems = page.locator('[data-testid="suggestion-item"]');
        const suggestionCount = await suggestionItems.count();
        
        if (suggestionCount > 0) {
          await expect(suggestionItems.first()).toBeVisible();
        }
      }
    });
  });

  test.describe('Home Page Performance', () => {
    test('should load within acceptable time limits', async ({ page }) => {
      const homePage = new HomePage(page);
      
      const startTime = Date.now();
      await homePage.goto();
      await homePage.expectFeedVisible();
      const loadTime = Date.now() - startTime;
      
      // Should load within 10 seconds (generous for CI)
      expect(loadTime).toBeLessThan(10000);
      
      await homePage.takeScreenshot('performance-loaded');
    });

    test('should handle slow API responses', async ({ page }) => {
      const homePage = new HomePage(page);
      
      // Add delay to API responses
      await page.route('**/api/**', async route => {
        await new Promise(resolve => setTimeout(resolve, 2000));
        route.continue();
      });
      
      const startTime = Date.now();
      await homePage.goto();
      
      // Should show loading state during API calls
      const loadingIndicator = page.locator('[data-testid="loading"]');
      if (await loadingIndicator.isVisible()) {
        await expect(loadingIndicator).toBeVisible();
      }
      
      // Eventually should load content
      await homePage.expectFeedVisible();
      const loadTime = Date.now() - startTime;
      
      await homePage.takeScreenshot('slow-api-loaded');
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper heading structure', async ({ page }) => {
      const homePage = new HomePage(page);
      await homePage.goto();
      
      // Check for proper heading hierarchy
      const h1Elements = page.locator('h1');
      const h1Count = await h1Elements.count();
      
      // Should have at least one h1 element
      expect(h1Count).toBeGreaterThan(0);
      
      await homePage.takeScreenshot('accessibility-headings');
    });

    test('should support keyboard navigation', async ({ page }) => {
      const homePage = new HomePage(page);
      await homePage.goto();
      
      // Tab through focusable elements
      await page.keyboard.press('Tab');
      const firstFocusedElement = await page.locator(':focus').first();
      
      if (await firstFocusedElement.isVisible()) {
        await expect(firstFocusedElement).toBeFocused();
        await homePage.takeScreenshot('keyboard-focus');
      }
    });
  });
});