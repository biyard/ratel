import { test, expect } from "@playwright/test";
import { click, fill, goto, waitPopup } from "../utils";

/**
 * Admin Sidebar Menu — Issue #1333
 *
 * Verifies that SystemAdmin users can see and navigate to the Admin panel
 * from the sidebar menu.
 *
 * User:
 *   - admin@ratel.foundation (UserType::Admin, user_type=98)
 *     Pre-seeded by LocalStack init script.
 *
 * Flow:
 *   1. Log in as the pre-seeded admin user
 *   2. Verify the "Admin" menu item is visible in the sidebar
 *   3. Click the Admin link and verify navigation to /admin
 *   4. Verify the admin page renders with "Reward Management" content
 *
 * NOTE: Requires backend built with `--features bypass`.
 */

test.describe.serial("Admin sidebar menu for SystemAdmin users (#1333)", () => {
  const adminEmail = "admin@ratel.foundation";
  const adminPassword = "admin!234";

  /** Login as admin in a fresh context and return { context, page }. */
  async function loginAsAdmin(browser) {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

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

    return { context, page };
  }

  // --- 1. Log in and verify Admin menu item is visible ---

  test("should show Admin menu item in sidebar after login", async ({
    browser,
  }) => {
    const { context, page } = await loginAsAdmin(browser);

    try {
      // User context is set synchronously during login;
      // the Admin menu should appear once the sidebar re-renders.
      await expect(
        page.getByText("Admin", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
    } finally {
      await context.close();
    }
  });

  // --- 2. Navigate to admin page via sidebar ---

  test("should navigate to /admin when clicking Admin menu", async ({
    browser,
  }) => {
    const { context, page } = await loginAsAdmin(browser);

    try {
      await expect(
        page.getByText("Admin", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
      await click(page, { text: "Admin" });
      await expect(page).toHaveURL(/\/admin/);
    } finally {
      await context.close();
    }
  });

  // --- 3. Verify admin page content ---

  test("should render the admin Reward Management page", async ({
    browser,
  }) => {
    const { context, page } = await loginAsAdmin(browser);

    try {
      await expect(
        page.getByText("Admin", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
      await click(page, { text: "Admin" });
      await expect(page).toHaveURL(/\/admin/);
      await expect(
        page.getByText("Reward Management", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
    } finally {
      await context.close();
    }
  });
});
