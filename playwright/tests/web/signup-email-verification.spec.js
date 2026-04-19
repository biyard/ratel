import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, waitPopup } from "../utils";

/**
 * Signup Email Verification Flow
 *
 * This scenario tests the complete email signup flow end-to-end, verifying
 * that the email verification code sending works correctly after the
 * refactoring from direct SES calls to event-driven Notification documents
 * (issue #1305).
 *
 * The flow exercises:
 *   1. Navigate to the home page
 *   2. Open the Sign In modal
 *   3. Switch to the signup form ("Create an account")
 *   4. Enter email and click "Send" (triggers send_code_handler which now
 *      creates a Notification document instead of calling SES directly)
 *   5. Enter the bypass verification code "000000" and click "Verify"
 *   6. Fill password, confirm password, display name, username
 *   7. Agree to Terms of Service
 *   8. Submit signup ("Finished Sign-up")
 *   9. Verify the modal closes and the user is logged in
 *
 * NOTE: Requires backend built with `--features bypass` so that email
 *       verification accepts the hardcoded code "000000".
 */

test.describe.serial("Signup with email verification (event-driven notification)", () => {
  let context;
  let page;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const testUser = {
    email: `e2e_signup_notif_${uniqueId}@biyard.co`,
    username: `su${uniqueId}`,
    displayName: `Signup Test ${uniqueId}`,
    password: "Test!234",
  };

  test.beforeAll(async ({ browser }) => {
    // Create a fresh browser context with no saved auth state
    context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    page = await context.newPage();
  });

  test.afterAll(async () => {
    await context.close();
  });

  // --- 1. Load the home page ---

  test("should load the home page", async () => {
    await goto(page, "/");
    await expect(page).toHaveURL("/");
  });

  // --- 2. Open Sign In modal and switch to signup ---

  test("should open Sign In modal and navigate to signup form", async () => {
    await click(page, { label: "Sign In" });
    await waitPopup(page, { visible: true });

    // Switch to signup form via "Create an account" link
    await click(page, { text: "Create an account" });

    // Verify the signup form is visible by checking for the email placeholder
    await getLocator(page, { placeholder: "Enter your email address" });
  });

  // --- 3. Send verification code ---
  //     This step exercises the refactored send_code_handler which now creates
  //     a Notification document in DynamoDB instead of calling SES directly.

  test("should send verification code to email", async () => {
    await fill(
      page,
      { placeholder: "Enter your email address" },
      testUser.email,
    );

    // Click "Send" to trigger verification code sending
    await click(page, { text: "Send" });

    // After sending, the verification code input should appear
    // (it is conditionally visible once sent_code is true)
    await getLocator(page, { placeholder: "Enter the verification code" });
  });

  // --- 4. Verify the code ---
  //     With the bypass feature enabled, "000000" is accepted as valid.

  test("should verify the email with bypass code", async () => {
    await fill(
      page,
      { placeholder: "Enter the verification code" },
      "000000",
    );

    await click(page, { text: "Verify" });

    // Wait for the "Send" button to disappear, indicating the email is verified.
    // The "Send" button is only shown when is_valid_email is false.
    await expect(page.getByText("Send", { exact: true })).toBeHidden({
      timeout: 10000,
    });
  });

  // --- 5. Fill signup details ---

  test("should fill password, display name, and username", async () => {
    // Password
    await fill(
      page,
      { placeholder: "Enter your password" },
      testUser.password,
    );

    // Confirm password
    await fill(
      page,
      { placeholder: "Re-enter your password" },
      testUser.password,
    );

    // Display name
    await fill(
      page,
      { placeholder: "Enter your display name" },
      testUser.displayName,
    );

    // Username
    await fill(
      page,
      { placeholder: "Enter your user name" },
      testUser.username,
    );

    // Verify all fields are filled
    const passwordInput = page.getByPlaceholder("Enter your password", {
      exact: true,
    });
    await expect(passwordInput).toHaveValue(testUser.password);
  });

  // --- 6. Agree to Terms of Service and submit ---

  test("should agree to ToS and complete signup", async () => {
    // Click the required Terms of Service checkbox
    await click(page, {
      label: "[Required] I have read and accept the Terms of Service.",
    });

    // Click "Finished Sign-up" to submit the signup form
    await click(page, { text: "Finished Sign-up" });

    // The signup modal should close after successful registration
    await waitPopup(page, { visible: false });
  });

  // --- 7. Verify the user is logged in ---

  test("should be logged in after signup", async () => {
    // After signup, navigate to home to verify authenticated state.
    // After the home-ui renewal the old "User Profile" sidebar is gone; on the
    // arena top bar the logged-in-only Drafts button appears instead (unlogged
    // users see the Sign In button in its place).
    await goto(page, "/");

    await getLocator(page, { testId: "home-btn-drafts" });
    await expect(page.getByTestId("home-btn-signin")).toBeHidden();
  });
});
