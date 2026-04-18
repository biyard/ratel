import { test, expect } from "@playwright/test";
import {
  click,
  clickNoNav,
  createTeamFromHome,
  createTeamPostFromHome,
  fill,
  goto,
  gotoFresh,
  getLocator,
  getEditor,
  waitPopup,
  addPollQuestion,
  fillPollQuestion,
  togglePrerequisite,
  setReward,
  commitAutosave,
} from "../utils";

// This test requires the backend to be built with --features bypass
// for signup verification with hardcoded code "000000".
//
// Scenario:
//   Creator (A): hi+admin1@biyard.co — logs in, creates a team + post with space,
//     creates Follow (2 credits), Quiz (2 credits), Poll (prerequisite),
//     final Poll (one day later), configures Panel app (Age + Gender), publishes.
//   User (B): hi+user-{ts}@biyard.co — signs up via space, participates.

test.describe
  .serial("Reward anonymous team space with collective panel", () => {
  let spaceUrl;

  const adminEmail = "hi+admin1@biyard.co";
  const adminPassword = "admin!234";
  const teamNickname = "Reward Panel Team";
  const teamUsername = `e2e_rp_${Date.now()}`;
  const postTitle = "Reward Panel E2E Test Post";
  const postContents =
    "This is a comprehensive end-to-end test post for verifying the reward " +
    "and collective panel flow. It covers team creation, space setup with " +
    "follow/quiz/poll actions including credit rewards and prerequisite " +
    "settings, panel attribute configuration for age and gender, and " +
    "anonymous user signup with space participation.";

  // Helper: hide the floating action button that may overlap modal buttons
  async function hideFab(page) {
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });
  }

  // Helper: login as admin1 in a fresh context
  async function loginAsAdmin(browser) {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    await goto(page, "/");
    await click(page, { label: "Sign In" });
    await fill(page, { placeholder: "Enter your email address" }, adminEmail);
    await click(page, { text: "Continue" });
    await fill(page, { placeholder: "Enter your password" }, adminPassword);
    await click(page, { text: "Continue" });
    await waitPopup(page, { visible: false });

    // Save storageState for reuse across serial tests
    await context.storageState({ path: "admin1.json" });

    return { context, page };
  }

  test("Login as admin and create a team with post and space", async ({
    browser,
  }) => {
    const { context, page } = await loginAsAdmin(browser);

    try {
      // Drive the full setup through the production UI:
      //   home (`/`) → Teams HUD → "Create Team" → ArenaTeamCreationPopup → submit
      // then home → Teams HUD → pick team → team home → Create Post button.
      await createTeamFromHome(page, {
        username: teamUsername,
        nickname: teamNickname,
        description: "E2E test team for reward panel flow",
      });

      const postId = await createTeamPostFromHome(
        page,
        teamUsername,
        postTitle,
        postContents,
      );

      // Space creation stays REST — the post-edit "Go to Space" affordance
      // was removed, and this suite focuses on reward/panel rather than
      // space-creation UX.
      const spaceRes = await page.request.post("/api/spaces/create", {
        data: { req: { post_id: postId } },
      });
      expect(spaceRes.ok(), `create space: ${await spaceRes.text()}`).toBeTruthy();
      const spaceId = (await spaceRes.json()).space_id;

      spaceUrl = `/spaces/${spaceId}`;

      await goto(page, `${spaceUrl}/dashboard`);
      await getLocator(page, { text: "Dashboard" });
    } finally {
      await context.close();
    }
  });

  test("Create a follow action with 2 credits", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Arena: admin's add-action card opens the TypePicker directly and
      // picking a type creates + navigates to the creator page. There is no
      // intermediate "Create" confirmation and no /actions list page.
      await gotoFresh(page, spaceUrl);
      await hideFab(page);
      await click(page, { testId: "admin-add-action-card" });
      await click(page, { testId: "type-option-follow" });

      await page.waitForURL(/\/actions\/follows\//, {
        waitUntil: "load",
        timeout: 60000,
      });

      // Wait for hydration by looking for a testid on the ConfigCard.
      await getLocator(page, { testId: "page-card-config" });

      // Reward toggle is only rendered for paid memberships with credits.
      // setReward no-ops when it is absent.
      await setReward(page, 2);
    } finally {
      await context.close();
    }
  });

  test("Create a quiz action with 2 credits", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await gotoFresh(page, spaceUrl);
      await hideFab(page);
      await click(page, { testId: "admin-add-action-card" });
      // Quiz is the default pick in the TypePicker.
      await click(page, { testId: "type-option-quiz" });

      await page.waitForURL(/\/actions\/quizzes\//, {
        waitUntil: "load",
        timeout: 60000,
      });

      // Arena quiz creator: ContentCard + QuestionsCard + ConfigCard inline
      // with per-field autosave. No tabs, no Save button.
      await fill(
        page,
        { placeholder: "Enter quiz title..." },
        "Reward Quiz: Knowledge Check",
      );

      const editor = await getEditor(page);
      await editor.fill(
        "This quiz tests participant knowledge and awards reward credits upon completion.",
      );
      await commitAutosave(page);

      // Single question with per-field blur so each onblur autosave commits.
      await click(page, { testId: "quiz-question-add" });
      const q0 = page.getByTestId("quiz-question-0");
      const q0Inputs = q0.locator("input.input");
      const fills = [
        "What does DAO stand for?",
        "Decentralized Autonomous Organization",
        "Digital Asset Operation",
      ];
      for (let i = 0; i < fills.length; i += 1) {
        await q0Inputs.nth(i).fill(fills[i]);
        await q0Inputs.nth(i).press("Tab");
        await page.waitForLoadState("load");
        await page.waitForTimeout(200);
      }

      // Reward toggle + 2 credits — no-op when the workspace is free-tier.
      await setReward(page, 2);
    } finally {
      await context.close();
    }
  });

  test("Create a poll with prerequisite enabled", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await gotoFresh(page, spaceUrl);
      await hideFab(page);
      await click(page, { testId: "admin-add-action-card" });
      await click(page, { testId: "type-option-poll" });

      await page.waitForURL(/\/actions\/polls\//, {
        waitUntil: "load",
        timeout: 60000,
      });

      await fill(
        page,
        { placeholder: "Enter poll title..." },
        "Prerequisite Poll: Budget Allocation",
      );
      await commitAutosave(page);

      await addPollQuestion(page, "single");
      await fillPollQuestion(page, 0, {
        title: "How should the budget be allocated?",
        options: ["Marketing", "R&D"],
      });

      // Prerequisite is toggled via the ConfigCard tile (no Settings tab).
      await togglePrerequisite(page);
    } finally {
      await context.close();
    }
  });

  test("Create a final poll scheduled one day later", async ({ browser }) => {
    test.setTimeout(120000);

    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await gotoFresh(page, spaceUrl);
      await hideFab(page);
      await click(page, { testId: "admin-add-action-card" });
      await click(page, { testId: "type-option-poll" });

      await page.waitForURL(/\/actions\/polls\//, {
        waitUntil: "load",
        timeout: 60000,
      });

      await fill(
        page,
        { placeholder: "Enter poll title..." },
        "Final Poll: One Day Later",
      );
      await commitAutosave(page);

      await addPollQuestion(page, "single");
      await fillPollQuestion(page, 0, {
        title: "Should we proceed with the proposal?",
        options: ["Yes", "No"],
      });

      // Arena Schedule: native datetime-local inputs (schedule-start /
      // schedule-end). Set start = tomorrow, end = day after tomorrow.
      const fmt = (d) => {
        const y = d.getFullYear();
        const mo = String(d.getMonth() + 1).padStart(2, "0");
        const da = String(d.getDate()).padStart(2, "0");
        const h = String(d.getHours()).padStart(2, "0");
        const mi = String(d.getMinutes()).padStart(2, "0");
        return `${y}-${mo}-${da}T${h}:${mi}`;
      };
      const tomorrow = new Date(Date.now() + 24 * 60 * 60 * 1000);
      const dayAfter = new Date(Date.now() + 2 * 24 * 60 * 60 * 1000);

      const startInput = page.getByTestId("schedule-start");
      await expect(startInput).toBeVisible();
      await startInput.fill(fmt(tomorrow));
      await startInput.blur();
      await page.waitForLoadState("load");

      const endInput = page.getByTestId("schedule-end");
      await expect(endInput).toBeVisible();
      await endInput.fill(fmt(dayAfter));
      await endInput.blur();
      await page.waitForLoadState("load");
      await page.waitForTimeout(500);
    } finally {
      await context.close();
    }
  });

  test("Configure Panel app with Age and Gender attributes", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl + "/apps");

      // The Panels app card has a "Setting" button with data-testid="configure-app-panels".
      // But Panels might need to be installed first if not default.
      // Check if Panels is in "Available Apps" or "Installed Apps".
      const installButton = page.locator('[data-testid="install-app-panels"]');
      if ((await installButton.count()) > 0) {
        // Panels app needs to be installed first
        await installButton.click();
        await page.waitForLoadState("load");
        // Wait for the app to appear in installed section
        await page.waitForTimeout(1000);
      }

      // Now click Settings on the Panels app card
      const settingButton = page.locator(
        '[data-testid="configure-app-panels"]',
      );
      // There may be multiple "Setting" buttons — the Panels one should be visible
      // after install. Find the one associated with Panels.
      // Each AppCard renders the Setting button. We need the Panels one.
      // The cards show app names. Find "Panels" text and its sibling Setting button.
      const panelsCard = page.locator("text=Panels").locator("../..");
      const panelsSettingBtn = panelsCard.locator(
        '[data-testid="configure-app-panels"]',
      );

      if ((await panelsSettingBtn.count()) > 0) {
        await panelsSettingBtn.click();
      } else {
        // Fallback: click any configure-app-panels button
        await settingButton.first().click();
      }

      await page.waitForURL(/\/apps\/panels/, { waitUntil: "load" });

      // Toggle Age attribute button via data-testid
      await getLocator(page, { text: "Attribute groups" });
      await click(page, { testId: "attr-btn-age" });
      await page.waitForLoadState("load");

      // Toggle Gender attribute button via data-testid
      await click(page, { testId: "attr-btn-gender" });
      await page.waitForLoadState("load");

      // Verify collective panel shows Age and Gender badges
      await getLocator(page, { text: "Collective Panel Attributes" });
    } finally {
      await context.close();
    }
  });

  test("Publish the space as public", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl + "/dashboard");

      await click(page, { text: "Publish" });
      await click(page, { testId: "public-option" });
      await click(page, { label: "Confirm visibility selection" });
    } finally {
      await context.close();
    }
  });

  test("New user signs up and participates in the space", async ({
    browser,
  }) => {
    // Create a fresh anonymous context
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Navigate to the published space (ArenaViewer at root URL)
      await goto(page, spaceUrl);

      // Pause CSS animations — card-float moves buttons continuously,
      // causing Playwright clicks to miss the target.
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      // Click Sign In on the ArenaViewer's SigninCard
      await clickNoNav(page, { testId: "btn-signin" });
      await waitPopup(page, { visible: true });

      // Switch to signup
      await click(page, { text: "Create an account" });

      // Fill signup form
      const ts = Date.now();
      const signupEmail = `hi+user-${ts}@biyard.co`;
      await fill(
        page,
        { placeholder: "Enter your email address" },
        signupEmail,
      );

      // Send and verify code (requires --features bypass)
      await click(page, { text: "Send" });
      await fill(
        page,
        { placeholder: "Enter the verification code" },
        "000000",
      );
      await click(page, { text: "Verify" });
      await expect(page.getByText("Send", { exact: true })).toBeHidden({
        timeout: 10000,
      });

      // Fill password
      await fill(page, { placeholder: "Enter your password" }, "Test!234");
      await fill(page, { placeholder: "Re-enter your password" }, "Test!234");

      // Fill display name and username
      await fill(
        page,
        { placeholder: "Enter your display name" },
        `E2E User ${ts}`,
      );
      await fill(page, { placeholder: "Enter your user name" }, `u${ts}`);

      // Agree to Terms of Service
      await click(page, {
        label: "[Required] I have read and accept the Terms of Service.",
      });

      // Submit signup
      await click(page, { text: "Finished Sign-up" });
      await waitPopup(page, { visible: false });

      await page.getByTestId("btn-verify").click({ force: true });
      await page.waitForLoadState();

      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      await clickNoNav(page, { testId: "btn-participate" });

      // Since Age and Gender panels are configured (collective type),
      // the ConsentModal appears. Collective panels collect self-declared
      // demographics during consent — no credential verification needed.
      await expect(page.getByTestId("card-consent")).toBeVisible();
      await page.locator('input[type="checkbox"]').check();
      await page.getByTestId("btn-consent-confirm").click();

      await page.waitForLoadState();
      // Wait for consent modal to disappear (server call + role transition)
      // await expect(page.getByTestId("card-consent")).toBeHidden({
      //   timeout: 15000,
      // });

      // PrerequisiteCard appears with the checklist
      await expect(page.getByTestId("card-prerequisite")).toBeVisible({
        timeout: 30000,
      });

      // Click the prerequisite poll item — opens the full-screen poll overlay.
      // The prereq list shows the poll question as the title, not the poll name.
      const prereqItem = page
        .getByTestId("card-prerequisite")
        .locator(".prereq-item")
        .first();
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

      // Submit the poll using testId
      await clickNoNav(page, { testId: "poll-submit" });

      // Confirm dialog appears — click confirm. Poll submit closes the
      // overlay in place (no navigation).
      await clickNoNav(page, { testId: "poll-confirm-submit" });

      // Wait for overlay to close
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
  });
});
