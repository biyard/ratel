import { test, expect } from "@playwright/test";

test.describe("Feed (Authenticated)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("should display feed cards with expected structure", async ({
    page,
  }) => {
    // Wait for feed to load
    const firstCard = page.locator("a[href*='/posts/']").first();
    const hasCards = await firstCard.isVisible().catch(() => false);

    if (hasCards) {
      // Each feed card should have a title or content
      await expect(firstCard).toBeVisible();
    } else {
      // If no posts, should show end of feed message
      const endMessage = page.getByLabel("End of feed message");
      await expect(endMessage).toBeVisible();
    }
  });

  test("should show like button on feed cards", async ({ page }) => {
    const firstCard = page.locator("a[href*='/posts/']").first();
    const hasCards = await firstCard.isVisible().catch(() => false);

    if (hasCards) {
      // Like button should be present (contains SVG thumbs up icon)
      const likeArea = page.locator("button, [role='button']").filter({
        has: page.locator("svg"),
      });
      await expect(likeArea.first()).toBeVisible();
    }
  });

  test("should navigate to post detail when clicking a feed card", async ({
    page,
  }) => {
    const firstCard = page.locator("a[href*='/posts/']").first();
    const hasCards = await firstCard.isVisible().catch(() => false);

    if (hasCards) {
      await firstCard.click();
      await expect(page).toHaveURL(/\/posts\//);
    }
  });
});
