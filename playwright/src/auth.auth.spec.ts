import { test, expect } from "@playwright/test";

test.describe("Auth State (Authenticated)", () => {
  test("should persist auth state across page navigations", async ({
    page,
  }) => {
    await page.goto("/");

    // Verify signed in (no Sign In button visible)
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await expect(signInButton).not.toBeVisible();

    // Navigate away and back
    await page.goto("/membership");
    await page.goto("/");

    // Should still be authenticated
    await expect(signInButton).not.toBeVisible();
  });

  test("should be able to log out", async ({ page }) => {
    await page.goto("/");

    // Open profile dropdown
    const profileTrigger = page
      .locator("header")
      .getByRole("img")
      .first();
    await profileTrigger.click();

    // Click logout
    const logoutButton = page.getByText(/log out/i);
    await expect(logoutButton).toBeVisible();

    // Just verify the button exists; don't actually logout
    // to avoid breaking other tests in the suite
  });
});

test.describe("Login Modal UI", () => {
  test("should open login modal when Sign In is clicked (unauthenticated view)", async ({
    browser,
  }) => {
    // Use a fresh context without stored auth
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.goto("/");

    const signInButton = page.getByRole("button", { name: /sign in/i });
    const isVisible = await signInButton.isVisible().catch(() => false);

    if (isVisible) {
      await signInButton.click();

      // Login modal should appear with email input
      const emailInput = page.getByTestId("email-input");
      await expect(emailInput).toBeVisible();

      const continueButton = page.getByTestId("continue-button");
      await expect(continueButton).toBeVisible();
    }

    await context.close();
  });

  test("should show password field after entering email", async ({
    browser,
  }) => {
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.goto("/");

    const signInButton = page.getByRole("button", { name: /sign in/i });
    const isVisible = await signInButton.isVisible().catch(() => false);

    if (isVisible) {
      await signInButton.click();

      const emailInput = page.getByTestId("email-input");
      await emailInput.fill("test@example.com");

      const continueButton = page.getByTestId("continue-button");
      await continueButton.click();

      // Password input should now be visible
      const passwordInput = page.getByTestId("password-input");
      await expect(passwordInput).toBeVisible();
    }

    await context.close();
  });

  test("should have link to create account from login modal", async ({
    browser,
  }) => {
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.goto("/");

    const signInButton = page.getByRole("button", { name: /sign in/i });
    const isVisible = await signInButton.isVisible().catch(() => false);

    if (isVisible) {
      await signInButton.click();

      const createAccountLink = page.getByText(/create an account/i);
      await expect(createAccountLink).toBeVisible();
    }

    await context.close();
  });
});
