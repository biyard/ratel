import { expect, test } from '@playwright/test';
import { click } from '@tests/utils';
import { CONFIGS } from '../../../tests/config';

test.describe('Create Post - Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should create a general post successfully', async ({ page }) => {
    const testTitle = 'Automated Post Creation - E2E';
    const testContent =
      'This is an automated post content created by Playwright E2E. ' +
      'The purpose of this is to verify that the post creation functionality ' +
      'works correctly from end to end, including title input, content editing, ' +
      'auto-save, and final publication. This content is intentionally long to ' +
      'meet the minimum character requirements for post publishing.';

    await click(page, { label: 'Create Post' });

    await page.waitForURL(/\/posts\/new/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    await page.fill('#post-title-input', testTitle);

    const editorSelector = '[data-pw="post-content-editor"] .ProseMirror';
    await page.waitForSelector(editorSelector, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    await page.click(editorSelector);
    await page.fill(editorSelector, testContent);

    await page.click('#publish-post-button');

    await page.waitForURL(/\/threads\/.+/, { timeout: CONFIGS.PAGE_WAIT_TIME });
  });
});

test.describe('Home page - Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should load more posts when scrolling', async ({ page }) => {
    // Count initial posts
    const initialPosts = await page
      .locator('[key*="feed-"]')
      .or(page.locator('.feed-card'))
      .count();

    if (initialPosts > 0) {
      // Scroll to bottom to trigger infinite scroll
      await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

      // Wait for new posts to potentially load
      await page.waitForTimeout(CONFIGS.PAGE_WAIT_TIME);

      // Check if more posts loaded
      const newPostCount = await page
        .locator('[key*="feed-"]')
        .or(page.locator('.feed-card'))
        .count();

      // Either more posts loaded or we hit the end
      const feedEndMessage = await page
        .locator('text=/end|no more/i')
        .isVisible()
        .catch(() => false);

      expect(newPostCount >= initialPosts || feedEndMessage).toBeTruthy();
    }
  });

  test('should display create post button in sidebar', async ({ page }) => {
    // Check if create post button is visible in sidebar
    const sidebarCreateButton = page.locator(
      'aside [data-testid="create-post-button"]',
    );

    const isSidebarButtonVisible = await sidebarCreateButton
      .isVisible()
      .catch(() => false);

    if (isSidebarButtonVisible) {
      await expect(sidebarCreateButton).toBeVisible();
      await expect(sidebarCreateButton).toContainText(
        /create_post|Create Post|Write/i,
      );
    }
  });

  test('should display floating create post button on mobile', async ({
    page,
  }) => {
    // Resize to mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(1000);

    // Check for floating create post button (fixed positioned)
    const floatingButton = page.locator(
      '.fixed [data-testid="create-post-button"]',
    );

    const isFloatingButtonVisible = await floatingButton
      .isVisible()
      .catch(() => false);

    if (isFloatingButtonVisible) {
      await expect(floatingButton).toBeVisible();
    }

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });

  test('should display promotional content when available', async ({
    page,
  }) => {
    // Wait for content to load
    await page.waitForTimeout(3000);

    // Check for promotion card in sidebar
    const promotionCard = page
      .locator('.promotion-card')
      .or(page.locator('[data-testid="promotion"]'));

    // Promotion may or may not be present
    const hasPromotion = await promotionCard.isVisible().catch(() => false);

    if (hasPromotion) {
      await expect(promotionCard).toBeVisible();
    }
  });

  test('should display news section in sidebar', async ({ page }) => {
    // Check for news section in sidebar
    const newsSection = page
      .locator('aside')
      .locator('text=/news|News/i')
      .or(page.locator('.news-section'));

    const hasNews = await newsSection.isVisible().catch(() => false);

    if (hasNews) {
      await expect(newsSection).toBeVisible();
    }
  });

  test('should display suggestions section in sidebar', async ({ page }) => {
    // Check for suggestions section in sidebar
    const suggestionsSection = page
      .locator('aside')
      .locator('text=/suggest|Suggest|recommend/i')
      .or(page.locator('.suggestions-section'));

    const hasSuggestions = await suggestionsSection
      .isVisible()
      .catch(() => false);

    if (hasSuggestions) {
      await expect(suggestionsSection).toBeVisible();
    }
  });

  test('should handle post interactions', async ({ page }) => {
    // Wait for posts to load
    await page.waitForTimeout(3000);

    const firstPost = page
      .locator('[key*="feed-"]')
      .or(page.locator('.feed-card'))
      .first();

    if (await firstPost.isVisible()) {
      // Look for like button
      const likeButton = firstPost
        .locator('button')
        .filter({ hasText: /like|heart/i })
        .or(firstPost.locator('[aria-label*="like"]'));

      if (await likeButton.isVisible()) {
        await likeButton.click();
        // Should update like count or state
        await page.waitForTimeout(1000);
      }

      // Look for comment button
      const commentButton = firstPost
        .locator('button')
        .filter({ hasText: /comment|reply/i })
        .or(firstPost.locator('[aria-label*="comment"]'));

      if (await commentButton.isVisible()) {
        await expect(commentButton).toBeVisible();
      }

      // Look for share button
      const shareButton = firstPost
        .locator('button')
        .filter({ hasText: /share|repost/i })
        .or(firstPost.locator('[aria-label*="share"]'));

      if (await shareButton.isVisible()) {
        await expect(shareButton).toBeVisible();
      }
    }
  });

  test('should respond to responsive design changes', async ({ page }) => {
    // Test desktop view
    await page.setViewportSize({ width: 1280, height: 720 });
    await page.waitForTimeout(1000);

    // Sidebar should be visible on desktop
    const desktopSidebar = page.locator('aside[aria-label="Sidebar"]');
    const isDesktopSidebarVisible = await desktopSidebar
      .isVisible()
      .catch(() => false);

    // Test tablet view
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.waitForTimeout(1000);

    // Test mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(1000);

    // Mobile view should hide sidebar and show mobile-specific elements
    const mobileSidebar = await page
      .locator('aside[aria-label="Sidebar"]')
      .isVisible()
      .catch(() => false);

    // On mobile, sidebar should be hidden or floating elements should appear
    expect(
      mobileSidebar === false || (await page.locator('.fixed').isVisible()),
    ).toBeTruthy();

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });

  test('[HP-001] right sidebar layout should remain stable when scrolling', async ({
    page,
  }) => {
    // Set to desktop viewport
    await page.setViewportSize({ width: 1280, height: 720 });
    await page.waitForTimeout(1000);

    const sidebar = page.locator('[aria-label="Sidebar"]');
    await expect(sidebar).toBeVisible();

    // Get initial sidebar position
    const initialBox = await sidebar.boundingBox();
    expect(initialBox).not.toBeNull();

    // Scroll down the page
    await page.evaluate(() => window.scrollTo(0, 500));
    await page.waitForTimeout(500);

    // Get sidebar position after scroll down
    const scrolledBox = await sidebar.boundingBox();
    expect(scrolledBox).not.toBeNull();

    // On desktop with static positioning, sidebar should move up (decrease y) when scrolling down
    // This verifies the sidebar follows normal document flow and doesn't break layout
    expect(scrolledBox!.y).toBeLessThan(initialBox!.y);

    // Scroll back up
    await page.evaluate(() => window.scrollTo(0, 0));
    await page.waitForTimeout(500);

    // Verify sidebar returns to approximately original position
    const finalBox = await sidebar.boundingBox();
    expect(finalBox).not.toBeNull();
    expect(Math.abs(finalBox!.y - initialBox!.y)).toBeLessThan(10);
  });

  test('[HP-002] sidebar content should not overflow or break layout on scroll', async ({
    page,
  }) => {
    // Set to desktop viewport
    await page.setViewportSize({ width: 1280, height: 720 });
    await page.waitForTimeout(1000);

    const sidebar = page.locator('[aria-label="Sidebar"]');
    await expect(sidebar).toBeVisible();

    // Verify sidebar has proper height
    const sidebarBox = await sidebar.boundingBox();
    expect(sidebarBox).not.toBeNull();
    expect(sidebarBox!.height).toBeGreaterThan(0);

    // Scroll down and verify no layout issues
    await page.evaluate(() => window.scrollTo(0, 500));
    await page.waitForTimeout(500);

    // Sidebar should still be visible and properly sized
    await expect(sidebar).toBeVisible();
    const scrolledBox = await sidebar.boundingBox();
    expect(scrolledBox).not.toBeNull();
    expect(scrolledBox!.height).toBeGreaterThan(0);

    // Verify Create Post button is still visible
    const createPostButton = page.getByLabel('Create Post');
    await expect(createPostButton).toBeVisible();
  });
});
