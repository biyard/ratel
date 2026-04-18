import { test, expect } from "@playwright/test";
import {
  click,
  clickNoNav,
  createAction,
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
  commitAutosave,
} from "../utils";

// This test covers the full space lifecycle with three users:
//
// Users:
//   - Creator (hi+user1@biyard.co): pre-authenticated via user.json
//   - NewUser: fresh signup (e2e_signup_{ts}@biyard.co)
//   - User2 (hi+user2@biyard.co): existing account
//
// Flow:
//   1.  Creator: Create team + post with space
//   2.  Creator: Add Discussion, Poll (prerequisite), Quiz, Follow actions
//   3.  Creator: Publish space publicly
//   4.  NewUser: Sign up via ArenaViewer, participate + complete prereq poll (overlay)
//   5.  User2: Log in via ArenaViewer, participate + complete prereq poll (overlay)
//   6.  Creator: Start the space
//   7.  Both participants: Complete follow action (follow the team)
//   8.  Both participants: Complete quiz action
//   9.  All three users: Discussion — 20 replies total (creator moderates)
//   10. Creator: Add a new final survey poll
//   11. Both participants: Complete final survey poll
//   12. Creator: Finish the space

// ─── User definitions ───────────────────────────────────────────────────────

const user2 = {
  email: "hi+user2@biyard.co",
  password: "admin!234",
};

// ─── Helpers ────────────────────────────────────────────────────────────────

/**
 * Set the action start date+time to now via the Settings tab.
 * Clicks today in the date picker, then sets the hour to the current
 * hour (or one hour earlier) so the action is immediately In Progress.
 * Assumes the Settings tab is already active.
 */
async function setStartDateToToday(page) {
  // Arena editor uses native datetime-local inputs for schedule. Fill both
  // start and end — the server-side save (UpdatePollRequest::Time) early-
  // returns unless both values are > 0, so filling start alone is a no-op.
  //
  // NOTE: the backend's `datetime_local_to_epoch_ms` parses the string with
  // `and_utc()`, i.e. treats the datetime-local value as UTC. JS local
  // getters (getFullYear/…/getHours) would produce a local-time string that
  // the backend would then re-interpret as UTC — on a non-UTC machine
  // (e.g. KST) this stores `started_at` hours in the future and the poll
  // becomes invisible to non-Creator roles. Use UTC getters so the string
  // matches the backend's UTC interpretation on any host timezone.
  const fmt = (date) => {
    const y = date.getUTCFullYear();
    const m = String(date.getUTCMonth() + 1).padStart(2, "0");
    const d = String(date.getUTCDate()).padStart(2, "0");
    const h = String(date.getUTCHours()).padStart(2, "0");
    const mm = String(date.getUTCMinutes()).padStart(2, "0");
    return `${y}-${m}-${d}T${h}:${mm}`;
  };
  const now = new Date();
  const endDate = new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000);

  const startInput = page.getByTestId("schedule-start");
  await expect(startInput).toBeVisible();
  await startInput.fill(fmt(now));
  await startInput.blur();
  await page.waitForLoadState("load");

  const endInput = page.getByTestId("schedule-end");
  await expect(endInput).toBeVisible();
  await endInput.fill(fmt(endDate));
  await endInput.blur();
  await page.waitForLoadState("load");
  // Small settle so the onblur server round-trip finishes before the caller
  // navigates away.
  await page.waitForTimeout(500);
}

/** Hide the floating action button that may overlap modal buttons. */
async function hideFab(page) {
  await page.evaluate(() => {
    const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
    if (fab) fab.style.display = "none";
  });
}

/**
 * Sign up a new user from the space page.
 * Returns { context, page, displayName } — caller must close context.
 */
