import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, waitPopup } from "../utils";

/**
 * Admin Sidebar Menu — Issue #1333
 *
 * Verifies that SystemAdmin users can see and navigate to the Admin panel
 * from the sidebar menu.
 *
 * User:
 *   - admin@ratel.foundation (UserType::Admin, user_type=98)
 *
 * Flow:
 *   1. Login as admin user and save storage state
 *   2. Verify the "Admin" menu item is visible in the sidebar
 *   3. Click the Admin link and verify navigation to /admin
 *   4. Verify the admin page renders with "Reward Management" content
 *
 * NOTE: Requires backend built with `--features bypass`.
 */

test.describe.serial("Admin sidebar menu for SystemAdmin users (#1333)", () => {
  const adminEmail = "admin@ratel.foundation";
  const adminPassword = "admin!234";

  // --- 1. Login as admin user ---

  test("should login as admin user", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");
      await click(page, { label: "Sign In" });
      await fill(
        page,
        { placeholder: "Enter your email address" },
        adminEmail,
      );
      await click(page, { text: "Continue" });
      await fill(
        page,
        { placeholder: "Enter your password" },
        adminPassword,
      );
      await click(page, { text: "Continue" });
      await waitPopup(page, { visible: false });

      await context.storageState({ path: "admin-system.json" });
    } finally {
      await context.close();
    }
  });

  // --- 2. Verify Admin menu item is visible ---

  test("should show Admin menu item in sidebar", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: "admin-system.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");
      await getLocator(page, { text: "Admin" });
    } finally {
      await context.close();
    }
  });

  // --- 3. Navigate to admin page ---

  test("should navigate to /admin when clicking Admin menu", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      storageState: "admin-system.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");
      await click(page, { text: "Admin" });
      await expect(page).toHaveURL(/\/admin/);
    } finally {
      await context.close();
    }
  });

  // --- 4. Verify admin page content ---

  test("should render the admin Reward Management page", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      storageState: "admin-system.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/admin");
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null,
      );
      await getLocator(page, { text: "Reward Management" });
    } finally {
      await context.close();
    }
  });
});
