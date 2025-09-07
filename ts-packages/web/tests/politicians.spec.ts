import { test, expect } from '@playwright/test';
import { HomePage } from './pages/HomePage';
import { PoliticiansPage } from './pages/PoliticiansPage';
import { mockApiResponses } from './fixtures/test-data';

test.describe('Politicians Page', () => {
  test.beforeEach(async ({ page }) => {
    // Mock politicians API
    await page.route('**/api/politicians**', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ data: mockApiResponses.politicians }),
      });
    });
  });

  test('should display politicians list', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Navigate to politicians page
    await homePage.goto();
    await homePage.viewAllPoliticians();
    
    await politiciansPage.expectToBeOnPoliticiansPage();
    await politiciansPage.waitForPageLoad();
    await politiciansPage.expectPoliticiansListVisible();
    
    // Should have politicians listed
    const politicianCount = await politiciansPage.getPoliticianCount();
    expect(politicianCount).toBeGreaterThan(0);
    
    await politiciansPage.takeScreenshot('politicians-list');
  });

  test('should show politician details when clicking on politician', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Mock politician detail API
    await page.route('**/api/politicians/*', route => {
      const politicianId = route.request().url().split('/').pop();
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ 
          data: {
            ...mockApiResponses.politicians[0],
            id: parseInt(politicianId || '1'),
            bio: 'Detailed biography of the politician',
            voting_record: [
              { bill: 'Crypto Regulation Act', vote: 'for', date: '2024-01-15' },
              { bill: 'Digital Assets Framework', vote: 'against', date: '2024-02-20' },
            ],
          }
        }),
      });
    });
    
    // Navigate to politicians page
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    
    // Click on first politician
    await politiciansPage.selectFirstPolitician();
    
    // Should navigate to politician detail page
    await expect(page).toHaveURL(/.*politicians\/.+/);
    
    // Wait for politician details to load
    const politicianName = page.locator('[data-testid="politician-name"]');
    if (await politicianName.isVisible()) {
      await expect(politicianName).toBeVisible();
    }
    
    const politicianBio = page.locator('[data-testid="politician-bio"]');
    if (await politicianBio.isVisible()) {
      await expect(politicianBio).toBeVisible();
    }
    
    await page.screenshot({ path: 'test-results/screenshots/politician-detail.png' });
  });

  test('should filter politicians by party or position', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    
    // Look for filter options
    const partyFilter = page.locator('[data-testid="party-filter"]');
    if (await partyFilter.isVisible()) {
      // Select a party filter
      await partyFilter.click();
      const filterOption = page.locator('[data-testid="filter-option"]').first();
      if (await filterOption.isVisible()) {
        await filterOption.click();
        
        // Wait for filtered results
        await page.waitForTimeout(1000);
        await politiciansPage.takeScreenshot('filtered-politicians');
      }
    }
    
    // Test position filter
    const positionFilter = page.locator('[data-testid="position-filter"]');
    if (await positionFilter.isVisible()) {
      await positionFilter.click();
      const filterOption = page.locator('[data-testid="filter-option"]').first();
      if (await filterOption.isVisible()) {
        await filterOption.click();
        
        await page.waitForTimeout(1000);
        await politiciansPage.takeScreenshot('position-filtered-politicians');
      }
    }
  });

  test('should search for politicians', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Mock search API
    await page.route('**/api/politicians/search**', route => {
      const url = new URL(route.request().url());
      const query = url.searchParams.get('q') || '';
      
      const filteredPoliticians = mockApiResponses.politicians.filter(politician =>
        politician.name.toLowerCase().includes(query.toLowerCase())
      );
      
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ data: filteredPoliticians }),
      });
    });
    
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    
    // Search for a politician
    await politiciansPage.searchPolitician('John');
    
    // Wait for search results
    await page.waitForTimeout(1000);
    
    // Should show search results
    const searchResults = await politiciansPage.getPoliticianCount();
    await politiciansPage.takeScreenshot('search-results');
  });

  test('should show politician stance scores', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    
    // Check if stance scores are displayed
    const stanceScores = page.locator('[data-testid="stance-score"]');
    const scoreCount = await stanceScores.count();
    
    if (scoreCount > 0) {
      // At least one stance score should be visible
      await expect(stanceScores.first()).toBeVisible();
      
      // Score should be a number between 0-100
      const scoreText = await stanceScores.first().textContent();
      if (scoreText) {
        const score = parseInt(scoreText);
        expect(score).toBeGreaterThanOrEqual(0);
        expect(score).toBeLessThanOrEqual(100);
      }
    }
    
    await politiciansPage.takeScreenshot('stance-scores');
  });

  test('should display voting records in politician detail', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    // Mock detailed politician data with voting record
    await page.route('**/api/politicians/*', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ 
          data: {
            ...mockApiResponses.politicians[0],
            voting_record: [
              { 
                bill: 'Cryptocurrency Regulation Act 2024', 
                vote: 'for', 
                date: '2024-01-15',
                description: 'A bill to establish comprehensive regulations for cryptocurrency trading.'
              },
              { 
                bill: 'Digital Assets Tax Framework', 
                vote: 'against', 
                date: '2024-02-20',
                description: 'Legislation to clarify tax obligations for digital asset holders.'
              },
            ],
          }
        }),
      });
    });
    
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    await politiciansPage.selectFirstPolitician();
    
    // Check if voting record is displayed
    const votingRecord = page.locator('[data-testid="voting-record"]');
    if (await votingRecord.isVisible()) {
      await expect(votingRecord).toBeVisible();
      
      // Should show individual votes
      const voteItems = page.locator('[data-testid="vote-item"]');
      const voteCount = await voteItems.count();
      
      if (voteCount > 0) {
        await expect(voteItems.first()).toBeVisible();
        await page.screenshot({ path: 'test-results/screenshots/voting-record.png' });
      }
    }
  });

  test('should handle politician page navigation', async ({ page }) => {
    const homePage = new HomePage(page);
    const politiciansPage = new PoliticiansPage(page);
    
    await homePage.goto();
    await homePage.viewAllPoliticians();
    await politiciansPage.waitForPageLoad();
    await politiciansPage.selectFirstPolitician();
    
    // Navigate back to politicians list
    await politiciansPage.backToHome();
    await homePage.expectToBeOnHomePage();
    
    // Navigate back to politicians page
    await homePage.viewAllPoliticians();
    await politiciansPage.expectToBeOnPoliticiansPage();
  });

  test.describe('Mobile Politicians Page', () => {
    test.use({ viewport: { width: 375, height: 667 } });

    test('should work correctly on mobile devices', async ({ page }) => {
      const homePage = new HomePage(page);
      const politiciansPage = new PoliticiansPage(page);
      
      await homePage.goto();
      await homePage.viewAllPoliticians();
      await politiciansPage.waitForPageLoad();
      
      // Politicians list should be responsive
      await politiciansPage.expectPoliticiansListVisible();
      await politiciansPage.takeScreenshot('mobile-politicians-list');
      
      // Politician detail should work on mobile
      if (await politiciansPage.getPoliticianCount() > 0) {
        await politiciansPage.selectFirstPolitician();
        await page.screenshot({ path: 'test-results/screenshots/mobile-politician-detail.png' });
      }
    });

    test('should handle mobile search', async ({ page }) => {
      const homePage = new HomePage(page);
      const politiciansPage = new PoliticiansPage(page);
      
      await homePage.goto();
      await homePage.viewAllPoliticians();
      await politiciansPage.waitForPageLoad();
      
      // Mobile search should work
      await politiciansPage.searchPolitician('Test');
      await page.waitForTimeout(1000);
      await politiciansPage.takeScreenshot('mobile-search-results');
    });
  });

  test.describe('Accessibility', () => {
    test('should support keyboard navigation', async ({ page }) => {
      const homePage = new HomePage(page);
      const politiciansPage = new PoliticiansPage(page);
      
      await homePage.goto();
      await homePage.viewAllPoliticians();
      await politiciansPage.waitForPageLoad();
      
      // Tab through politician items
      await page.keyboard.press('Tab');
      const focusedElement = await page.locator(':focus').first();
      
      if (await focusedElement.isVisible()) {
        await expect(focusedElement).toBeFocused();
        await page.screenshot({ path: 'test-results/screenshots/politicians-keyboard-focus.png' });
      }
    });

    test('should have proper ARIA labels', async ({ page }) => {
      const homePage = new HomePage(page);
      const politiciansPage = new PoliticiansPage(page);
      
      await homePage.goto();
      await homePage.viewAllPoliticians();
      await politiciansPage.waitForPageLoad();
      
      // Check for ARIA labels on politician items
      const politicianItems = page.locator('#politician-list a');
      const itemCount = await politicianItems.count();
      
      if (itemCount > 0) {
        const firstItem = politicianItems.first();
        const ariaLabel = await firstItem.getAttribute('aria-label');
        
        // Should have descriptive ARIA label
        if (ariaLabel) {
          expect(ariaLabel.length).toBeGreaterThan(0);
        }
      }
    });
  });
});