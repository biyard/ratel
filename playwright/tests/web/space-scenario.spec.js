import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor, waitPopup } from "../utils";

/**
 * Space Governance Scenario — Full E2E
 *
 * Users:
 *   - Creator1    : hi+user1@biyard.co  — pre-authenticated (user.json)
 *   - Participant1: hi+user2@biyard.co  — existing account, invited as team Member
 *                                         (non-admin → space viewer role)
 *   - Viewer      : fresh sign-up via space page
 *                   → clicks Participate but does NOT complete prerequisite poll
 *   - Participant2 : fresh sign-up via space page → completes prerequisite poll
 *   - Participant3 : fresh sign-up via space page → completes prerequisite poll
 *
 * Test execution order:
 *   1.  Creator1: Create team
 *   2.  Creator1: Invite Participant1 as Member (non-admin → viewer role in space)
 *   3.  Creator1: Create post + space
 *   4.  Creator1: Add Follow Team action
 *   5.  Creator1: Add Quiz with 3 study attachments + 2x boost
 *   6.  Creator1: Add preliminary Poll (사전조사, prerequisite)
 *   7.  Creator1: Add final Poll (최종조사, 10x boost)
 *   8.  Creator1: Add Discussion (not published, 5x boost)
 *   9.  Creator1: General settings — anonymous ON
 *   10. Creator1: Panel settings — age + gender, collective
 *   11. Creator1: Publish space as public
 *   12. Viewer:       Sign up from space + Participate (no prereq completion)
 *   13. Participant2: Sign up from space + complete prerequisite poll
 *   14. Participant3: Sign up from space + complete prerequisite poll
 *
 * NOTE: Requires backend built with `--features bypass` so that email
 *       verification accepts the hardcoded code "000000".
 */

// ─── User definitions ───────────────────────────────────────────────────────

const participant1 = {
  email: "hi+user2@biyard.co",
  password: "admin!234",
};

// ─── Helpers ────────────────────────────────────────────────────────────────

/**
 * Signs up a new user via the published space page's "Sign In" button.
 * Returns { context, page } — caller is responsible for context.close().
 */
async function signUpFromSpace(
  browser,
  { email, username, name, password },
  spaceUrl
) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
  });
  const page = await context.newPage();

  await goto(page, spaceUrl);
  await click(page, { testId: "btn-signin" });
  // "Create an account" link is inside the modal — waiting for it implies modal is open
  await click(page, { text: "Create an account" });

  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Send" });
  await fill(page, { placeholder: "Enter the verification code" }, "000000");
  await click(page, { text: "Verify" });
  // "Send" disappears once server confirms the code is valid
  await expect(page.getByText("Send", { exact: true })).toBeHidden({
    timeout: 10000,
  });

  await fill(page, { placeholder: "Enter your password" }, password);
  await fill(page, { placeholder: "Re-enter your password" }, password);
  await fill(page, { placeholder: "Enter your display name" }, name);
  await fill(page, { placeholder: "Enter your user name" }, username);
  await click(page, {
    label: "[Required] I have read and accept the Terms of Service.",
  });
  await click(page, { text: "Finished Sign-up" });
  await waitPopup(page, { visible: false });

  return { context, page };
}

/**
 * Signs up from the space page then clicks Participate but does NOT complete
 * the prerequisite poll — simulates a viewer who is registered but inactive.
 */
async function signUpAndSkipPrereq(browser, user, spaceUrl) {
  const { context, page } = await signUpFromSpace(browser, user, spaceUrl);

  try {
    await goto(page, spaceUrl);
    await click(page, { testId: "btn-participate" });

    // ConsentModal appears — check the consent checkbox and confirm
    await expect(page.getByTestId("card-consent")).toBeVisible();
    await page.locator('input[type="checkbox"]').check();
    await click(page, { testId: "btn-consent-confirm" });

    // PrerequisiteCard appears — user sees the checklist but doesn't complete anything
    await expect(page.getByTestId("card-prerequisite")).toBeVisible({
      timeout: 15000,
    });
  } finally {
    await context.close();
  }
}

/**
 * Signs up from the space page then completes the prerequisite poll.
 */