async function signUpFromSpace(browser, spaceUrl) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, spaceUrl);
  // Pause card-float animation so Playwright clicks don't miss
  await page.addStyleTag({
    content:
      "*, *::before, *::after { animation-play-state: paused !important; }",
  });
  await clickNoNav(page, { testId: "btn-signin" });
  await waitPopup(page, { visible: true });
  await click(page, { text: "Create an account" });

  const signupEmail = `e2e_signup_${Date.now()}@biyard.co`;
  await fill(page, { placeholder: "Enter your email address" }, signupEmail);
  await click(page, { text: "Send" });
  await fill(page, { placeholder: "Enter the verification code" }, "000000");
  await click(page, { text: "Verify" });
  await expect(page.getByText("Send", { exact: true })).toBeHidden({
    timeout: 10000,
  });

  await fill(page, { placeholder: "Enter your password" }, "Test!234");
  await fill(page, { placeholder: "Re-enter your password" }, "Test!234");

  const uniqueId = Date.now().toString();
  const displayName = `E2E User ${uniqueId}`;
  await fill(page, { placeholder: "Enter your display name" }, displayName);
  await fill(page, { placeholder: "Enter your user name" }, `u${uniqueId}`);
  await click(page, {
    label: "[Required] I have read and accept the Terms of Service.",
  });
  await click(page, { text: "Finished Sign-up" });
  await waitPopup(page, { visible: false });

  return { context, page, displayName };
}

/**
 * Participate in the space and complete the prerequisite poll.
 *
 * Updated flow (arena prerequisite-card redesign):
 *   1. Navigate to the space root URL (ArenaViewer).
 *   2. Click "Participate" — this space has no panel requirements so
 *      the participate call fires directly (no consent modal).
 *   3. After participation the user becomes a Candidate and sees the
 *      PrerequisiteCard with the checklist of required actions.
 *   4. Click the prerequisite poll item — opens a full-screen overlay.
 *   5. Select a poll option inside the overlay, submit, and confirm.
 *   6. After the overlay closes the WaitingCard appears (all done).
 */
async function participateAndCompletePoll(page, _spaceUrl, pollOptionText) {
  // Verify credential if the button exists (bypass mode — just click it)
  const verifyBtn = page.getByTestId("btn-verify");
  if (await verifyBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
    await verifyBtn.click({ force: true });
    await page.waitForLoadState();
  }

  // Pause card-float animation
  await page.addStyleTag({
    content:
      "*, *::before, *::after { animation-play-state: paused !important; }",
  });

  // Click participate button on the ArenaViewer
  await clickNoNav(page, { testId: "btn-participate" });

  // PrerequisiteCard appears (no consent modal since no panels configured)
  await expect(page.getByTestId("card-prerequisite")).toBeVisible({
    timeout: 30000,
  });

  // Click the prerequisite poll item — opens the full-screen poll overlay
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

  // Select the specific poll option inside the overlay
  await overlay.getByText(pollOptionText, { exact: true }).click();

  // Submit the poll using testId (avoids ambiguity with confirm dialog)
  await clickNoNav(page, { testId: "poll-submit" });

  // Confirm dialog appears — click confirm. Poll submit closes the overlay
  // in place (no navigation), so we must not wait for a load event.
  await clickNoNav(page, { testId: "poll-confirm-submit" });

  // Wait for overlay to close (server call completes + overlay signal cleared)
  await expect(page.getByTestId("poll-arena-overlay")).toBeHidden({
    timeout: 30000,
  });

  // After completing all prerequisites, user should see the WaitingCard
  await expect(page.getByTestId("card-waiting")).toBeVisible({
    timeout: 30000,
  });
}

// ─── Test suite ─────────────────────────────────────────────────────────────

