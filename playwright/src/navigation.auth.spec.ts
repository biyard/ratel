import { test, expect } from "@playwright/test";

test.describe("Navigation (Authenticated)", () => {
  test("should navigate to home page via logo", async ({ page }) => {
    await page.goto("/membership");
    await page.getByRole("link", { name: /home/i }).first().click();
    await expect(page).toHaveURL("/");
  });

  test("should navigate to membership page", async ({ page }) => {
    await page.goto("/");

    const membershipLink = page.getByRole("link", { name: /membership/i });
    await expect(membershipLink).toBeVisible();
    await membershipLink.click();
    await expect(page).toHaveURL(/\/membership/);
  });

  test("should open profile dropdown on click", async ({ page }) => {
    await page.goto("/");

    // Click the profile area in the header to open dropdown
    const profileTrigger = page
      .locator("header")
      .getByRole("img")
      .first();
    await profileTrigger.click();

    // Dropdown should show "Log Out" option
    const logoutButton = page.getByText(/log out/i);
    await expect(logoutButton).toBeVisible();
  });

  test("should show teams section in profile dropdown", async ({ page }) => {
    await page.goto("/");

    const profileTrigger = page
      .locator("header")
      .getByRole("img")
      .first();
    await profileTrigger.click();

    const teamsLabel = page.getByText(/teams/i).first();
    await expect(teamsLabel).toBeVisible();
  });

  test("should navigate to create post page", async ({ page }) => {
    await page.goto("/");

    const createPostBtn = page.getByRole("link", {
      name: /create post/i,
    });
    await createPostBtn.click();
    await expect(page).toHaveURL(/\/posts\/.*\/edit/);
  });
});
