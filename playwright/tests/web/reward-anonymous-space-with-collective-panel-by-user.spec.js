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

  test("Login as space admin and create a team with post and space", async ({
    page,
  }) => {
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
    expect(
      spaceRes.ok(),
      `create space: ${await spaceRes.text()}`,
    ).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;

    spaceUrl = `/spaces/${spaceId}`;

    // Route is `/dashboard/:..rest`, so the dx Routable's catch-all needs a
    // trailing slash to match an empty rest. Without it the path falls through
    // to PageNotFound. Other specs that go through the UI hit the redirect
    // path and avoid this; REST-created spaces navigate directly so we must
    // include the trailing slash explicitly.
    await goto(page, `${spaceUrl}/dashboard/`);
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard\/?$/, {
      waitUntil: "load",
      timeout: 10000,
    });
  });

  test("Create a follow action with 2 credits", async ({ page }) => {
    // Arena: admin's add-action card opens the TypePicker directly and
    // picking a type creates + navigates to the creator page. There is no
    // intermediate "Create" confirmation and no /actions list page.
    await goto(page, spaceUrl);
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
  });

  test("Create a quiz action with 2 credits", async ({ page }) => {
    await goto(page, spaceUrl);
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
  });

  test("Create a poll with prerequisite enabled", async ({ page }) => {
    await goto(page, spaceUrl);
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
  });

  test("Create a final poll scheduled one day later", async ({ page }) => {
    await goto(page, spaceUrl);
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

    // Schedule inputs were removed in the status-based refactor — actions
    // now transition via the creator's explicit Publish button. This test
    // leaves the poll in Designing; prerequisite visibility comes from
    // the prerequisite toggle later in the flow.
  });

  test("Configure Panel app with Age and Gender attributes", async ({
    page,
  }) => {
    // Real UI flow:
    //   open Settings panel from topbar (btn-settings)
    //   → if Panels not installed: click "+ 설치" (install-app-panels)
    //   → click "설정" on the now-installed Panels row (settings-app-panels)
    //   → navigates to /apps/panels
    // Direct goto to /apps/ no longer works because the apps list is rendered
    // inside the Settings sliding panel, not on a standalone /apps page.
    await goto(page, spaceUrl);
    await hideFab(page);

    // Open the Settings panel.
    await clickNoNav(page, { testId: "btn-settings" });
    await page.waitForSelector(
      '[data-testid="settings-panel"][data-open="true"]',
      { timeout: 10000 },
    );

    // Install the Panels app if it isn't already.
    const installPanels = page.locator('[data-testid="install-app-panels"]');
    if ((await installPanels.count()) > 0) {
      await installPanels.first().click();
      // Wait for the install to flip the row into the "Installed apps" section.
      // The settings button (settings-app-panels) only renders for installed
      // apps, so its appearance is the install confirmation.
      await page.waitForSelector('[data-testid="settings-app-panels"]', {
        timeout: 15000,
      });
    }

    // Open the Panels settings page.
    await page
      .locator('[data-testid="settings-app-panels"]')
      .first()
      .click();
    await page.waitForURL(/\/apps\/panels/, { waitUntil: "load" });

    // Toggle Age + Gender attribute cards. The panels page was
    // rewritten to the arena layout, which renames the card testids
    // from `attr-btn-{name}` (old DOM) to `attr-{name}` (see
    // `panels/views/home/attribute_groups.rs` → `AttributeToggleCard`).
    await getLocator(page, { text: "Attribute groups" });
    await clickNoNav(page, { testId: "attr-age" });
    await clickNoNav(page, { testId: "attr-gender" });

    // Visibility of "Collective Panel Attributes" is the auto-save
    // confirmation — the section only renders after at least one
    // collective attribute flips on. Replaces the earlier
    // `waitForLoadState("load")` filler which was racy.
    await getLocator(page, { text: "Collective Panel Attributes" });
  });

  test("Publish the space as public", async ({ page }) => {
    // Real UI flow:
    //   /spaces/{id}/ (arena viewer) → topbar publish icon (btn-publish)
    //   → SpaceVisibilityModal opens → pick "공개/Public" (public-option)
    //   → click "게시/Publish" (aria-label "Confirm visibility selection")
    // The previous flow used `{ text: "Publish" }` which doesn't match the
    // localized topbar copy ("게시" in Korean) and it relied on a now-removed
    // standalone /dashboard publish button.
    await goto(page, spaceUrl + "/");

    await clickNoNav(page, { testId: "btn-publish" });
    await waitPopup(page, { visible: true });
    await clickNoNav(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
    await waitPopup(page, { visible: false });
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

      // Passwordless email-code flow. Enter email → Continue (sends code) →
      // code "000000" → Continue (verifies). A brand-new email surfaces
      // UserNotFound, which opens the signup modal with the email + code
      // already verified — only profile fields remain.
      const ts = Date.now();
      const signupEmail = `hi+user-${ts}@biyard.co`;
      await fill(
        page,
        { placeholder: "Enter your email address" },
        signupEmail,
      );
      await click(page, { testId: "continue-button" });
      await fill(page, { testId: "code-input" }, "000000");
      await click(page, { testId: "continue-button" });

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

      // FR-2 #8 — fresh signups land on /onboarding/connections
      // (cross-posting post-signup interstitial). Skip it so the test
      // can return to the space-driven flow and find btn-participate.
      await expect(page).toHaveURL(/\/onboarding\/connections/, {
        timeout: 10000,
      });
      await click(page, { testId: "onboarding-skip" });
      await goto(page, spaceUrl);

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
