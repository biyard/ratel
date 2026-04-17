import { test, expect } from "@playwright/test";
import {
  click,
  clickNoNav,
  createTeamFromHome,
  createTeamPostFromHome,
  fill,
  goto,
  getLocator,
  getEditor,
  waitPopup,
  addPollQuestion,
  fillPollQuestion,
  togglePrerequisite,
  commitAutosave,
} from "../utils";

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
  // Pause card-float animation so Playwright clicks don't miss
  await page.addStyleTag({
    content: "*, *::before, *::after { animation-play-state: paused !important; }",
  });
  await clickNoNav(page, { testId: "btn-signin" });
  await waitPopup(page, { visible: true });
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
    // Verify credential (bypass mode — just click the button)
    await page.getByTestId("btn-verify").click({ force: true });
    await page.waitForLoadState();

    await page.addStyleTag({
      content: "*, *::before, *::after { animation-play-state: paused !important; }",
    });
    await clickNoNav(page, { testId: "btn-participate" });

    // ConsentModal appears — check the consent checkbox and confirm
    await expect(page.getByTestId("card-consent")).toBeVisible();
    await page.locator('input[type="checkbox"]').check();
    await click(page, { testId: "btn-consent-confirm" });

    // Wait for consent modal to disappear (server call + role transition)
    await expect(page.getByTestId("card-consent")).toBeHidden({
      timeout: 15000,
    });

    // PrerequisiteCard appears — user sees the checklist but doesn't complete anything
    await expect(page.getByTestId("card-prerequisite")).toBeVisible({
      timeout: 30000,
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
    // Verify credential (bypass mode — just click the button)
    await page.getByTestId("btn-verify").click({ force: true });
    await page.waitForLoadState();

    await page.addStyleTag({
      content: "*, *::before, *::after { animation-play-state: paused !important; }",
    });
    await clickNoNav(page, { testId: "btn-participate" });

    // ConsentModal appears — check the consent checkbox and confirm
    await expect(page.getByTestId("card-consent")).toBeVisible();
    await page.locator('input[type="checkbox"]').check();
    await click(page, { testId: "btn-consent-confirm" });

    // Wait for consent modal to disappear (server call + role transition)
    await expect(page.getByTestId("card-consent")).toBeHidden({
      timeout: 15000,
    });

    // PrerequisiteCard appears with the checklist
    await expect(page.getByTestId("card-prerequisite")).toBeVisible({
      timeout: 30000,
    });

    // Click the prerequisite poll item — opens the full-screen poll overlay
    const prereqItem = page.getByTestId("card-prerequisite").locator(".prereq-item").first();
    await prereqItem.click();

    // Poll overlay appears with an Overview screen first
    const overlay = page.getByTestId("poll-arena-overlay");
    await expect(overlay).toBeVisible();

    // Click "Begin Poll" to enter the question view
    await clickNoNav(page, { testId: "poll-arena-begin" });

    // Wait for poll options to load
    await expect(overlay.locator(".option-single").first()).toBeVisible({
      timeout: 30000,
    });

    // Select the first poll option inside the overlay
    await overlay.locator(".option-single").first().click();

    // Submit the poll using testId (avoids ambiguity with confirm dialog's Submit text)
    await clickNoNav(page, { testId: "poll-submit" });

    // Confirm dialog appears — click confirm. Poll submit now closes the
    // overlay in place (no navigation), so we must not wait for a load event.
    await clickNoNav(page, { testId: "poll-confirm-submit" });

    // Wait for overlay to close (server call completes + overlay signal cleared)
    await expect(page.getByTestId("poll-arena-overlay")).toBeHidden({
      timeout: 30000,
    });

    // After completing all prerequisites, user should see the WaitingCard
    await expect(page.getByTestId("card-waiting")).toBeVisible({
      timeout: 30000,
    });
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
    // Drive the full setup through the production UI:
    //   home (`/`) → Teams HUD → "Create Team" → ArenaTeamCreationPopup → submit
    // then home → Teams HUD → pick team → team home → "Create Post".
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "E2E test team for space actions",
    });

    const postId = await createTeamPostFromHome(
      page,
      teamUsername,
      postTitle,
      postContents,
    );

    // Space creation stays REST — the post-edit "Go to Space" affordance was
    // removed, and this suite focuses on governance rather than space-creation.
    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(spaceRes.ok(), `create space: ${await spaceRes.text()}`).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;

    spaceUrl = `/spaces/${spaceId}`;

    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
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

    // Arena-style quiz creator page: no tabs — ContentCard + QuestionsCard +
    // ConfigCard are all inline on the same page with per-field autosave.

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

    // Attach study material files via the inline Attachments section
    const dummyPdf = {
      name: "study-material-1.pdf",
      mimeType: "application/pdf",
      buffer: Buffer.from("%PDF-1.4 dummy content"),
    };
    const fileInputs = page.locator('input[type="file"][accept*=".pdf"]');
    if ((await fileInputs.count()) > 0) {
      const uploader = fileInputs.first();
      await uploader.setInputFiles(dummyPdf);
      await uploader.setInputFiles({
        ...dummyPdf,
        name: "study-material-2.pdf",
      });
      await uploader.setInputFiles({
        ...dummyPdf,
        name: "study-material-3.pdf",
      });
    }

    // INFO: available credits is zero when free membership — reward boost
    // configuration removed with the arena migration of the creator page.

    // QuestionsCard (arena-style): clicking the add button creates a
    // SingleChoice question with two empty option inputs directly. There is
    // no question-type picker modal and no "Add Option" button in the new
    // UI, so we just fill the default options in place.
    await click(page, { testId: "quiz-question-add" });
    await page.waitForLoadState("load");

    const q0 = page.getByTestId("quiz-question-0");
    const q0Inputs = q0.locator("input.input");
    await q0Inputs
      .nth(0)
      .fill("What is the primary goal of decentralized governance?");
    await q0Inputs.nth(1).fill("Centralize all decisions in one authority");
    await q0Inputs.nth(2).fill("Enable transparent collective decision-making");

    // Mark option 2 as the correct answer via the radio dot
    await page.getByTestId("quiz-question-0-opt-1").locator(".q-opt__radio").click();
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
    await commitAutosave(page);

    // Arena poll editor: two option inputs, no Add Option button.
    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "How familiar are you with decentralized governance?",
      options: ["Very familiar", "Not familiar at all"],
    });

    // Mark as prerequisite via the ConfigCard tile (no more Settings tab).
    await togglePrerequisite(page);
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
    await commitAutosave(page);

    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title:
        "After studying the materials, how would you rate your understanding?",
      options: ["Significantly improved", "No change"],
    });

    // Reward + boost setup: free-tier credits would be 0, so the boost
    // steps from the legacy Settings tab are intentionally skipped here.
    // setReward(page, 10) can be invoked once the user has paid credits.
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
    // Discussion's creator route ends with /edit in the arena editor.
    await page.waitForURL(/\/actions\/discussions\/[^/]+\/edit/, {
      waitUntil: "networkidle",
    });

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Governance Framework Discussion"
    );
    // Arena discussion editor has no category field and no Save button —
    // fields autosave on blur.

    const editor = await getEditor(page);
    await editor.fill(
      "This is the main governance discussion thread. Share your insights after completing " +
        "the study materials and preliminary survey."
    );
    await commitAutosave(page);

    // Reward + boost setup: free-tier credits would be 0, so the boost
    // steps from the legacy Settings tab are intentionally skipped here.
    // setReward(page, 5) can be invoked once the user has paid credits.
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
