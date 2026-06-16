import { test } from "@playwright/test";
import { waitPopup, click, fill, goto } from "./utils";

test("create storage state", async ({ page }) => {
  const email = `hi+user1@biyard.co`;
  // Passwordless email-code login. The backend is built with --features
  // bypass, so "000000" is always accepted as the verification code.
  const code = "000000";

  await goto(page, "/");

  await click(page, { testId: "home-btn-signin" });
  await fill(page, { placeholder: "Enter your email address" }, email);
  // Step 1: send the verification code (reveals the code field).
  await click(page, { testId: "continue-button" });
  // Step 2: enter the code and verify → existing user is logged in.
  await fill(page, { testId: "code-input" }, code);
  await click(page, { testId: "continue-button" });

  await waitPopup(page, { visible: false });

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: "user.json" });

  console.log("✅ Global authenticated user setup completed");
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: user.json`);
});