async function signUpAndParticipate(browser, user, spaceUrl) {
  const { context, page } = await signUpFromSpace(browser, user, spaceUrl);

  try {
    await goto(page, spaceUrl);
    await click(page, { testId: "btn-participate" });

    // ConsentModal appears — check the consent checkbox and confirm
    await expect(page.getByTestId("card-consent")).toBeVisible();
    await page.locator('input[type="checkbox"]').check();
    await click(page, { testId: "btn-consent-confirm" });

    // PrerequisiteCard appears with the checklist
    await expect(page.getByTestId("card-prerequisite")).toBeVisible({
      timeout: 15000,
    });

    // Click the prerequisite poll item — opens the full-screen poll overlay
    await click(page, { text: "Opinion Survey: Pre-study" });
    // Poll overlay appears (no page navigation)
    await expect(page.getByTestId("poll-arena-overlay")).toBeVisible();

    // Select the first radio option inside the overlay
    const options = page.locator(
      '[data-testid="poll-arena-overlay"] input[type="radio"]:visible'
    );
    if ((await options.count()) > 0) {
      await options.first().click();
    } else {
      await page
        .locator('[data-testid="poll-arena-overlay"] [data-pw="poll-option"]')
        .first()
        .click();
    }

    // Submit the poll — confirm dialog appears (response_editable is false by default)
    await click(page, { text: "Submit" });
    await click(page, { testId: "poll-confirm-submit" });
    await expect(page.getByTestId("poll-arena-overlay")).toBeHidden();

    // After completing all prerequisites, user should see the WaitingCard
    await expect(page.getByTestId("card-waiting")).toBeVisible({ timeout: 15000 });
  } finally {
    await context.close();
  }
}

async function openSpaceAppSettings(page, spaceUrl, appType) {
  await goto(page, spaceUrl + "/apps");

  const installButton = page.getByTestId(`install-app-${appType}`);
  if (await installButton.isVisible().catch(() => false)) {
    await installButton.click();
    await expect(page.getByTestId(`configure-app-${appType}`)).toBeVisible();
  }

  await click(page, { testId: `configure-app-${appType}` });
}

// ─── Test suite ─────────────────────────────────────────────────────────────

