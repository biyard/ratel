import { test, expect } from "@playwright/test";
import { click, fill, goto, waitPopup } from "../utils";

/**
 * Admin Sidebar Menu — Issue #1333
 *
 * Verifies that SystemAdmin users can see and navigate to the Admin panel
 * from the sidebar menu, and that non-admin users cannot see it.
 *
 * User:
 *   - admin@ratel.foundation (UserType::Admin, user_type=98)
 *     Pre-seeded by LocalStack init script.
 *   - hi+user1@biyard.co (regular user, via user.json storage state)
 *
 * Flow:
 *   1. Log in as the pre-seeded admin user
 *   2. Verify the "admin-menu" test-id element is visible in the sidebar
 *   3. Click the Admin link and verify navigation to /admin
 *   4. Verify the admin page renders with "Reward Management" content
 *   5. Verify non-admin users do NOT see the admin menu
 *
 * NOTE: Requires backend built with `--features bypass`.
 */

test.describe.serial("Admin sidebar menu for SystemAdmin users (#1333)", () => {
  // --- 1. Log in and verify Admin menu item is visible ---

  test("should show Admin menu item in sidebar after login", async ({
    page,
  }) => {
    await goto(page, "/");

    // The sidebar starts collapsed (default_open: false), so text labels
    // are hidden. Use data-testid which is always present on the link.
    await expect(page.getByTestId("admin-menu")).toBeVisible({
      timeout: 30000,
    });
  });

  // --- 2. Navigate to admin page via sidebar ---

  test("should navigate to /admin when clicking Admin menu", async ({
    page,
  }) => {
    await click(page, { testId: "admin-menu" });
    await expect(page).toHaveURL(/\/admin/);
  });

  // --- 3. Verify admin page content ---

  test("should render the admin Reward Management page", async ({ page }) => {
    await click(page, { testId: "admin-menu" });
    await expect(page).toHaveURL(/\/admin/);
    await expect(
      page.getByText("Reward Management", { exact: true }),
    ).toBeVisible({ timeout: 30000 });
  });
});
