import { test, expect } from "@playwright/test";
import { CONFIGS } from "../../config";
import { wrap } from "../../utils";

test.describe("Auth Flow - Login", () => {
  test("should show errors and login successfully", async ({ page }, testInfo) => {
    const p = wrap(page, testInfo.project.name, "auth/login");
    await p.goto("/", { waitUntil: "load", timeout: CONFIGS.PAGE_WAIT_TIME });

    await p.getByRole("button", { name: "Sign In" }).click();

    await p.getByRole("button", { name: "Continue", exact: true }).click();
    await expect(p.locator("text=Please enter a valid email")).toBeVisible();

    await p.getByRole("textbox", { name: "Enter your email address" }).fill(CONFIGS.credentials.newUserEmail);
    await p.getByRole("button", { name: "Continue", exact: true }).click();
    await expect(p.locator("text=This email is not registered.")).toBeVisible();

    await p.getByRole("textbox", { name: "Enter your email address" }).fill(CONFIGS.credentials.email);
    await p.getByRole("button", { name: "Continue", exact: true }).click();

    await p.getByRole("textbox", { name: "Enter your password" }).fill("weak");
    await expect(p.locator("text=Password must contain letters")).toBeVisible();

    await p.getByRole("textbox", { name: "Enter your password" }).fill(CONFIGS.credentials.password);
    await p.getByRole("button", { name: "Sign in", exact: true }).click();

    await expect(p).toHaveURL(/spaces|drafts/);
  });
});
