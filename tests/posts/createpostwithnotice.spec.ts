import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Create Post with Notice Space", () => {
  test("[CPWN-001] should login, create a post, and attach a notice space", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByRole("textbox", { name: /email/i }).fill(CONFIGS.SECRETS.testemail as string);
    await page.getByRole("button", { name: /^continue$/i }).click();
    await page.getByRole("textbox", { name: /password/i }).fill(CONFIGS.SECRETS.password as string);
    await page.getByRole("button", { name: /^sign in$/i }).click();
    await page.screenshot({ path: "test-results/CPWN-001/notice-login.png" });

    await page.getByText("Create Post").first().click();
    await page.getByRole("textbox", { name: "Write a title..." }).fill("Notice: Downtime Scheduled");
    await page.getByRole("textbox").nth(1).fill("The system will be down for maintenance on Friday.");
    await page.screenshot({ path: "test-results/CPWN-001/notice-filled.png" });

    await page.getByRole("button", { name: "Create a Space" }).click();
    await page.getByText("Notice").click();
    await page.getByRole("button", { name: "Next" }).click();

    await page.getByRole("checkbox", { name: "Activate Booster" }).check();
    await page.getByRole("button", { name: "Booster x" }).click();
    await page.getByText("Booster x 100").click();
    await page.screenshot({ path: "test-results/CPWN-001/notice-config.png" });

    await page.getByRole("button", { name: "Create", exact: true }).click();
    await expect(page.getByText("Notice")).toBeVisible();
    await page.screenshot({ path: "test-results/CPWN-001/notice-created.png" });
  });
});
