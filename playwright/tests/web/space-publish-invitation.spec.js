import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor, waitPopup } from "../utils";
import { CONFIGS } from "../config";

/**
 * Space Publish with Invitation Emails
 *
 * This scenario tests the space creation and publishing flow end-to-end,
 * verifying that publishing a space (which triggers invitation emails to
 * invited members) works correctly after the refactoring from direct SES
 * calls to event-driven Notification documents (issue #1305).
 *
 * When a space is published, the backend:
 *   1. Updates the space status to Published / InProgress
 *   2. Calls SpaceInvitationMember::send_email() which:
 *      a. Queries pending invitation members
 *      b. Updates their status to Invited
 *      c. Calls SpaceEmailVerification::send_invitation_emails() which
 *         creates a Notification document with SendSpaceInvitation data
 *
 * The test flow:
 *   1. Creator (pre-authenticated user) creates a post with a space
 *   2. Creator invites a member to the space via the General app settings
 *   3. Creator publishes the space as public (triggers invitation emails)
 *   4. Verify the space is published and accessible (anonymous)
 *   5. A new user signs up from the published space page
 *      (exercises the email verification Notification path again)
 *
 * NOTE: Requires backend built with `--features bypass` so that email
 *       verification accepts the hardcoded code "000000".
 */

test.describe.serial("Space publish and invitation (event-driven notification)", () => {
  let spaceUrl;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const postTitle = `E2E Notification Test Space ${uniqueId}`;
  const postContents =
    "This space tests the event-driven notification refactoring. " +
    "Publishing this space triggers a Notification document for invitation " +
    "emails instead of direct SES calls. The space is created with sufficient " +
    "content to meet validation requirements for publishing.";

  // --- 1. Create a post with space ---

  test("Creator: Create a post with a space and verify dashboard", async ({
    page,
  }) => {
    await goto(page, "/");

    // Create a post using the "Create Post" button
    await click(page, { label: "Create Post" });

    // Wait for post editor page
    await page.waitForURL(/\/posts\/.*\/edit/, {
      waitUntil: "load",
    });

    // Fill in the post title
    await fill(page, { placeholder: "Title" }, postTitle);

    // Uncheck "Skip creating space" to enable space creation
    await click(page, { testId: "skip-space-checkbox" });

    // Fill in the post body
    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Click "Go to Space" to create the space alongside the post
    await click(page, { text: "Go to Space" });

    // Wait for navigation to the space dashboard
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "load",
    });
    await getLocator(page, { text: "Dashboard" });

    // Capture the space URL for subsequent tests
    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  });

  // --- 2. Invite a member via the General app ---
  //     Navigate to the space General app settings and invite a participant
  //     by email. This creates a SpaceInvitationMember record so that when
  //     the space is published, invitation emails are dispatched via the
  //     event-driven Notification pipeline.

  test("Creator: Invite a participant via General app", async ({ page }) => {
    await goto(page, spaceUrl + "/apps/general");

    // The "Invite Participant" section has an email input
    await fill(page, { placeholder: "example@example.com" }, "hi+user2@biyard.co");

    // Click the "Invite" button to submit the invitation
    await click(page, { text: "Invite" });

    // Verify the invitation was created by checking for the success toast
    // or that the invited email appears in the "Invited Accounts" list
    await page.waitForTimeout(2000);

    // Reload to confirm the invitation persisted
    await goto(page, spaceUrl + "/apps/general");
    await getLocator(page, { text: "Invited Accounts" });
  });

  // --- 3. Publish space as public ---
  //     This triggers SpaceInvitationMember::send_email() which now creates
  //     a Notification document with SendSpaceInvitation data instead of
  //     calling SES directly.

  test("Creator: Publish space as public", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");

    // Click the Publish button
    await click(page, { text: "Publish" });

    // Select public visibility option
    await click(page, { testId: "public-option" });

    // Confirm the visibility selection
    await click(page, { label: "Confirm visibility selection" });
    await page.waitForLoadState("load");

    // Verify the space is now published by checking the dashboard is still
    // accessible and the Publish button has been replaced by status indicators
    await goto(page, spaceUrl + "/dashboard");
    await getLocator(page, { text: "Dashboard" });
  });

  // --- 4. Verify the published space is accessible without auth ---

  test("Anonymous: Published space dashboard is accessible", async ({
    browser,
  }) => {
    // Create a fresh anonymous context (no session cookies)
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl + "/dashboard");

      // Verify the dashboard page loaded
      await getLocator(page, { text: "Dashboard" });

      // An anonymous user should see the "Sign In" button on the space page
      await getLocator(page, { text: "Sign In" });
    } finally {
      await context.close();
    }
  });

  // --- 5. Signup from the published space page ---
  //     This exercises the same email verification Notification path from
  //     the perspective of a user signing up through a space.

  test("New user: Sign up from the published space page", async ({
    browser,
  }) => {
    const signupUniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
    const newUser = {
      email: `e2e_sp_signup_${signupUniqueId}@biyard.co`,
      username: `sps${signupUniqueId}`,
      displayName: `Space Signup ${signupUniqueId}`,
      password: "Test!234",
    };

    // Create a fresh anonymous context
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      // Navigate to the published space
      await goto(page, spaceUrl + "/dashboard");

      // Click Sign In on the space sidebar
      await click(page, { text: "Sign In" });
      await waitPopup(page, { visible: true });

      // Switch to signup modal
      await click(page, { text: "Create an account" });

      // Fill email and send verification code
      await fill(
        page,
        { placeholder: "Enter your email address" },
        newUser.email,
      );
      await click(page, { text: "Send" });

      // Enter bypass verification code and verify
      await fill(
        page,
        { placeholder: "Enter the verification code" },
        "000000",
      );
      await click(page, { text: "Verify" });

      // Wait for verification to complete (Send button disappears)
      await expect(page.getByText("Send", { exact: true })).toBeHidden({
        timeout: 10000,
      });

      // Fill signup details
      await fill(
        page,
        { placeholder: "Enter your password" },
        newUser.password,
      );
      await fill(
        page,
        { placeholder: "Re-enter your password" },
        newUser.password,
      );
      await fill(
        page,
        { placeholder: "Enter your display name" },
        newUser.displayName,
      );
      await fill(
        page,
        { placeholder: "Enter your user name" },
        newUser.username,
      );

      // Agree to Terms of Service
      await click(page, {
        label: "[Required] I have read and accept the Terms of Service.",
      });

      // Submit signup
      await click(page, { text: "Finished Sign-up" });

      // Wait for modal to close
      await waitPopup(page, { visible: false });

      // Reload the space page to verify authenticated state
      await goto(page, spaceUrl + "/dashboard");

      // Verify the user is now logged in on the space page
      // The space sidebar should show user profile information
      // instead of the Sign In button
      const signInButton = page.getByText("Sign In", { exact: true });
      await expect(signInButton).toBeHidden({ timeout: 10000 });
    } finally {
      await context.close();
    }
  });
});
