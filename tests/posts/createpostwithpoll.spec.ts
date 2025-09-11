import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Create Post with Poll Space", () => {
  test("should login, create a post, and attach a poll space", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByRole("textbox", { name: /email/i }).fill(CONFIGS.SECRETS.testemail as string);
    await page.getByRole("button", { name: /^continue$/i }).click();
    await page.getByRole("textbox", { name: /password/i }).fill(CONFIGS.SECRETS.password as string);
    await page.getByRole("button", { name: /^sign in$/i }).click();
    await page.screenshot({ path: "test-results/CPWP-001/poll-login.png" });

    // Create Post
    await page.getByText("Create Post").first().click();
    await page.getByRole("textbox", { name: "Write a title..." }).fill("Poll: Favorite Framework?");
    await page.getByRole("textbox").nth(1).fill("Vote for your favorite frontend framework!");
    await page.screenshot({ path: "test-results/CPWP-001/poll-filled.png" });

    // Create Poll Space
    await page.getByRole("button", { name: "Create a Space" }).click();
    await page.getByText("Poll").click();
    await page.getByRole("button", { name: "Next" }).click();

    // Pick Poll details (dates, booster, etc.)
    await page.getByRole("checkbox", { name: "Activate Booster" }).check();
    await page.getByRole("button", { name: "Booster x" }).click();
    await page.getByText("Booster x 10").click();
    await page.screenshot({ path: "test-results/CPWP-001/poll-config.png" });

    // Create
    await page.getByRole("button", { name: "Create", exact: true }).click();
    await expect(page.getByText("Poll Close")).toBeVisible();
    await page.screenshot({ path: "test-results/CPWP-001/poll-created.png" });
  });
});
