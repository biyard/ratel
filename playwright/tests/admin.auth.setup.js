import { test } from "@playwright/test";
import { waitPopup, click, fill, goto } from "./utils";

test("create storage state", async ({ page }) => {
  const email = `admin@ratel.foundation`;
  const password = "admin!234";

  await goto(page, "/");

  await click(page, { testId: "home-btn-signin" });
  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, password);
  await click(page, { text: "Continue" });

  await waitPopup(page, { visible: false });

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: "admin.json" });

  console.log("✅ Global authenticated user setup completed");
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: admin.json`);
});
