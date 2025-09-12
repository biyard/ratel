import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Create Post Without Space", () => {
  test("[COP-001] should login and create a simple post without a space", async ({ page }) => {
    await page.goto("/");
    await page.screenshot({ path: "test-results/COP-001/landing.png" });

    // Sign in
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByRole("textbox", { name: /email/i }).fill(CONFIGS.SECRETS.testemail as string);
    await page.getByRole("button", { name: /^continue$/i }).click();
    await page.getByRole("textbox", { name: /password/i }).fill(CONFIGS.SECRETS.password as string);
    await page.getByRole("button", { name: /^sign in$/i }).click();
    await page.screenshot({ path: "test-results/COP-001/logged-in.png" });

    // Create Post
    await page.getByText("Create Post").first().click();
    await page.getByRole("textbox", { name: "Write a title..." }).fill("Playwright Post");
    await page.getByRole("textbox").nth(1).fill("This is a post without a space.");
    await page.screenshot({ path: "test-results/COP-001/post-filled.png" });

    // Save draft or publish (depending on flow)
    await page.getByRole("button", { name: "Create" }).click();
    await expect(page.getByText("Playwright Post")).toBeVisible();
    await page.screenshot({ path: "test-results/COP-001/post-created.png" });
  });
});
