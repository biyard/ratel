import { test, expect } from "@playwright/test";

test.describe("Home Page (Authenticated)", () => {
  test("should load home page successfully", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveURL("/");
  });

  test("should display navigation menu", async ({ page }) => {
    await page.goto("/");

    const homeLink = page.getByRole("link", { name: /home/i });
    await expect(homeLink).toBeVisible();
  });

  test("should display profile dropdown instead of sign in button", async ({
    page,
  }) => {
    await page.goto("/");

    // Authenticated user should NOT see "Sign In" button
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await expect(signInButton).not.toBeVisible();
  });

  test("should display create post button", async ({ page }) => {
    await page.goto("/");

    const createPostBtn = page.getByRole("link", {
      name: /create post/i,
    });
    await expect(createPostBtn).toBeVisible();
  });

  test("should display feed list", async ({ page }) => {
    await page.goto("/");

    // Feed should render at least one post or show end-of-feed message
    const feedContent = page
      .locator("article, [aria-label='End of feed message']")
      .first();
    await expect(feedContent).toBeVisible();
  });
});
