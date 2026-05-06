// @ts-check
/**
 * Docs recording: Getting Started → Sign up & sign in
 *
 * Records the email-signup flow end-to-end so the docs can embed a video
 * showing a brand-new user joining Ratel:
 *   1. Land on the home page
 *   2. Open the "Join the Movement" popup
 *   3. Switch to "Create an account"
 *   4. Send verification code, paste "000000", verify
 *   5. Fill password, display name, username
 *   6. Accept ToS, finish signup
 *   7. Land back on the home page logged in (Drafts button visible)
 *
 * Requires backend built with `--features bypass` so verification accepts
 * the hardcoded code "000000". This spec is intentionally fresh-context
 * (no shared storageState) so each run produces a clean signup recording.
 *
 * Output: Playwright captures a per-test .webm video under
 *   playwright/test-results/docs-getting-started-signup-*-/video.webm
 * The recording driver (companion script) converts to .mp4 / .gif and
 * copies into docs/static/media/getting-started/signup.<ext>.
 */

import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, waitPopup } from "../utils";

// One serial describe so all steps are recorded into a single video file.
test.describe.serial("docs/getting-started — signup", () => {
  let context;
  let page;

  // Unique-per-run user so the recording is repeatable on a fresh DB.
  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const docsUser = {
    email: `e2e_docs_signup_${uniqueId}@biyard.co`,
    username: `du${uniqueId}`,
    displayName: `Docs Demo ${uniqueId}`,
    password: "Test!234",
  };

  test.beforeAll(async ({ browser }) => {
    // Fresh context — no auth state, no cookies. Forces full signup flow.
    // Viewport matches the Desktop project (1440x950) so the recording is
    // consistent with the rest of the docs media set.
    context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 900 },
      recordVideo: {
        dir: "test-results/docs/getting-started/",
        size: { width: 1440, height: 900 },
      },
    });
    page = await context.newPage();
  });

  test.afterAll(async () => {
    // Closing the context flushes the .webm to disk.
    await context.close();
  });

  test("01 — load home page", async () => {
    await goto(page, "/");
    await expect(page).toHaveURL("/");
    // Brief settle so the recording starts on a clean home view.
    await page.waitForTimeout(800);
  });

  test("02 — open Join the Movement popup", async () => {
    await click(page, { testId: "home-btn-signin" });
    await waitPopup(page, { visible: true });
    await page.waitForTimeout(400);
  });

  test("03 — switch to Create an account", async () => {
    await click(page, { text: "Create an account" });
    await getLocator(page, { placeholder: "Enter your email address" });
    await page.waitForTimeout(400);
  });

  test("04 — send verification code", async () => {
    await fill(
      page,
      { placeholder: "Enter your email address" },
      docsUser.email,
    );
    await click(page, { text: "Send" });
    await getLocator(page, { placeholder: "Enter the verification code" });
  });

  test("05 — paste bypass code & verify", async () => {
    await fill(
      page,
      { placeholder: "Enter the verification code" },
      "000000",
    );
    await click(page, { text: "Verify" });
    await expect(page.getByText("Send", { exact: true })).toBeHidden({
      timeout: 10000,
    });
  });

  test("06 — fill password, display name, username", async () => {
    await fill(
      page,
      { placeholder: "Enter your password" },
      docsUser.password,
    );
    await fill(
      page,
      { placeholder: "Re-enter your password" },
      docsUser.password,
    );
    await fill(
      page,
      { placeholder: "Enter your display name" },
      docsUser.displayName,
    );
    await fill(
      page,
      { placeholder: "Enter your user name" },
      docsUser.username,
    );
    await page.waitForTimeout(400);
  });

  test("07 — accept ToS and finish signup", async () => {
    await click(page, {
      label: "[Required] I have read and accept the Terms of Service.",
    });
    await click(page, { text: "Finished Sign-up" });
    await waitPopup(page, { visible: false });
  });

  test("08 — land on home, logged in", async () => {
    await goto(page, "/");
    await getLocator(page, { testId: "home-btn-drafts" });
    await expect(page.getByTestId("home-btn-signin")).toBeHidden();
    // Hold the final frame so the video closes on a clear "logged in" view.
    await page.waitForTimeout(1500);
  });
});
