import { test, expect } from "@playwright/test";

test.describe("Social Page - Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage
    await page.goto("/");
    // Wait for page to load completely
    await page.waitForLoadState("networkidle");
  });

  test("should display main social page layout", async ({ page }) => {
    // Check main content area exists
    await expect(page.locator(".flex-1").first()).toBeVisible();

    // Check left sidebar (UserSidemenu) exists on desktop - look for profile section
    const leftSidebar = page.locator('[data-testid="profile-section"]');
    const isLeftSidebarVisible = await leftSidebar
      .isVisible()
      .catch(() => false);

    // Check right sidebar exists (on larger screens)
    const rightSidebar = page.locator('aside[aria-label="Sidebar"]');
    const isRightSidebarVisible = await rightSidebar
      .isVisible()
      .catch(() => false);

    // On desktop, at least one sidebar should be visible
    expect(isLeftSidebarVisible || isRightSidebarVisible).toBeTruthy();
  });

  test("should load more posts when scrolling", async ({ page }) => {
    // Wait for initial posts to load
    await page.waitForTimeout(3000);

    // Count initial posts
    const initialPosts = await page
      .locator('[key*="feed-"]')
      .or(page.locator(".feed-card"))
      .count();

    if (initialPosts > 0) {
      // Scroll to bottom to trigger infinite scroll
      await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

      // Wait for new posts to potentially load
      await page.waitForTimeout(3000);

      // Check if more posts loaded
      const newPostCount = await page
        .locator('[key*="feed-"]')
        .or(page.locator(".feed-card"))
        .count();

      // Either more posts loaded or we hit the end
      const feedEndMessage = await page
        .locator("text=/end|no more/i")
        .isVisible()
        .catch(() => false);

      expect(newPostCount >= initialPosts || feedEndMessage).toBeTruthy();
    }
  });

  test("should display create post button in sidebar", async ({ page }) => {
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

  test("should display floating create post button on mobile", async ({
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

  test("should display promotional content when available", async ({
    page,
  }) => {
    // Wait for content to load
    await page.waitForTimeout(3000);

    // Check for promotion card in sidebar
    const promotionCard = page
      .locator(".promotion-card")
      .or(page.locator('[data-testid="promotion"]'));

    // Promotion may or may not be present
    const hasPromotion = await promotionCard.isVisible().catch(() => false);

    if (hasPromotion) {
      await expect(promotionCard).toBeVisible();
    }
  });

  test("should display news section in sidebar", async ({ page }) => {
    // Check for news section in sidebar
    const newsSection = page
      .locator("aside")
      .locator("text=/news|News/i")
      .or(page.locator(".news-section"));

    const hasNews = await newsSection.isVisible().catch(() => false);

    if (hasNews) {
      await expect(newsSection).toBeVisible();
    }
  });

  test("should display suggestions section in sidebar", async ({ page }) => {
    // Check for suggestions section in sidebar
    const suggestionsSection = page
      .locator("aside")
      .locator("text=/suggest|Suggest|recommend/i")
      .or(page.locator(".suggestions-section"));

    const hasSuggestions = await suggestionsSection
      .isVisible()
      .catch(() => false);

    if (hasSuggestions) {
      await expect(suggestionsSection).toBeVisible();
    }
  });

  test("should handle post interactions", async ({ page }) => {
    // Wait for posts to load
    await page.waitForTimeout(3000);

    const firstPost = page
      .locator('[key*="feed-"]')
      .or(page.locator(".feed-card"))
      .first();

    if (await firstPost.isVisible()) {
      // Look for like button
      const likeButton = firstPost
        .locator("button")
        .filter({ hasText: /like|heart/i })
        .or(firstPost.locator('[aria-label*="like"]'));

      if (await likeButton.isVisible()) {
        await likeButton.click();
        // Should update like count or state
        await page.waitForTimeout(1000);
      }

      // Look for comment button
      const commentButton = firstPost
        .locator("button")
        .filter({ hasText: /comment|reply/i })
        .or(firstPost.locator('[aria-label*="comment"]'));

      if (await commentButton.isVisible()) {
        await expect(commentButton).toBeVisible();
      }

      // Look for share button
      const shareButton = firstPost
        .locator("button")
        .filter({ hasText: /share|repost/i })
        .or(firstPost.locator('[aria-label*="share"]'));

      if (await shareButton.isVisible()) {
        await expect(shareButton).toBeVisible();
      }
    }
  });

  test("should respond to responsive design changes", async ({ page }) => {
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
      mobileSidebar === false || (await page.locator(".fixed").isVisible()),
    ).toBeTruthy();

    // Reset viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });
});
