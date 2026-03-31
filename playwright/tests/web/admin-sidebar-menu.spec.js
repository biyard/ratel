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
 *
 * Flow:
 *   1. Login as admin user, promote to admin via DynamoDB, save storage state
 *   2. Verify the "Admin" menu item is visible in the sidebar
 *   3. Click the Admin link and verify navigation to /admin
 *   4. Verify the admin page renders with "Reward Management" content
 *
 * NOTE: Requires backend built with `--features bypass`.
 */

/**
 * Promote a user to SystemAdmin (user_type=98) by directly updating
 * the DynamoDB record in LocalStack. This ensures the user has admin
 * privileges regardless of how the login flow creates/updates users.
 */
async function promoteToAdmin(email) {
  const endpoint =
    process.env.DYNAMO_ENDPOINT || "http://localhost:4566";
  const tableName = "ratel-local-main";

  // Query GSI3 to find user by email
  const queryResp = await fetch(endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-amz-json-1.0",
      "X-Amz-Target": "DynamoDB_20120810.Query",
    },
    body: JSON.stringify({
      TableName: tableName,
      IndexName: "gsi3",
      KeyConditionExpression: "gsi3_pk = :email",
      ExpressionAttributeValues: {
        ":email": { S: `EMAIL#${email}` },
      },
    }),
  });

  const queryResult = await queryResp.json();
  if (!queryResult.Items || queryResult.Items.length === 0) {
    throw new Error(`User with email ${email} not found in DynamoDB`);
  }

  const user = queryResult.Items[0];
  const pk = user.pk.S;
  const sk = user.sk.S;

  // Update user_type to Admin (98) and GSI4 partition key for consistency
  await fetch(endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-amz-json-1.0",
      "X-Amz-Target": "DynamoDB_20120810.UpdateItem",
    },
    body: JSON.stringify({
      TableName: tableName,
      Key: {
        pk: { S: pk },
        sk: { S: sk },
      },
      UpdateExpression: "SET user_type = :ut, gsi4_pk = :gsi4pk",
      ExpressionAttributeValues: {
        ":ut": { N: "98" },
        ":gsi4pk": { S: "USER_TYPE#98" },
      },
    }),
  });
}

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

      // Ensure the user is promoted to admin in DynamoDB
      await promoteToAdmin(adminEmail);

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
      // User context loads asynchronously after WASM hydration;
      // use a longer timeout so the admin menu has time to appear.
      await expect(
        page.getByText("Admin", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
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
      // Wait for user context to load before clicking
      await expect(
        page.getByText("Admin", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
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
      await expect(
        page.getByText("Reward Management", { exact: true }),
      ).toBeVisible({ timeout: 30000 });
    } finally {
      await context.close();
    }
  });
});