test.describe.serial("Space governance scenario", () => {
  let spaceUrl;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const teamNickname = `E2E Gov Team ${uniqueId}`;
  const teamUsername = `e2e_gov_${uniqueId}`;

  const freshPassword = "Test!234";

  const viewer = {
    email: `e2e_vw_${uniqueId}@biyard.co`,
    username: `vw${uniqueId}`,
    name: `Viewer ${uniqueId}`,
    password: freshPassword,
  };

  const participant2 = {
    email: `e2e_p2_${uniqueId}@biyard.co`,
    username: `p2${uniqueId}`,
    name: `Participant Two ${uniqueId}`,
    password: freshPassword,
  };

  const participant3 = {
    email: `e2e_p3_${uniqueId}@biyard.co`,
    username: `p3${uniqueId}`,
    name: `Participant Three ${uniqueId}`,
    password: freshPassword,
  };

  const postTitle = "Governance Research Space";
  const postContents =
    "This space is created for structured governance research. It contains follow, quiz, poll, " +
    "and discussion actions to facilitate organized participation and data collection. " +
    "Participants complete prerequisite actions before accessing the main governance discussion.";

  // ─── 1. Create team ───────────────────────────────────────────────────────

  test("Create a team and post with space, then verify dashboard", async ({
    page,
  }) => {
    // Step 1: Navigate to home and open profile dropdown
    await goto(page, "/");

    // Open profile dropdown by clicking the user profile button in the navbar
    // Target the button specifically by its accessible name (img alt + text)
    await click(page, { label: "User Profile" });

    // Step 2: Click "Create Team" in the dropdown
    await click(page, { text: "Create Team" });

    // Step 3: Fill in team creation form
    // Team Name (nickname)
    const nicknameInput = page.locator('[data-testid="team-nickname-input"]');
    await nicknameInput.fill(teamNickname);

    // Team ID (username)
    const usernameInput = page.locator('[data-testid="team-username-input"]');
    await usernameInput.fill(teamUsername);

    // Team description
    const descInput = page.locator('[data-testid="team-description-input"]');
    await descInput.fill("E2E test team for space actions");

    // Click Create button to submit the form
    await click(page, { text: "Create" });

    // Wait for navigation to the team home page
    // Routes use /{username}/home (no /teams/ prefix)
    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "load",
    });
    await page.waitForFunction(
      () => document.querySelector("[data-dioxus-id]") !== null
    );

    // Step 4: Create a post via the Create button on the team home page
    await click(page, { text: "Create" });

    // Wait for post edit page to load
    await page.waitForURL(/\/posts\/.*\/edit/, {
      waitUntil: "load",
    });

    // Step 6: Fill in the post
    await fill(page, { placeholder: "Title" }, postTitle);

    // Uncheck "Skip creating space" to enable space creation
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Step 7: Click "Go to Space" to create the space
    await click(page, { text: "Go to Space" });

    // Wait for navigation to the space dashboard
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "load",
    });
    await getLocator(page, { text: "Dashboard" });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  });

  // TODO: add team member into team

  // ─── 3. Create post + space ───────────────────────────────────────────────
  // Started from team home while WASM is hydrated to avoid SSR-only onclick issue.

  // test("Creator1: Create post with space", async ({ page }) => {
  //   await goto(page, `/${teamUsername}/home`);

  //   await click(page, { text: "Create" });
  //   await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

  //   await fill(page, { placeholder: "Title" }, postTitle);
  //   await click(page, { testId: "skip-space-checkbox" });

  //   const editor = await getEditor(page);
  //   await editor.fill(postContents);

  //   await click(page, { text: "Go to Space" });

  //   await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
  //     waitUntil: "load",
  //   });
  //   await getLocator(page, { text: "Dashboard" });

  //   const url = new URL(page.url());
  //   spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  // });

  // ─── 4. Follow Team action ────────────────────────────────────────────────

  test("Creator1: Add Follow Team action", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-follow" });

    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    await click(page, { text: "Create" });
    await page.waitForURL(/\/actions\/follows\//, { waitUntil: "networkidle" });
  });

  // ─── 5. Quiz + 3 study attachments + 2x boost ────────────────────────────

  test("Creator1: Add Quiz with study materials and 2x boost", async ({
    page,
  }) => {
    await goto(page, spaceUrl + "/actions");

    await click(page, { text: "Select Action Type" });
    // Quiz is the default selection — no extra click needed

    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
    await click(page, { text: "Create" });
    await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "networkidle" });

    // Overview tab: title + description
    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Study Material Quiz: Governance Basics"
    );

    const editor = await getEditor(page);
    await editor.fill(
      "Read the attached study materials carefully before answering the quiz questions. " +
        "This quiz verifies understanding of the governance framework."
    );

    await page.getByRole("tab", { name: "Upload" }).click();
    await page.waitForLoadState("load");

    // Attach 3 dummy study material files
    const dummyPdf = {
      name: "study-material-1.pdf",
      mimeType: "application/pdf",
      buffer: Buffer.from("%PDF-1.4 dummy content"),
    };
    const fileInputs = page.locator('input[type="file"][accept*=".pdf"]');
    if ((await fileInputs.count()) > 0) {
      const uploader = fileInputs.first();

      await uploader.setInputFiles(dummyPdf);
      // FIXME: check this logic
      // await expect(
      //   page.getByText("study-material-1.pdf", { exact: true })
      // ).toBeVisible();

      await uploader.setInputFiles({
        ...dummyPdf,
        name: "study-material-2.pdf",
      });
      // await expect(
      //   page.getByText("study-material-2.pdf", { exact: true })
      // ).toBeVisible();

      await uploader.setInputFiles({
        ...dummyPdf,
        name: "study-material-3.pdf",
      });
      // await expect(
      //   page.getByText("study-material-3.pdf", { exact: true })
      // ).toBeVisible();
    }

    // Setting tab: 2x boost reward
    await page.getByRole("tab", { name: "Setting" }).click();
    await page.waitForLoadState("load");

    // INFO: available credits is zero when free membership
    // const rewardCard = page.locator("text=Reward").locator("../../..");
    // await rewardCard.locator("button[role='switch']").click();
    // await page.waitForLoadState("load");

    // const creditInput = page.locator('input[type="number"]:visible');
    // await creditInput.first().fill("2");
    // await page.keyboard.press("Tab");
    // await page.waitForLoadState("load");

    // Quiz tab: add a single-choice question
    await page.getByRole("tab", { name: "Quiz" }).click();
    await page.waitForLoadState("load");

    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs
      .nth(0)
      .fill("What is the primary goal of decentralized governance?");
    await textInputs.nth(1).fill("Centralize all decisions in one authority");
    await textInputs
      .nth(2)
      .fill("Enable transparent collective decision-making");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("Maximize individual profit");

    // Mark option 2 as the correct answer
    const checkboxLabels = page.locator('label:has(input[type="checkbox"])');
    await checkboxLabels.nth(1).click();
    await page.waitForLoadState("load");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");
  });

  // ─── 6. Poll 1 — 사전조사 (prerequisite) ─────────────────────────────────

  test("Creator1: Add preliminary Poll (사전조사, prerequisite)", async ({
    page,
  }) => {
    await goto(page, spaceUrl + "/actions");

    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });

    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    await click(page, { text: "Create" });
    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "networkidle" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Opinion Survey: Pre-study"
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs
      .nth(0)
      .fill("How familiar are you with decentralized governance?");
    await textInputs.nth(1).fill("Very familiar");
    await textInputs.nth(2).fill("Somewhat familiar");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("Not familiar at all");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    // Settings tab: mark as prerequisite
    await page.getByRole("tab", { name: "Settings" }).click();
    await page.waitForLoadState("load");

    const prerequisiteCard = page.locator("text=Prerequisite").locator("../..");
    await prerequisiteCard.locator("button").click();
    await page.waitForLoadState("load");
  });

  // ─── 7. Poll 2 — 최종조사 (10x boost) ────────────────────────────────────

  test("Creator1: Add final Poll (최종조사, 10x boost)", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });

    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    await click(page, { text: "Create" });
    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "networkidle" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Opinion Survey: Post-study"
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs
      .nth(0)
      .fill(
        "After studying the materials, how would you rate your understanding?"
      );
    await textInputs.nth(1).fill("Significantly improved");
    await textInputs.nth(2).fill("Somewhat improved");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("No change");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    // Settings tab: 10x boost reward
    await page.getByRole("tab", { name: "Settings" }).click();
    await page.waitForLoadState("load");

    // INFO: available credits is zero when free membership
    // const rewardCard = page.locator("text=Reward").locator("../../..");
    // await rewardCard.locator("button[role='switch']").click();
    // await page.waitForLoadState("load");

    // const creditInput = page.locator('input[type="number"]:visible');
    // await creditInput.first().fill("10");
    // await page.keyboard.press("Tab");
    // await page.waitForLoadState("load");
  });

  // ─── 8. Discussion — saved but not published, 5x boost ───────────────────

  test("Creator1: Add Discussion (not published, 5x boost)", async ({
    page,
  }) => {
    await goto(page, spaceUrl + "/actions");

    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-discussion" });

    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    await click(page, { text: "Create" });
    await page.waitForURL(/\/actions\/discussions\//, {
      waitUntil: "networkidle",
    });

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Governance Framework Discussion"
    );
    await fill(
      page,
      { placeholder: "Enter category (optional)..." },
      "Governance"
    );

    const editor = await getEditor(page);
    await editor.fill(
      "This is the main governance discussion thread. Share your insights after completing " +
        "the study materials and preliminary survey."
    );

    // Save without publishing
    await click(page, { text: "Save" });
    await page.waitForLoadState("load");

    // Settings tab: 5x boost reward
    await page.getByRole("tab", { name: "Setting" }).click();
    await page.waitForLoadState("load");

    // INFO: available credits is zero when free membership
    // const rewardCard = page.locator("text=Reward").locator("../../..");
    // await rewardCard.locator("button[role='switch']").click();
    // await page.waitForLoadState("load");

    // const creditInput = page.locator('input[type="number"]:visible');
    // await creditInput.first().fill("5");
    // await page.keyboard.press("Tab");
    // await page.waitForLoadState("load");
  });

  // ─── 9. General settings — anonymous ON ───────────────────────────────────

  test("Creator1: Configure space settings — anonymous ON", async ({
    page,
  }) => {
    await openSpaceAppSettings(page, spaceUrl, "general");
    await page.waitForURL(/\/apps\/general$/, { waitUntil: "load" });

    const anonymousSwitch = page.locator("text=Anonymous").locator("../..");
    await anonymousSwitch.locator("button[role='switch']").click();
    await page.waitForLoadState("load");
  });

  // ─── 10. Panel settings — age + gender, collective ───────────────────────

  test("Creator1: Configure panel — age + gender, collective type", async ({
    page,
  }) => {
    await openSpaceAppSettings(page, spaceUrl, "panels");
    await page.waitForURL(/\/apps\/panels$/, { waitUntil: "load" });

    const attributeGroups = page
      .getByText("Attribute groups", { exact: true })
      .locator("..");

    await attributeGroups.getByRole("button", { name: "Age" }).click();
    await page.waitForLoadState("networkidle");

    await attributeGroups.getByRole("button", { name: "Gender" }).click();
    await page.waitForLoadState("networkidle");

    await getLocator(page, { text: "Collective Panel Attributes" });
  });

  // ─── 11. Publish space as public ──────────────────────────────────────────

  test("Creator1: Publish space as public", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Publish" });
    await click(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
    await page.waitForLoadState("load");
  });

  // ─── 12. Viewer: sign up + Participate (no prereq completion) ────────────

  test("Viewer: Sign up from space and Participate without completing prereq", async ({
    browser,
  }) => {
    await signUpAndSkipPrereq(browser, viewer, spaceUrl);
  });

  // ─── 13–14. Participants: sign up + complete prerequisite poll ───────────

  test("Participant2: Sign up from space and complete prerequisite poll", async ({
    browser,
  }) => {
    await signUpAndParticipate(browser, participant2, spaceUrl);
  });

  test("Participant3: Sign up from space and complete prerequisite poll", async ({
    browser,
  }) => {
    await signUpAndParticipate(browser, participant3, spaceUrl);
  });
});
