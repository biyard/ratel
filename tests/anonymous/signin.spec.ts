import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("User Sign-in Flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("[SI-001] should reject invalid email format", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    const emailInput = page.getByRole("textbox", { name: /email/i });
    await emailInput.fill("jake");
    await page.getByRole("button", { name: /^continue$/i }).click();

    await expect(page.getByText(/please enter a valid email/i)).toBeVisible();
  });

  test("[SI-002] should reject unregistered email", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    const emailInput = page.getByRole("textbox", { name: /email/i });
    await emailInput.fill("jake@gmail.com");
    await page.getByRole("button", { name: /^continue$/i }).click();

    await expect(page.getByText(/this email is not registered/i)).toBeVisible();
  });

  test("[SI-003] should accept registered email and require password", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    const emailInput = page.getByRole("textbox", { name: /email/i });
    await emailInput.fill(CONFIGS.SECRETS.testemail as string);
    await page.getByRole("button", { name: /^continue$/i }).click();

    await expect(page.getByText(/password/i)).toBeVisible();
  });

  test("[SI-004] should login with valid credentials", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    const emailInput = page.getByRole("textbox", { name: /email/i });
    await emailInput.fill(CONFIGS.SECRETS.testemail as string);
    await page.getByRole("button", { name: /^continue$/i }).click();

    const passwordInput = page.getByRole("textbox", { name: /password/i });
    await passwordInput.fill(CONFIGS.SECRETS.password as string);

    const signInButton = page.getByRole("button", { name: /^sign in$/i });
    await expect(signInButton).toBeEnabled();
    await signInButton.click();

    await expect(page.getByText(/'/)).toBeVisible();
  });
});
