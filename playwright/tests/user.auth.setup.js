import { test } from "@playwright/test";
import { waitPopup, click, fill, goto, waitForHydrated } from "./utils";

test("create storage state", async ({ page }) => {
  const email = `hi+user1@biyard.co`;
  const password = "admin!234";

  await goto(page, "/");

  // Wait for the Sign In button to be hydrated — hot-patch toasts from
  // Dioxus dev server can trigger re-renders that temporarily detach
  // event handlers, causing the click to be silently dropped.
  // await waitForHydrated(page, "home-btn-signin");
  await click(page, { testId: "home-btn-signin" });
  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, password);
  await click(page, { text: "Continue" });

  await waitPopup(page, { visible: false });

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: "user.json" });

  console.log("✅ Global authenticated user setup completed");
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: user.json`);
});
