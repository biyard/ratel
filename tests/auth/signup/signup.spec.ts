import { test, expect } from "@playwright/test";
import { CONFIGS } from "../../config";
import { wrap } from "../../utils";

test.describe("Auth Flow - Create Account", () => {
  test("should show validation and allow account creation steps", async ({ page }, testInfo) => {
    const p = wrap(page, testInfo.project.name, "/");
    await p.goto("/", { waitUntil: "load", timeout: CONFIGS.PAGE_WAIT_TIME });

    await p.getByRole("button", { name: "Sign In" }).click();
    await p.getByRole("button", { name: "Create an account" }).click();

    await p.getByRole("button", { name: "Send" }).click();

    await expect(p.locator("text=Please enter a valid email").first()).toBeVisible();

    await p.locator('input[name="username"]').fill(CONFIGS.credentials.newUserEmail);
    await p.getByRole("button", { name: "Send" }).click();

    await p.getByRole("textbox", { name: "Verification code" }).fill(CONFIGS.credentials.verificationCode);
    await p.getByRole("button", { name: "Verify" }).click();

    await p.getByRole("textbox", { name: "Display Name" }).fill(CONFIGS.credentials.displayName);
    await p.getByRole("textbox", { name: "User Name" }).fill(CONFIGS.credentials.username);

    await p.getByText("I have read and accept the Terms of Service").click();

    await p.getByRole('button', { name: 'Finished Sign-up' }).click();
  });
});
