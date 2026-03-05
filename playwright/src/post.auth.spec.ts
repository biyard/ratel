import { test, expect } from "@playwright/test";

test.describe("Post Creation (Authenticated)", () => {
  test("should navigate to post editor from create button", async ({
    page,
  }) => {
    await page.goto("/");

    const createPostBtn = page.getByRole("link", {
      name: /create post/i,
    });
    await expect(createPostBtn).toBeVisible();
    await createPostBtn.click();

    await expect(page).toHaveURL(/\/posts\/.*\/edit/);
  });

  test("should display post editor elements", async ({ page }) => {
    await page.goto("/");

    const createPostBtn = page.getByRole("link", {
      name: /create post/i,
    });
    await createPostBtn.click();
    await expect(page).toHaveURL(/\/posts\/.*\/edit/);

    // Editor page should have content area
    const editorArea = page.locator(
      "[contenteditable], textarea, [role='textbox']",
    );
    await expect(editorArea.first()).toBeVisible();
  });
});

test.describe("Post Detail (Authenticated)", () => {
  test("should display post content on detail page", async ({ page }) => {
    await page.goto("/");

    const firstCard = page.locator("a[href*='/posts/']").first();
    const hasCards = await firstCard.isVisible().catch(() => false);

    if (hasCards) {
      const href = await firstCard.getAttribute("href");
      await page.goto(href!);

      // Post detail should have content
      await expect(page.locator("main, article, section").first()).toBeVisible();
    }
  });
});
