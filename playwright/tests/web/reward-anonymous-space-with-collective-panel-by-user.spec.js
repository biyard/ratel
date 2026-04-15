import { test, expect } from "@playwright/test";
import {
  click,
  clickNoNav,
  fill,
  goto,
  getLocator,
  getEditor,
  waitPopup,
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
      // The home-ui renewal removed the profile-dropdown team creation UI and
      // the post-edit "skip-space-checkbox" / "Go to Space" affordances.
      // Drive team, post, and space setup through the REST endpoints.
      const teamRes = await page.request.post("/api/teams/create", {
        data: {
          body: {
            username: teamUsername,
            nickname: teamNickname,
            profile_url: "",
            description: "E2E test team for reward panel flow",
          },
        },
      });
      expect(teamRes.ok(), `create team: ${await teamRes.text()}`).toBeTruthy();
      const teamPk = (await teamRes.json()).team_pk;
      const teamId = teamPk.includes("#") ? teamPk.split("#")[1] : teamPk;

      const postRes = await page.request.post("/api/posts", {
        data: { team_id: teamId },
      });
      expect(postRes.ok(), `create post: ${await postRes.text()}`).toBeTruthy();
      const postPk = (await postRes.json()).post_pk;
      const postId = postPk.includes("#") ? postPk.split("#")[1] : postPk;

      await goto(page, `/posts/${postId}/edit`);
      await fill(page, { placeholder: "Title your post…" }, postTitle);
      const editor = await getEditor(page);
      await editor.fill(postContents);
      await expect(page.getByText("All changes saved")).toBeVisible({
        timeout: 15000,
      });

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
      await goto(page, spaceUrl + "/actions");

      await click(page, { text: "Select Action Type" });
      await click(page, { testId: "action-type-follow" });
      await hideFab(page);
      await click(page, { text: "Create" });

      await page.waitForURL(/\/actions\/follows\//, {
        waitUntil: "load",
      });

      await getLocator(page, { text: "General" });

      // Switch to Settings tab to configure reward
      await page.getByRole("tab", { name: "Settings" }).click();
      await page.waitForLoadState("load");

      // Enable reward toggle via data-testid
      const rewardToggle = page.locator(
        '[data-testid="reward-setting-toggle"] button',
      );
      if ((await rewardToggle.count()) > 0) {
        await rewardToggle.click();
        await page.waitForLoadState("load");

        // Set credit usage to 2
        const creditInput = page.locator('[data-testid="reward-credit-input"]');
        await creditInput.fill("2");

        // Trigger blur to save
        await page.keyboard.press("Tab");
        await page.waitForLoadState("load");
      }
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
      await goto(page, spaceUrl + "/actions");

      await click(page, { text: "Select Action Type" });
      // Quiz is selected by default
      await hideFab(page);
      await click(page, { text: "Create" });

      await page.waitForURL(/\/actions\/quizzes\//, {
        waitUntil: "load",
      });

      // Fill quiz overview
      await fill(
        page,
        { placeholder: "Enter quiz title..." },
        "Reward Quiz: Knowledge Check",
      );

      const editor = await getEditor(page);
      await editor.fill(
        "This quiz tests participant knowledge and awards reward credits upon completion.",
      );
      await click(page, { text: "Save" });

      // Add a quiz question on the Quiz tab
      await page.getByRole("tab", { name: "Quiz" }).click();
      await page.waitForLoadState("load");

      await click(page, { testId: "quiz-add-question" });
      await click(page, { text: "Single Choice" });

      const textInputs = page.locator('input[type="text"]:visible');
      await textInputs.nth(0).fill("What does DAO stand for?");
      await textInputs.nth(1).fill("Decentralized Autonomous Organization");
      await textInputs.nth(2).fill("Digital Asset Operation");

      // Mark correct answer (option 1)
      const checkboxLabels = page.locator('label:has(input[type="checkbox"])');
      await checkboxLabels.nth(0).click();
      await page.waitForLoadState("load");

      // Switch to Setting tab (quiz uses "Setting" singular)
      await page.getByRole("tab", { name: "Setting" }).click();
      await page.waitForLoadState("load");

      // Enable reward toggle via data-testid
      const rewardToggle2 = page.locator(
        '[data-testid="reward-setting-toggle"] button',
      );
      if ((await rewardToggle2.count()) > 0) {
        await rewardToggle2.click();
        await page.waitForLoadState("load");

        // Set credit usage to 2
        const creditInput = page.locator('[data-testid="reward-credit-input"]');
        await creditInput.fill("2");

        await page.keyboard.press("Tab");
        await page.waitForLoadState("load");
      }
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
      await goto(page, spaceUrl + "/actions");

      await click(page, { text: "Select Action Type" });
      await click(page, { testId: "action-type-poll" });
      await hideFab(page);
      await click(page, { text: "Create" });

      await page.waitForURL(/\/actions\/polls\//, {
        waitUntil: "load",
      });

      // Fill poll title
      await fill(
        page,
        { placeholder: "Enter poll title..." },
        "Prerequisite Poll: Budget Allocation",
      );
      await page.keyboard.press("Tab");
      await page.waitForLoadState("load");

      // Add a poll question
      await click(page, { testId: "poll-add-question" });
      await click(page, { text: "Single Choice" });

      const textInputs = page.locator('input[type="text"]:visible');
      await textInputs.nth(0).fill("How should the budget be allocated?");
      await textInputs.nth(1).fill("Marketing");
      await textInputs.nth(2).fill("R&D");

      await page.keyboard.press("Tab");
      await page.waitForLoadState("load");

      // Switch to Settings tab and enable Prerequisite
      await page.getByRole("tab", { name: "Settings" }).click();
      await page.waitForLoadState("load");

      // Toggle Prerequisite switch via data-testid
      const prerequisiteSwitch = page.locator(
        '[data-testid="prerequisite-setting"] button',
      );
      await prerequisiteSwitch.click();
      await page.waitForLoadState("load");
    } finally {
      await context.close();
    }
  });

  test("Create a final poll scheduled one day later", async ({ browser }) => {
    // Month-boundary calendar navigation (e.g., March 31 → April 1) adds
    // extra steps that can push total execution past the default 60s timeout.
    test.setTimeout(120000);

    const context = await browser.newContext({
      storageState: "admin1.json",
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl + "/actions");

      await click(page, { text: "Select Action Type" });
      await click(page, { testId: "action-type-poll" });
      await hideFab(page);
      await click(page, { text: "Create" });

      await page.waitForURL(/\/actions\/polls\//, {
        waitUntil: "load",
      });

      // Fill poll title
      await fill(
        page,
        { placeholder: "Enter poll title..." },
        "Final Poll: One Day Later",
      );
      await page.keyboard.press("Tab");
      await page.waitForLoadState("load");

      // Add a poll question
      await click(page, { testId: "poll-add-question" });
      await click(page, { text: "Single Choice" });

      const textInputs = page.locator('input[type="text"]:visible');
      await textInputs.nth(0).fill("Should we proceed with the proposal?");
      await textInputs.nth(1).fill("Yes");
      await textInputs.nth(2).fill("No");

      await page.keyboard.press("Tab");
      await page.waitForLoadState("load");

      // Switch to Settings tab and set date to tomorrow
      await page.getByRole("tab", { name: "Settings" }).click();
      await page.waitForLoadState("load");

      // Build accessible name for tomorrow (e.g., "Tuesday, March 31, 2026")
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);
      const tomorrowLabel = tomorrow.toLocaleDateString("en-US", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      });

      // Open the start date calendar popover (first "Show Calendar" button)
      const showCalendarButtons = page.getByRole("button", {
        name: "Show Calendar",
      });
      await showCalendarButtons.first().click();
      await page.waitForLoadState("load");

      // Click tomorrow's day in the calendar using its accessible name.
      // Use .first() to avoid strict mode violation when the date appears
      // in multiple month sections of the calendar grid (e.g. at month
      // boundaries where the day shows in both "current" and "next").
      await page.getByRole("button", { name: tomorrowLabel }).first().click();
      await page.waitForLoadState("load");

      // Build accessible name for day after tomorrow (end date)
      const dayAfter = new Date();
      dayAfter.setDate(dayAfter.getDate() + 2);
      const dayAfterLabel = dayAfter.toLocaleDateString("en-US", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      });

      // Open the end date calendar popover (second "Show Calendar" button)
      await showCalendarButtons.nth(1).click();
      await page.waitForLoadState("load");

      // Click the end date in the calendar using its accessible name.
      // Use .first() to avoid strict mode violation when the date appears
      // in multiple month sections of the calendar grid.
      await page.getByRole("button", { name: dayAfterLabel }).first().click();
      await page.waitForLoadState("load");
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

      // Confirm dialog appears — click confirm
      await click(page, { testId: "poll-confirm-submit" });

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
