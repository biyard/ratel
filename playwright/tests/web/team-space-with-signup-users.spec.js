import { test, expect } from "@playwright/test";
import {
  click,
  fill,
  goto,
  getLocator,
  getEditor,
  waitPopup,
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
//   4.  NewUser: Sign up and participate (complete prerequisite poll)
//   5.  User2: Log in and participate (complete prerequisite poll)
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

  await goto(page, spaceUrl + "/dashboard");
  await click(page, { text: "Sign In" });
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
 * The page should already be on spaceUrl/dashboard as a logged-in user.
 */
async function participateAndCompletePoll(page, spaceUrl, pollOptionText) {
  await goto(page, spaceUrl + "/dashboard");
  await click(page, { text: "Participate" });
  await getLocator(page, { text: "Required Actions" });

  // Click the prerequisite poll
  await click(page, { text: "How should the team allocate the Q2 budget?" });
  await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

  // Answer the single choice question
  await click(page, { text: pollOptionText });
  await click(page, { text: "Submit" });
  await page.waitForLoadState("load");
}

// ─── Test suite ─────────────────────────────────────────────────────────────

test.describe.serial("Space with actions created by a team", () => {
  // Increase timeout for this complex multi-user test suite
  test.setTimeout(120000);

  let spaceUrl;
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
    await goto(page, "/");
    await click(page, { label: "User Profile" });
    await click(page, { text: "Create Team" });

    const nicknameInput = page.locator('[data-testid="team-nickname-input"]');
    await nicknameInput.fill(teamNickname);
    const usernameInput = page.locator('[data-testid="team-username-input"]');
    await usernameInput.fill(teamUsername);
    const descInput = page.locator('[data-testid="team-description-input"]');
    await descInput.fill("E2E test team for space actions");

    await click(page, { text: "Create" });
    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "load",
    });

    await click(page, { text: "Create" });
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    await fill(page, { placeholder: "Title" }, postTitle);
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(postContents);

    await click(page, { text: "Go to Space" });
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "load",
    });
    await getLocator(page, { text: "Dashboard" });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  });

  // ─── 2. Creator: Add actions ──────────────────────────────────────────────

  test("Create a discussion action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-discussion" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/discussions\//, {
      waitUntil: "load",
    });

    // Save the discussion URL for later use
    discussionUrl = new URL(page.url()).pathname;

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Team Discussion: Governance Framework",
    );
    await fill(
      page,
      { placeholder: "Enter category (optional)..." },
      "Governance",
    );

    const editor = await getEditor(page);
    await editor.fill(
      "This discussion was created by a team to explore governance frameworks and decision-making processes within the space.",
    );

    await click(page, { text: "Save" });
  });

  test("Create a poll action (prerequisite) in the space", async ({
    page,
  }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Team Poll: Budget Allocation",
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs.nth(0).fill("How should the team allocate the Q2 budget?");
    await textInputs.nth(1).fill("Increase marketing spend");
    await textInputs.nth(2).fill("Invest in R&D");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("Save for reserves");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    // Enable prerequisite setting
    await page.getByRole("tab", { name: "Settings" }).click();
    await page.waitForLoadState("load");

    const prerequisiteCard = page.locator("text=Prerequisite").locator("../..");
    await prerequisiteCard.locator("button").click();
    await page.waitForLoadState("load");
  });

  test("Create a quiz action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Team Quiz: Protocol Knowledge Check",
    );

    const editor = await getEditor(page);
    await editor.fill(
      "This quiz tests knowledge about the governance protocol. Created by the team for participant engagement.",
    );
    await click(page, { text: "Save" });

    await page.getByRole("tab", { name: "Quiz" }).click();
    await page.waitForLoadState("load");

    // Add first question (Single Choice)
    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs
      .nth(0)
      .fill("What is the primary purpose of governance in a DAO?");
    await textInputs.nth(1).fill("To centralize power");
    await textInputs.nth(2).fill("To enable collective decision-making");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("To maximize profits only");

    // Mark correct answer (option 2)
    const checkboxLabels = page.locator('label:has(input[type="checkbox"])');
    await checkboxLabels.nth(1).click();
    await page.waitForLoadState("load");

    // Add second question (Multiple Choice)
    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Multiple Choice" });

    await textInputs
      .nth(4)
      .fill("Which of the following are benefits of decentralized governance?");
    await textInputs.nth(5).fill("Transparency");
    await textInputs.nth(6).fill("Community participation");

    await page.getByRole("button", { name: "Add Option" }).nth(1).click();
    await page.waitForLoadState("load");
    await textInputs.nth(7).fill("Single point of failure");

    // Mark correct answers (options 1 and 2 for Q2)
    await checkboxLabels.nth(3).click();
    await checkboxLabels.nth(4).click();

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");
  });

  test("Create a follow action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-follow" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/follows\//, { waitUntil: "load" });
    await getLocator(page, { text: "General" });
  });

  // ─── 3. Creator: Publish space ────────────────────────────────────────────

  test("Publish the space publicly", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Publish" });
    await click(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
  });

  // ─── 4. NewUser: Sign up and participate ──────────────────────────────────

  test("NewUser: Sign up and participate in the space", async ({
    browser,
  }) => {
    const { context, page, displayName } = await signUpFromSpace(
      browser,
      spaceUrl,
    );
    try {
      await participateAndCompletePoll(page, spaceUrl, "Invest in R&D");

      // Navigate back to dashboard and verify participation
      await goto(page, spaceUrl + "/dashboard");
      const userProfile = page.locator("#space-user-profile");
      await expect(userProfile).toBeVisible();
      await expect(userProfile).toContainText(displayName);

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
      // Navigate to space dashboard and log in from there.
      await goto(page, spaceUrl + "/dashboard");
      await click(page, { text: "Sign In" });
      await waitPopup(page, { visible: true });
      await fill(page, { placeholder: "Enter your email address" }, user2.email);
      await click(page, { text: "Continue" });
      await fill(page, { placeholder: "Enter your password" }, user2.password);
      await click(page, { text: "Continue" });
      await waitPopup(page, { visible: false });

      // After login the space page re-renders with auth state.
      // Click Participate directly — no goto needed since page is already loaded.
      await click(page, { text: "Participate" });
      await getLocator(page, { text: "Required Actions" });

      // Complete the prerequisite poll
      await click(page, { text: "How should the team allocate the Q2 budget?" });
      await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });
      await click(page, { text: "Increase marketing spend" });
      await click(page, { text: "Submit" });
      await page.waitForLoadState("load");

      // Navigate back to dashboard to verify participation.
      // Use page.goto directly to avoid goto helper's WASM waitForResponse
      // which can hang when WASM is served from the browser memory cache.
      await page.goto(spaceUrl + "/dashboard");
      await page.waitForLoadState("domcontentloaded");
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null,
      );
      const userProfile = page.locator("#space-user-profile");
      await expect(userProfile).toBeVisible();

      // Save storage state for reuse
      user2StoragePath = `e2e-user2-${Date.now()}.json`;
      await context.storageState({ path: user2StoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 6. Creator: Start the space ──────────────────────────────────────────

  test("Creator: Start the space", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Start" });
    await click(page, { testId: "start-space-button" });
    await page.waitForLoadState("load");
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
      await goto(page, spaceUrl + "/actions");

      // Click the follow action card to navigate to it
      await click(page, { text: "Follow Users" });
      await page.waitForURL(/\/actions\/follows\//, { waitUntil: "load" });

      // Click "Follow" on the first non-creator user (the team creator)
      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible();
      await followBtn.click();
      await page.waitForLoadState("load");
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
      await goto(page, spaceUrl + "/actions");
      await click(page, { text: "Follow Users" });
      await page.waitForURL(/\/actions\/follows\//, { waitUntil: "load" });

      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible();
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
      await goto(page, spaceUrl + "/actions");
      await click(page, { text: "Team Quiz: Protocol Knowledge Check" });
      await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "load" });

      // Overview page → click Next to start
      await click(page, { testId: "quiz-read-next" });

      // Q1 (Single Choice): Select "To enable collective decision-making"
      await click(page, { text: "To enable collective decision-making" });
      // Auto-advances to Q2 after single-choice selection

      // Q2 (Multiple Choice): Select "Transparency" and "Community participation"
      await click(page, { text: "Transparency" });
      await click(page, { text: "Community participation" });

      // Submit quiz
      await click(page, { text: "Submit" });
      await page.waitForLoadState("load");
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
      await goto(page, spaceUrl + "/actions");
      await click(page, { text: "Team Quiz: Protocol Knowledge Check" });
      await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "load" });

      await click(page, { testId: "quiz-read-next" });

      // Q1: Select "To enable collective decision-making"
      await click(page, { text: "To enable collective decision-making" });

      // Q2: Select "Transparency" and "Community participation"
      await click(page, { text: "Transparency" });
      await click(page, { text: "Community participation" });

      await click(page, { text: "Submit" });
      await page.waitForLoadState("load");
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
    // The discussion URL was saved earlier when the creator created it.
    // Each user will post comments in round-robin fashion.

    const comments = [
      // Round 1 (comments 1-3)
      {
        user: "creator",
        text: "Welcome to the governance discussion. Let's explore how we can improve decision-making in our community.",
      },
      {
        user: "newUser",
        text: "Thanks for starting this. I think transparency should be our top priority in any governance model.",
      },
      {
        user: "user2",
        text: "Agreed on transparency. But we also need to consider efficiency. Too many votes slow things down.",
      },
      // Round 2 (comments 4-6)
      {
        user: "creator",
        text: "Good points from both of you. Let's consider a delegated voting system as a middle ground.",
      },
      {
        user: "newUser",
        text: "Delegated voting sounds promising. How do we prevent vote concentration among a few delegates?",
      },
      {
        user: "user2",
        text: "We could implement term limits for delegates and require periodic re-delegation.",
      },
      // Round 3 (comments 7-9)
      {
        user: "creator",
        text: "As moderator, I'd like to note that both proposals have merit. Let's think about implementation costs.",
      },
      {
        user: "newUser",
        text: "Implementation-wise, smart contracts could automate the delegation and term limit logic.",
      },
      {
        user: "user2",
        text: "We should also consider gas costs. A layer-2 solution might be necessary for frequent votes.",
      },
      // Round 4 (comments 10-12)
      {
        user: "creator",
        text: "Excellent technical considerations. Let me summarize the key proposals so far.",
      },
      {
        user: "newUser",
        text: "I'd also suggest we look at quadratic voting as an alternative to simple majority rules.",
      },
      {
        user: "user2",
        text: "Quadratic voting is interesting but complex for new users. We need good UX design around it.",
      },
      // Round 5 (comments 13-15)
      {
        user: "creator",
        text: "UX is critical. We should test any voting mechanism with a small group before full deployment.",
      },
      {
        user: "newUser",
        text: "I volunteer to be part of the testing group. The quiz we took earlier gave me good context.",
      },
      {
        user: "user2",
        text: "Count me in too. We could run a pilot governance vote on a non-critical decision first.",
      },
      // Round 6 (comments 16-18)
      {
        user: "creator",
        text: "Perfect. Let's plan the pilot for next month. I'll create a timeline proposal.",
      },
      {
        user: "newUser",
        text: "For the pilot, I suggest we vote on the community event theme. Low stakes, good learning opportunity.",
      },
      {
        user: "user2",
        text: "Great idea. We should document the entire process so other communities can learn from us.",
      },
      // Round 7 (comments 19-20)
      {
        user: "creator",
        text: "Agreed on documentation. I'll set up a shared doc. Thank you both for the productive discussion!",
      },
      {
        user: "newUser",
        text: "Thank you for moderating! Looking forward to the pilot governance vote next month.",
      },
    ];

    // Post creator comments (hi+user1@biyard.co — uses default storageState)
    const creatorComments = comments.filter((c) => c.user === "creator");
    for (const c of creatorComments) {
      await goto(page, discussionUrl);
      await fill(page, { placeholder: "Write a comment..." }, c.text);
      // Click the send button (icon-only button next to the input)
      const commentInput = page.getByPlaceholder("Write a comment...", {
        exact: true,
      });
      // Clear focus from input to enable the button, then find and click it
      const sendBtns = page.locator(
        "button:not([disabled]):has(svg)",
      );
      // The comment send button is adjacent to the input
      await commentInput.press("Tab");
      // Locate the send button by finding buttons near the comment input
      const sendBtn = sendBtns.first();
      await sendBtn.click();
      await page.waitForLoadState("load");
    }

    // Post newUser comments
    {
      const context = await browser.newContext({
        storageState: newUserStoragePath,
        viewport: { width: 1440, height: 950 },
        locale: "en-US",
      });
      const userPage = await context.newPage();

      try {
        const userComments = comments.filter((c) => c.user === "newUser");
        for (const c of userComments) {
          await goto(userPage, discussionUrl);
          await fill(
            userPage,
            { placeholder: "Write a comment..." },
            c.text,
          );
          const commentInput = userPage.getByPlaceholder(
            "Write a comment...",
            { exact: true },
          );
          const sendBtns = userPage.locator(
            "button:not([disabled]):has(svg)",
          );
          await commentInput.press("Tab");
          const sendBtn = sendBtns.first();
          await sendBtn.click();
          await userPage.waitForLoadState("load");
        }
      } finally {
        await context.close();
      }
    }

    // Post user2 comments
    {
      const context = await browser.newContext({
        storageState: user2StoragePath,
        viewport: { width: 1440, height: 950 },
        locale: "en-US",
      });
      const userPage = await context.newPage();

      try {
        const userComments = comments.filter((c) => c.user === "user2");
        for (const c of userComments) {
          await goto(userPage, discussionUrl);
          await fill(
            userPage,
            { placeholder: "Write a comment..." },
            c.text,
          );
          const commentInput = userPage.getByPlaceholder(
            "Write a comment...",
            { exact: true },
          );
          const sendBtns = userPage.locator(
            "button:not([disabled]):has(svg)",
          );
          await commentInput.press("Tab");
          const sendBtn = sendBtns.first();
          await sendBtn.click();
          await userPage.waitForLoadState("load");
        }
      } finally {
        await context.close();
      }
    }

    // Verify total comment count from creator's perspective
    await goto(page, discussionUrl);
    const commentHeader = page.getByText(/Comments \(\d+\)/);
    await expect(commentHeader).toBeVisible();
  });

  // ─── 10. Creator: Add final survey poll ───────────────────────────────────

  test("Creator: Add a final survey poll", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Final Survey: Space Experience",
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs
      .nth(0)
      .fill("How would you rate your overall experience in this space?");
    await textInputs.nth(1).fill("Excellent");
    await textInputs.nth(2).fill("Good");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("Needs improvement");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");
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
      await goto(page, spaceUrl + "/actions");
      await click(page, { text: "Final Survey: Space Experience" });
      await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

      // Answer the question
      await click(page, { text: "Excellent" });
      await click(page, { text: "Submit" });
      await page.waitForLoadState("load");
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
      await goto(page, spaceUrl + "/actions");
      await click(page, { text: "Final Survey: Space Experience" });
      await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

      // Answer the question
      await click(page, { text: "Good" });
      await click(page, { text: "Submit" });
      await page.waitForLoadState("load");
    } finally {
      await context.close();
    }
  });

  // ─── 12. Creator: Finish the space ────────────────────────────────────────

  test("Creator: Finish the space", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Finish" });
    await click(page, { testId: "end-space-button" });
    await page.waitForLoadState("load");
  });
});