test.describe.serial("Space with actions created by a team", () => {
  // Increase timeout for this complex multi-user test suite
  test.setTimeout(120000);

  let spaceUrl;
  // `discussionUrl` is assigned inside the "Create a discussion action"
  // test and kept for later inspection by future step additions. The
  // read-side is intentionally absent for now; the declaration is required
  // so the assignment doesn't throw a ReferenceError in strict mode.
  // eslint-disable-next-line no-unused-vars
  let discussionUrl;

  // We'll save newUser and user2 storage states for reuse across tests
  let newUserStoragePath;
  let user2StoragePath;

  const teamNickname = `Test Team`;
  const teamUsername = `e2e_team_${Date.now()}`;
  const postTitle = "Team Post for Space Actions E2E Test";
  const postContents =
    "This is a test post created by a team through Playwright E2E testing. " +
    "It verifies the full flow of team creation, post creation, space actions, " +
    "and publishing. The content is intentionally long enough to meet the minimum " +
    "character requirement for post content validation.";

  // ─── 1. Creator: Create team + post with space ────────────────────────────

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

    // Space creation remains REST — the post-edit "Go to Space" affordance was
    // removed by the post-edit renewal, so this suite keeps its focus on the
    // governance/actions flow rather than the orthogonal space-creation UI.
    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(spaceRes.ok(), `create space: ${await spaceRes.text()}`).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;

    spaceUrl = `/spaces/${spaceId}`;

    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
  });

  // ─── 2. Creator: Add actions ──────────────────────────────────────────────

  test("Create a discussion action in the space", async ({ page }) => {
    await createAction(
      page,
      spaceUrl,
      "discuss",
      /\/actions\/discussions\/[^/]+\/edit/,
    );

    // Save the discussion URL for later use
    discussionUrl = new URL(page.url()).pathname;

    // Arena-style editor: inline autosave, no category field, no Save button.
    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Team Discussion: Governance Framework",
    );

    const editor = await getEditor(page);
    await editor.fill(
      "This discussion was created by a team to explore governance frameworks and decision-making processes within the space.",
    );

    // Blur the editor so the autosave debounce commits.
    await page.keyboard.press("Tab");
  });

  test("Create a poll action (prerequisite) in the space", async ({ page }) => {
    await createAction(page, spaceUrl, "poll", /\/actions\/polls\//);

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Team Poll: Budget Allocation",
    );
    await commitAutosave(page);

    // Arena poll editor: exactly two option inputs, no "Add Option" button.
    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "How should the team allocate the Q2 budget?",
      options: ["Increase marketing spend", "Invest in R&D"],
    });

    // Prerequisite is toggled via the ConfigCard tile (no more Settings tab).
    await togglePrerequisite(page);
  });

  test("Create a quiz action in the space", async ({ page }) => {
    await createAction(page, spaceUrl, "quiz", /\/actions\/quizzes\//);

    // Arena-style quiz creator page: no tabs, no Save button. ContentCard +
    // QuestionsCard + ConfigCard render inline with per-field autosave.

    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Team Quiz: Protocol Knowledge Check",
    );

    const editor = await getEditor(page);
    await editor.fill(
      "This quiz tests knowledge about the governance protocol. Created by the team for participant engagement.",
    );
    await commitAutosave(page);

    // Single question with per-field blur so each onblur autosave
    // completes before the next input is touched.
    await click(page, { testId: "quiz-question-add" });
    const q0 = page.getByTestId("quiz-question-0");
    const q0Inputs = q0.locator("input.input");
    const fills = [
      "What is the primary purpose of governance in a DAO?",
      "To centralize power",
      "To enable collective decision-making",
    ];
    for (let i = 0; i < fills.length; i += 1) {
      await q0Inputs.nth(i).fill(fills[i]);
      await q0Inputs.nth(i).press("Tab");
      await page.waitForLoadState("load");
      await page.waitForTimeout(200);
    }
  });

  test("Create a follow action in the space", async ({ page }) => {
    await createAction(page, spaceUrl, "follow", /\/actions\/follows\//);
    // Arena follow creator: verify the ConfigCard renders (TargetsCard +
    // ConfigCard are inline, no more General tab).
    await getLocator(page, { testId: "page-card-config" });
  });

  // ─── 3. Creator: Publish space ────────────────────────────────────────────

  test("Publish the space publicly", async ({ page }) => {
    // Arena flow: ArenaTopbar exposes the Publish action as an admin-only
    // HUD button (btn-publish). The legacy "/dashboard" Publish button has
    // been superseded — Publish now opens SpaceVisibilityModal in-place.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-publish" });
    await click(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
  });

  // ─── 4. NewUser: Sign up and participate ──────────────────────────────────

  test("NewUser: Sign up and participate in the space", async ({ browser }) => {
    const { context, page } = await signUpFromSpace(browser, spaceUrl);
    try {
      await participateAndCompletePoll(page, spaceUrl, "Invest in R&D");

      // Save storage state for reuse
      newUserStoragePath = `e2e-newuser-${Date.now()}.json`;
      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 5. User2: Log in and participate ─────────────────────────────────────

  test("User2: Log in and participate in the space", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Navigate to space and log in via the ArenaViewer SigninCard.
      await goto(page, spaceUrl);
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });
      await clickNoNav(page, { testId: "btn-signin" });
      await waitPopup(page, { visible: true });
      await fill(
        page,
        { placeholder: "Enter your email address" },
        user2.email,
      );
      await click(page, { text: "Continue" });
      await fill(page, { placeholder: "Enter your password" }, user2.password);
      await click(page, { text: "Continue" });
      await waitPopup(page, { visible: false });

      // Use the shared helper for robust participation.
      await participateAndCompletePoll(
        page,
        spaceUrl,
        "Increase marketing spend",
      );

      // Save storage state for reuse
      user2StoragePath = `e2e-user2-${Date.now()}.json`;
      await context.storageState({ path: user2StoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 6. Creator: Start the space ──────────────────────────────────────────

  test("Creator: Start the space", async ({ page }) => {
    // Arena flow: after publish, ArenaTopbar swaps btn-publish for btn-start
    // (admin-only). Clicking it opens SpaceStartModal; start-space-button
    // inside the modal fires the actual state transition.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-start" });
    await click(page, { testId: "start-space-button" });
  });

  // ─── 7. Both participants: Follow action ──────────────────────────────────

  test("NewUser: Complete follow action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      // Click "Follow" on the follow card inline in the carousel
      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible({ timeout: 10000 });
      await followBtn.click();

      // After completing follow, the card animates into the archive.
      // Prerequisite poll was already completed (count starts at 1),
      // so follow completion brings it to "2".
      const archiveCount = page.locator(".archive-btn__count");
      await expect(archiveCount).toBeVisible({ timeout: 15000 });
      await expect(archiveCount).toHaveText("2", { timeout: 10000 });
    } finally {
      await context.close();
    }
  });

  test("User2: Complete follow action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Navigate to space root — ActionDashboard shows follow card inline
      await goto(page, spaceUrl);
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      // Click "Follow" on the follow card in the carousel
      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible({ timeout: 10000 });
      await followBtn.click();
      await page.waitForLoadState("load");
    } finally {
      await context.close();
    }
  });

  // ─── 8. Both participants: Quiz action ────────────────────────────────────

  test("NewUser: Complete quiz action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      // Find and click the quiz card in the carousel to open the overlay
      const quizCard = page.locator('[data-type="quiz"]').first();
      await expect(quizCard).toBeVisible({ timeout: 10000 });
      await quizCard.click();

      // Quiz arena overlay appears
      const overlay = page.getByTestId("quiz-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 10000 });

      // Overview page — click Begin to start
      await expect(page.getByTestId("quiz-arena-overview")).toBeVisible({
        timeout: 10000,
      });
      await clickNoNav(page, { testId: "quiz-arena-begin" });

      // Wait for quiz questions area to be visible
      await expect(page.getByTestId("quiz-arena-questions")).toBeVisible({
        timeout: 10000,
      });

      // Q1 (Single Choice): Select "To enable collective decision-making"
      await expect(
        overlay.getByText("To enable collective decision-making", {
          exact: true,
        }),
      ).toBeVisible({ timeout: 10000 });
      await overlay
        .getByText("To enable collective decision-making", { exact: true })
        .click();

      // Quiz was simplified to a single question — submit directly.
      await clickNoNav(page, { testId: "quiz-arena-submit" });

      // Wait for overlay to close (submission completes + overlay signal cleared)
      await expect(page.getByTestId("quiz-arena-overlay")).toBeHidden({
        timeout: 30000,
      });

      // After completing quiz, archive badge count should show "3"
      // (prerequisite poll + follow + quiz).
      const archiveCount = page.locator(".archive-btn__count");
      await expect(archiveCount).toBeVisible({ timeout: 15000 });
      await expect(archiveCount).toHaveText("3", { timeout: 10000 });
    } finally {
      await context.close();
    }
  });

  test("User2: Complete quiz action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await page.addStyleTag({
        content:
          "*, *::before, *::after { animation-play-state: paused !important; }",
      });

      // Find and click the quiz card in the carousel to open the overlay
      const quizCard = page.locator('[data-type="quiz"]').first();
      await expect(quizCard).toBeVisible({ timeout: 10000 });
      await quizCard.click();

      // Quiz arena overlay appears
      const overlay = page.getByTestId("quiz-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 10000 });

      // Overview page — click Begin to start
      await expect(page.getByTestId("quiz-arena-overview")).toBeVisible({
        timeout: 10000,
      });
      await clickNoNav(page, { testId: "quiz-arena-begin" });

      // Wait for quiz questions area to be visible
      await expect(page.getByTestId("quiz-arena-questions")).toBeVisible({
        timeout: 10000,
      });

      // Q1 (Single Choice): Select "To enable collective decision-making"
      await expect(
        overlay.getByText("To enable collective decision-making", {
          exact: true,
        }),
      ).toBeVisible({ timeout: 10000 });
      await overlay
        .getByText("To enable collective decision-making", { exact: true })
        .click();

      // Quiz was simplified to a single question — submit directly.
      await clickNoNav(page, { testId: "quiz-arena-submit" });

      // Wait for overlay to close
      await expect(page.getByTestId("quiz-arena-overlay")).toBeHidden({
        timeout: 30000,
      });
    } finally {
      await context.close();
    }
  });

  // ─── 9. Discussion: 20 replies across three users ─────────────────────────
  // Creator moderates; NewUser and User2 discuss the topic.

  test("Discussion: All three users post 20 comments", async ({
    page,
    browser,
  }) => {
    // 20 comments across 3 browser contexts — full page navigation +
    // WASM hydration per comment easily blows through the 120s suite
    // default, so give this test its own budget.
    test.setTimeout(240000);
    // Discussion now opens as an overlay from the ActionDashboard carousel.
    // Each user navigates to spaceUrl, clicks the discussion card, types
    // in the comment textarea ("Share your thoughts..."), and clicks "Post".

    const comments = [
      {
        user: "creator",
        text: "Welcome to the governance discussion. Let's explore how we can improve decision-making.",
      },
      {
        user: "newUser",
        text: "I think transparency should be our top priority in any governance model.",
      },
      {
        user: "user2",
        text: "Agreed on transparency. But we also need to consider efficiency.",
      },
      {
        user: "creator",
        text: "Good points. Let's consider a delegated voting system as a middle ground.",
      },
      {
        user: "newUser",
        text: "Delegated voting sounds promising. How do we prevent vote concentration?",
      },
      {
        user: "user2",
        text: "We could implement term limits for delegates and require periodic re-delegation.",
      },
      {
        user: "creator",
        text: "Both proposals have merit. Let's think about implementation costs.",
      },
      {
        user: "newUser",
        text: "Smart contracts could automate the delegation and term limit logic.",
      },
      {
        user: "user2",
        text: "We should also consider gas costs. A layer-2 solution might be necessary.",
      },
      {
        user: "creator",
        text: "Excellent technical considerations. Let me summarize the key proposals.",
      },
      {
        user: "newUser",
        text: "I'd suggest we look at quadratic voting as an alternative.",
      },
      {
        user: "user2",
        text: "Quadratic voting is interesting but complex. We need good UX design.",
      },
      {
        user: "creator",
        text: "UX is critical. We should test with a small group before full deployment.",
      },
      { user: "newUser", text: "I volunteer to be part of the testing group." },
      {
        user: "user2",
        text: "Count me in. We could run a pilot governance vote on a non-critical decision.",
      },
      {
        user: "creator",
        text: "Let's plan the pilot for next month. I'll create a timeline proposal.",
      },
      {
        user: "newUser",
        text: "For the pilot, I suggest we vote on the community event theme.",
      },
      {
        user: "user2",
        text: "Great idea. We should document the entire process.",
      },
      {
        user: "creator",
        text: "Agreed on documentation. Thank you both for the productive discussion!",
      },
      {
        user: "newUser",
        text: "Thank you for moderating! Looking forward to the pilot vote.",
      },
    ];

    // Helper: open discussion overlay from the ActionDashboard carousel.
    // Uses gotoFresh because every comment cycle reopens this, and
    // Dioxus's arena gets corrupted by the prior overlay close — a plain
    // `goto` would no-op the discussion-card click on subsequent
    // iterations.
    //
    // The card is selected by `[data-type="discuss"]` (not a testid), so
    // the click() helper's per-testid hydration precheck doesn't run for
    // it. Under CI's slower runner we still occasionally see the click
    // fire before Dioxus binds the onclick handler. Wrap the click in a
    // retry loop that re-clicks on a fresh page until the overlay opens
    // — bounded to 3 attempts so a real broken overlay still fails fast.
    async function openDiscussionOverlay(pg) {
      const overlay = pg.getByTestId("discussion-arena-overlay");
      let lastError;
      for (let attempt = 0; attempt < 3; attempt++) {
        await gotoFresh(pg, spaceUrl);
        const discCard = pg.locator('[data-type="discuss"]').first();
        await expect(discCard).toBeVisible({ timeout: 10000 });
        await pg.waitForTimeout(500);
        await discCard.click();
        try {
          await expect(overlay).toBeVisible({ timeout: 5000 });
          return;
        } catch (err) {
          lastError = err;
          // Overlay didn't open — fall through and retry from a fresh
          // page. Common under CI when the arena is mid-corruption.
        }
      }
      throw lastError;
    }

    // Helper: post a comment in the discussion overlay textarea. The
    // submit handler clears the textarea and calls comments_loader.restart
    // on success, so the overlay stays open and shows the latest server
    // state on every submit — good enough for cross-user interleaving.
    async function postComment(pg, text) {
      const textarea = pg.locator(".comment-input__textarea").first();
      await expect(textarea).toBeVisible({ timeout: 10000 });
      await textarea.fill(text);
      await pg.locator(".comment-input__submit").first().click();
      // Wait for the comment to render in the list (text is unique per entry)
      await expect(
        pg.locator(".comment-item__text", { hasText: text }).first(),
      ).toBeVisible({
        timeout: 10000,
      });
    }

    // Open 3 concurrent browser contexts — one per user — and leave the
    // discussion overlay open in each so we can interleave comments in
    // the order declared by `comments` (simulating a real-time thread)
    // without paying SSR/WASM-hydration costs per message.
    const newUserContext = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const user2Context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const newUserPage = await newUserContext.newPage();
    const user2Page = await user2Context.newPage();

    try {
      // Open the discussion overlay in all three tabs up front.
      await Promise.all([
        openDiscussionOverlay(page),
        openDiscussionOverlay(newUserPage),
        openDiscussionOverlay(user2Page),
      ]);

      const pageByUser = {
        creator: page,
        newUser: newUserPage,
        user2: user2Page,
      };

      // Walk the comment script in order. After each submit, that user's
      // overlay refetches comments (add_comment → comments_loader.restart),
      // so the next time they post they see everyone else's prior messages.
      for (const c of comments) {
        const pg = pageByUser[c.user];
        await postComment(pg, c.text);
      }

      // Close each participant's overlay.
      await clickNoNav(newUserPage, { testId: "discussion-arena-back" });
      await expect(
        newUserPage.getByTestId("discussion-arena-overlay"),
      ).toBeHidden({ timeout: 10000 });

      await clickNoNav(user2Page, { testId: "discussion-arena-back" });
      await expect(
        user2Page.getByTestId("discussion-arena-overlay"),
      ).toBeHidden({ timeout: 10000 });

      await clickNoNav(page, { testId: "discussion-arena-back" });
      await expect(page.getByTestId("discussion-arena-overlay")).toBeHidden({
        timeout: 10000,
      });
    } finally {
      await newUserContext.close();
      await user2Context.close();
    }

    // Verify comment count from creator's perspective
    await openDiscussionOverlay(page);
    const countBadge = page.locator(".comments-panel__count");
    await expect(countBadge).toBeVisible();
  });

  // ─── 10. Creator: Add final survey poll ───────────────────────────────────

  test("Creator: Add a final survey poll", async ({ page }) => {
    await goto(page, spaceUrl);
    await hideFab(page);
    // Arena: admin's add-action card opens the TypePicker directly; the
    // type pick immediately creates the action and navigates.
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-poll" });

    await page.waitForURL(/\/actions\/polls\//, {
      waitUntil: "load",
      timeout: 60000,
    });

    // After the space is published, new actions default to started_at =
    // now + 1 hour, so is_action_locked is false and the creator lands
    // directly on PollCreatorPage — no settings toggle needed.

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Final Survey: Space Experience",
    );
    await commitAutosave(page);

    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "How would you rate your overall experience in this space?",
      options: ["Excellent", "Good"],
    });

    // Set start date to today via the ConfigCard's Schedule section
    // (the old "Settings" tab is gone in the arena editor).
    await setStartDateToToday(page);
  });

  // ─── 11. Both participants: Complete final survey ─────────────────────────

  test("NewUser: Complete final survey poll", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Navigate to space root — ActionDashboard shows poll card in carousel
      await goto(page, spaceUrl);

      // Clicking the poll card now opens the full-screen poll overlay
      // in place (no navigation).
      const pollCard = page.locator('[data-type="poll"]').first();
      await expect(pollCard).toBeVisible({ timeout: 10000 });
      await pollCard.click();

      const overlay = page.getByTestId("poll-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 15000 });

      await clickNoNav(page, { testId: "poll-arena-begin" });

      await expect(
        overlay.getByText("Excellent", { exact: true }),
      ).toBeVisible({ timeout: 10000 });
      await overlay.getByText("Excellent", { exact: true }).click();

      await clickNoNav(page, { testId: "poll-submit" });
      await clickNoNav(page, { testId: "poll-confirm-submit" });

      await expect(overlay).toBeHidden({ timeout: 30000 });
    } finally {
      await context.close();
    }
  });

  test("User2: Complete final survey poll", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      // Navigate to space root — ActionDashboard shows poll card in carousel
      await goto(page, spaceUrl);

      // Clicking the poll card now opens the full-screen poll overlay
      // in place (no navigation).
      const pollCard = page.locator('[data-type="poll"]').first();
      await expect(pollCard).toBeVisible({ timeout: 10000 });
      await pollCard.click();

      const overlay = page.getByTestId("poll-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 15000 });

      await clickNoNav(page, { testId: "poll-arena-begin" });

      await expect(overlay.getByText("Good", { exact: true })).toBeVisible({
        timeout: 10000,
      });
      await overlay.getByText("Good", { exact: true }).click();

      await clickNoNav(page, { testId: "poll-submit" });
      await clickNoNav(page, { testId: "poll-confirm-submit" });

      await expect(overlay).toBeHidden({ timeout: 30000 });
    } finally {
      await context.close();
    }
  });

  // ─── 12. Creator: Finish the space ────────────────────────────────────────

  test("Creator: Finish the space", async ({ page }) => {
    // Arena flow: after space is Ongoing, ArenaTopbar swaps btn-start for
    // btn-finish (admin-only). The "Finish" label lives only in the
    // hover tooltip (opacity:0 until :hover) so we cannot click by text —
    // use the stable testid instead. Clicking opens SpaceEndModal;
    // end-space-button inside the modal fires the actual transition.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-finish" });
    await click(page, { testId: "end-space-button" });
  });
});
