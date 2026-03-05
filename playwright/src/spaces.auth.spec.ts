import { test, expect } from "@playwright/test";

test.describe("Spaces (Authenticated)", () => {
  test("should load spaces page", async ({ page }) => {
    await page.goto("/spaces");
    await expect(page).toHaveURL(/\/spaces/);
  });

  test("should display space content or empty state", async ({ page }) => {
    await page.goto("/spaces");

    // Page should have some visible content
    await expect(page.locator("body")).not.toBeEmpty();
  });
});
