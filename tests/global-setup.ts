import { chromium, expect, FullConfig } from "@playwright/test";
import { CONFIGS } from "./config";
import * as fs from "fs";
import * as path from "path";

async function globalSetup(config: FullConfig) {
  console.log("🚀 Starting global setup for authenticated tests...");

  // Ensure test-results directory exists
  const authDir = "test-results/.auth";
  if (!fs.existsSync(authDir)) {
    fs.mkdirSync(authDir, { recursive: true });
  }

  // Launch browser
  const browser = await chromium.launch();
  const page = await browser.newPage();

  try {
    const id = CONFIGS.PLAYWRIGHT.ID;
    const email = `playwright+${id}@ratel.foundation`;
    const password = "password1234!@#$";
    const displayName = `Playwright User ${id}`;
    const userName = `playwrightuser-${id}`;
    console.log(`🆕 Creating new user: ${email} / ${password}`);

    // Save test user information
    const testUser = {
      email,
      password,
      displayName,
      username: userName,
    };
    fs.writeFileSync(
      "test-results/.auth/test-user.json",
      JSON.stringify(testUser, null, 2),
    );

    await page.goto(CONFIGS.PLAYWRIGHT.BASE_URL!);
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByText("Create an account").click();

    await page.getByPlaceholder(/email/i).fill(email);
    await page.getByText("Send").click();

    await page.getByText("Verify").click();

    await page.getByPlaceholder(/password/i).fill(password);
    await page.getByPlaceholder(/display name/i).fill(displayName);
    await page.getByPlaceholder(/user name/i).fill(userName);

    // Accept terms by clicking the label (checkbox is hidden)
    const tosCheckbox = page.locator('label[for="agree_checkbox"]');
    await tosCheckbox.click();
    await page.getByRole("button", { name: /finished sign-up/i }).click();
    await expect(page.getByText(/start/i)).toBeVisible();

    // Save Playwright storage state for authenticated tests
    await page.context().storageState({ path: "test-results/.auth/user.json" });

    console.log("✅ Global authenticated user setup completed");
    console.log(`📄 Test user saved: ${email}`);
    console.log(`🔐 Storage state saved to: test-results/.auth/user.json`);
  } catch (error) {
    console.error("❌ Global setup failed:", error);
    throw error;
  } finally {
    await browser.close();
  }
}

export default globalSetup;
