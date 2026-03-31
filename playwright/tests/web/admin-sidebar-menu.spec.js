import { test, expect } from "@playwright/test";
import { click, goto, getLocator } from "../utils";

/**
 * Admin Sidebar Menu — Issue #1333
 *
 * Verifies that SystemAdmin users can see and navigate to the Admin panel
 * from the sidebar menu.
 *
 * User:
 *   - hi+user1@biyard.co (pre-authenticated via user.json, SystemAdmin)
 *
 * Flow:
 *   1. Load home page (authenticated)
 *   2. Verify the "Admin" menu item is visible in the sidebar
 *   3. Click the Admin link and verify navigation to /admin
 *   4. Verify the admin page renders with "Reward Management" content
 *
 * NOTE: Requires backend built with `--features bypass`.
 */

test.describe.serial("Admin sidebar menu for SystemAdmin users (#1333)", () => {
  // --- 1. Load the home page ---

  test("should load the home page as authenticated user", async ({ page }) => {
    await goto(page, "/");
    await expect(page).toHaveURL("/");

    // Verify user is logged in — profile button should be visible
    await getLocator(page, { label: "User Profile" });
  });

  // --- 2. Verify Admin menu item is visible ---

  test("should show Admin menu item in sidebar", async ({ page }) => {
    await goto(page, "/");

    // The Admin menu link should be visible in the sidebar for admin users
    await getLocator(page, { text: "Admin" });
  });

  // --- 3. Navigate to admin page ---

  test("should navigate to /admin when clicking Admin menu", async ({
    page,
  }) => {
    await goto(page, "/");

    await click(page, { text: "Admin" });

    await expect(page).toHaveURL(/\/admin/);
  });

  // --- 4. Verify admin page content ---

  test("should render the admin Reward Management page", async ({ page }) => {
    await goto(page, "/admin");

    // Wait for Dioxus hydration
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null,
    );

    // The admin page should display the Reward Management section
    await getLocator(page, { text: "Reward Management" });
  });
});
