import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("User Sign-in Flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  // test("[SI-001] should reject invalid email format", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();
  //   await page.screenshot({ path: "test-results/SI-001/01-signin-popup.png" });

  //   const emailInput = page.getByRole("textbox", { name: /email/i });
  //   await emailInput.fill("jake");
  //   await page.screenshot({
  //     path: "test-results/SI-001/02-invalid-email-filled.png",
  //   });

  //   await page.getByRole("button", { name: /^continue$/i }).click();
  //   await page.screenshot({
  //     path: "test-results/SI-001/03-after-continue.png",
  //   });

  //   await expect(page.getByText(/please enter a valid email/i)).toBeVisible();
  // });

  // test("[SI-002] should reject unregistered email", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();
  //   await page.screenshot({ path: "test-results/SI-002/01-signin-popup.png" });

  //   const emailInput = page.getByRole("textbox", { name: /email/i });
  //   await emailInput.fill("jake@gmail.com");
  //   await page.screenshot({
  //     path: "test-results/SI-002/02-unregistered-email.png",
  //   });

  //   await page.getByRole("button", { name: /^continue$/i }).click();
  //   await page.screenshot({
  //     path: "test-results/SI-002/03-after-continue.png",
  //   });

  //   await expect(page.getByText(/this email is not registered/i)).toBeVisible();
  // });

  // test("[SI-003] should accept registered email and require password", async ({
  //   page,
  // }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();
  //   await page.screenshot({ path: "test-results/SI-003/01-signin-popup.png" });

  //   const emailInput = page.getByRole("textbox", { name: /email/i });
  //   await emailInput.fill(CONFIGS.SECRETS.testemail as string);
  //   await page.screenshot({ path: "test-results/SI-003/02-valid-email-filled.png" });

  //   await page.getByRole("button", { name: /^continue$/i }).click();
  //   await page.screenshot({ path: "test-results/SI-003/03-password-required.png" });

  //   await expect(page.getByText(/password/i)).toBeVisible();
  // });

  // test("[SI-004] should login with valid credentials", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();
  //   await page.screenshot({ path: "test-results/SI-004/01-signin-popup.png" });

  //   const emailInput = page.getByRole("textbox", { name: /email/i });
  //   await emailInput.fill(CONFIGS.SECRETS.testemail as string);
  //   await page.screenshot({ path: "test-results/SI-004/02-email-filled.png" });

  //   await page.getByRole("button", { name: /^continue$/i }).click();
  //   await page.screenshot({ path: "test-results/SI-004/03-password-field.png" });

  //   const passwordInput = page.getByRole("textbox", { name: /password/i });
  //   await passwordInput.fill(CONFIGS.SECRETS.password as string);
  //   await page.screenshot({ path: "test-results/SI-004/04-password-filled.png" });

  //   const signInButton = page.getByRole("button", { name: /^sign in$/i });
  //   await expect(signInButton).toBeEnabled();
  //   await signInButton.click();
  //   await page.screenshot({ path: "test-results/SI-004/05-after-login.png" });

  //   await expect(page.getByText(/|/)).toBeVisible();
  // });
});
