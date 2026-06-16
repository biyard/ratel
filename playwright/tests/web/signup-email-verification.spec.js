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
 *   6. Fill display name and username (passwordless — no password step)
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

  // --- 2. Open Sign In modal (unified email-code flow) ---

  test("should open Sign In modal and show the email form", async () => {
    await click(page, { label: "Sign In" });
    await waitPopup(page, { visible: true });

    // Unified passwordless flow: the modal opens straight to the email
    // field. There is no separate "Create an account" step anymore — a
    // verified code for an unknown account routes into signup automatically.
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

    // The single "Continue" button sends the code on the first press;
    // the code input is revealed once send_code resolves.
    await click(page, { testId: "continue-button" });

    await getLocator(page, { placeholder: "Enter the verification code" });
  });

  // --- 4. Verify the code and route into signup ---
  //     With the bypass feature enabled, "000000" is accepted as valid.

  test("should verify the code and route to the signup form", async () => {
    await fill(
      page,
      { placeholder: "Enter the verification code" },
      "000000",
    );

    // The second "Continue" press verifies the code and attempts login;
    // since this email has no account, the modal routes into signup with
    // the email + code carried over (already verified). The signup form's
    // display-name field appearing confirms the transition.
    await click(page, { testId: "continue-button" });

    await getLocator(page, { placeholder: "Enter your display name" });
  });

  // --- 5. Fill signup details ---

  test("should fill display name and username", async () => {
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

    // Verify the username field is filled (passwordless — no password input)
    const usernameInput = page.getByPlaceholder("Enter your user name", {
      exact: true,
    });
    await expect(usernameInput).toHaveValue(testUser.username);
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
    // After the topbar regroup (Compose/Drafts/AI-assist collapsed into a
    // Create ▾ dropdown) there's no longer a top-level Drafts button. The
    // most reliable logged-in-only element on the arena topbar is the
    // Teams dropdown trigger (`home-btn-teams` is rendered only when
    // `has_user` is true), and unlogged users see `home-btn-signin` in
    // place of the account dropdown.
    await goto(page, "/");

    await getLocator(page, { testId: "home-btn-teams" });
    await expect(page.getByTestId("home-btn-signin")).toBeHidden();
  });
});
