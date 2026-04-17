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
} from "../utils";

// Full space lifecycle E2E test:
//
// 1.  Creator: Create team → post → space
// 2.  Creator: Create poll (prerequisite) + set reward
// 3.  Creator: Create poll (normal) + set reward
// 4.  Creator: Create discussion + set reward
// 5.  Creator: Create quiz + set reward
// 6.  Creator: Create follow + set reward
// 7.  Creator: Enable anonymous participation + join anytime
// 8.  Creator: Publish space
// 9.  User1: Sign up → participate (only prereq visible) → complete prereq (before start)
// 10. Creator: Start space
// 11. User1: Complete each action (follow, quiz, poll, discussion) (after start)
// 12. User2: Sign up → participate (only prereq visible) → complete prereq (after start)
// 13. User2: Complete each action (follow, quiz, poll, discussion) (after start)
// 14. Creator: Finish space

const user2 = {
  email: "hi+user2@biyard.co",
  password: "admin!234",
};

// ─── Helpers ────────────────────────────────────────────────────────────────

async function hideFab(page) {
  await page.evaluate(() => {
    const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
    if (fab) fab.style.display = "none";
  });
}

async function pauseAnimations(page) {
  await page.addStyleTag({
    content:
      "*, *::before, *::after { animation-play-state: paused !important; }",
  });
}

async function enableReward(page, credits) {
  await page.getByRole("tab", { name: "Settings" }).click();
  await page.waitForLoadState("load");

  const rewardToggle = page.locator(
    '[data-testid="reward-setting-toggle"] button'
  );
  if ((await rewardToggle.count()) > 0) {
    await rewardToggle.click();
    await page.waitForLoadState("load");

    const creditInput = page.locator('[data-testid="reward-credit-input"]');
    await creditInput.fill(String(credits));
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");
  }
}

async function enableRewardOnSettingTab(page, credits) {
  await page.getByRole("tab", { name: "Setting" }).click();
  await page.waitForLoadState("load");

  const rewardToggle = page.locator(
    '[data-testid="reward-setting-toggle"] button'
  );
  if ((await rewardToggle.count()) > 0) {
    await rewardToggle.click();
    await page.waitForLoadState("load");

    const creditInput = page.locator('[data-testid="reward-credit-input"]');
    await creditInput.fill(String(credits));
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");
  }
}

async function signUpFromSpace(browser, spaceUrl) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, spaceUrl);
  await pauseAnimations(page);
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

async function participateAndCompletePoll(page, pollOptionText) {
  const verifyBtn = page.getByTestId("btn-verify");
  if (await verifyBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
    await verifyBtn.click({ force: true });
    await page.waitForLoadState();
  }

  await pauseAnimations(page);

  // Click participate
  await clickNoNav(page, { testId: "btn-participate" });

  // PrerequisiteCard appears
  await expect(page.getByTestId("card-prerequisite")).toBeVisible({
    timeout: 30000,
  });

  // Verify only prereq poll is visible, other actions should NOT be visible
  // (discussion, quiz, follow cards should be hidden before prereq completion)
  const prereqCard = page.getByTestId("card-prerequisite");
  const prereqItems = prereqCard.locator(".prereq-item");
  await expect(prereqItems).toHaveCount(1, { timeout: 10000 });

  // Click prereq poll item → opens poll overlay
  await prereqItems.first().click();

  const overlay = page.getByTestId("poll-arena-overlay");
  await expect(overlay).toBeVisible();

  await clickNoNav(page, { testId: "poll-arena-begin" });

  await expect(overlay.locator(".option-single").first()).toBeVisible({
    timeout: 30000,
  });

  await overlay.getByText(pollOptionText, { exact: true }).click();

  await clickNoNav(page, { testId: "poll-submit" });
  await clickNoNav(page, { testId: "poll-confirm-submit" });

  await expect(page.getByTestId("poll-arena-overlay")).toBeHidden({
    timeout: 30000,
  });

  // After prereq done, user should see the WaitingCard (space not started)
  // or ActionDashboard (space started)
}

async function loginUser2FromSpace(browser, spaceUrl) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, spaceUrl);
  await pauseAnimations(page);
  await clickNoNav(page, { testId: "btn-signin" });
  await waitPopup(page, { visible: true });
  await fill(page, { placeholder: "Enter your email address" }, user2.email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, user2.password);
  await click(page, { text: "Continue" });
  await waitPopup(page, { visible: false });

  return { context, page };
}

// ─── Test suite ─────────────────────────────────────────────────────────────

test.describe.serial("Full space lifecycle with rewards", () => {
  test.setTimeout(120000);

  let spaceUrl;
  let newUserStoragePath;
  let user2StoragePath;

  const teamNickname = "Lifecycle Team";
  const teamUsername = `e2e_lc_${Date.now()}`;
  const postTitle = "Full Lifecycle E2E Test Post";
  const postContents =
    "This is a comprehensive end-to-end test post for verifying the full " +
    "space lifecycle including team creation, all action types with rewards, " +
    "prerequisite checks, anonymous participation, join anytime settings, " +
    "multi-user participation, and space finish flow.";

  // ─── 1. Creator: Create team + post + space ──────────────────────────────

  test("Create team, post, and space", async ({ page }) => {
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "E2E test team for full lifecycle",
    });

    const postId = await createTeamPostFromHome(
      page,
      teamUsername,
      postTitle,
      postContents
    );

    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(
      spaceRes.ok(),
      `create space: ${await spaceRes.text()}`
    ).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;

    spaceUrl = `/spaces/${spaceId}`;

    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
  });

  // ─── 2. Create poll (prerequisite) + reward ──────────────────────────────

  test("Create prerequisite poll with reward", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Prerequisite Poll: Interest Check"
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs.nth(1).fill("What topic interests you most?");
    await textInputs.nth(2).fill("Technology");
    await textInputs.nth(3).fill("Science");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(4).fill("Arts");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    // Enable prerequisite + reward
    await page.getByRole("tab", { name: "Settings" }).click();
    await page.waitForLoadState("load");

    const prerequisiteSwitch = page.locator(
      '[data-testid="prerequisite-setting"] button'
    );
    await prerequisiteSwitch.click();
    await page.waitForLoadState("load");

    // Set reward
    const rewardToggle = page.locator(
      '[data-testid="reward-setting-toggle"] button'
    );
    if ((await rewardToggle.count()) > 0) {
      await rewardToggle.click();
      await page.waitForLoadState("load");

      const creditInput = page.locator('[data-testid="reward-credit-input"]');
      await creditInput.fill("1");
      await page.keyboard.press("Tab");
      await page.waitForLoadState("load");
    }
  });

  // ─── 3. Create poll (normal) + reward ────────────────────────────────────

  test("Create normal poll with reward", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-poll" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/polls\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Survey: Feature Priority"
    );
    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs.nth(1).fill("Which feature should we build next?");
    await textInputs.nth(2).fill("Mobile app");
    await textInputs.nth(3).fill("API improvements");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(4).fill("Documentation");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await enableReward(page, 2);
  });

  // ─── 4. Create discussion + reward ───────────────────────────────────────

  test("Create discussion with reward", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-discussion" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/discussions\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Discussion: Roadmap Planning"
    );
    await fill(
      page,
      { placeholder: "Enter category (optional)..." },
      "Planning"
    );

    const editor = await getEditor(page);
    await editor.fill(
      "Let's discuss the upcoming roadmap priorities and share ideas for the next quarter."
    );

    await click(page, { text: "Save" });

    await enableReward(page, 2);
  });

  // ─── 5. Create quiz + reward ─────────────────────────────────────────────

  test("Create quiz with reward", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "load" });

    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Quiz: Platform Knowledge"
    );

    const editor = await getEditor(page);
    await editor.fill("Test your knowledge about our platform fundamentals.");
    await click(page, { text: "Save" });

    // Add quiz question
    await page.getByRole("tab", { name: "Quiz" }).click();
    await page.waitForLoadState("load");

    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Single Choice" });

    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs.nth(0).fill("What is the main purpose of spaces?");
    await textInputs.nth(1).fill("Collective decision-making");
    await textInputs.nth(2).fill("Social media posting");

    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("load");
    await textInputs.nth(3).fill("File storage");

    // Mark correct answer (option 1)
    const checkboxLabels = page.locator('label:has(input[type="checkbox"])');
    await checkboxLabels.nth(0).click();
    await page.waitForLoadState("load");

    await page.keyboard.press("Tab");
    await page.waitForLoadState("load");

    await enableRewardOnSettingTab(page, 2);
  });

  // ─── 6. Create follow + reward ───────────────────────────────────────────

  test("Create follow with reward", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");
    await click(page, { text: "Select Action Type" });
    await click(page, { testId: "action-type-follow" });
    await hideFab(page);
    await click(page, { text: "Create" });

    await page.waitForURL(/\/actions\/follows\//, { waitUntil: "load" });
    await getLocator(page, { text: "General" });

    await enableReward(page, 2);
  });

  // ─── 7. Enable anonymous + join anytime via UI ───────────────────────────

  test("Enable anonymous participation and join anytime", async ({ page }) => {
    await goto(page, spaceUrl + "/apps/general");

    // Toggle "Anonymous Participation" switch
    const anonCard = page
      .locator("text=Anonymous Participation")
      .locator("../..");
    const anonSwitch = anonCard.locator("button").first();
    await anonSwitch.click();
    await page.waitForLoadState("load");

    // Toggle "Join Anytime" switch
    const joinCard = page.locator("text=Join Anytime").locator("../..");
    const joinSwitch = joinCard.locator("button").first();
    await joinSwitch.click();
    await page.waitForLoadState("load");
  });

  // ─── 8. Publish space ────────────────────────────────────────────────────

  test("Publish the space publicly", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Publish" });
    await click(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
  });

  // ─── 9. User1: Sign up + prereq BEFORE start ────────────────────────────

  test("User1: Sign up and complete prerequisite (before start)", async ({
    browser,
  }) => {
    const { context, page } = await signUpFromSpace(browser, spaceUrl);
    try {
      await participateAndCompletePoll(page, "Technology");

      // Before start, user should see WaitingCard (space not started yet)
      await expect(page.getByTestId("card-waiting")).toBeVisible({
        timeout: 30000,
      });

      newUserStoragePath = `e2e-lifecycle-user1-${Date.now()}.json`;
      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 10. Creator: Start space ────────────────────────────────────────────

  test("Start the space", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");
    await click(page, { text: "Start" });
    await click(page, { testId: "start-space-button" });
    await page.waitForLoadState("load");
  });

  // ─── 11. User1: Complete each action (after start) ──────────────────────

  test("User1: Complete follow action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible({ timeout: 10000 });
      await followBtn.click();

      const archiveCount = page.locator(".archive-btn__count");
      await expect(archiveCount).toBeVisible({ timeout: 15000 });

      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  test("User1: Complete quiz action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const quizCard = page.locator('[data-type="quiz"]').first();
      await expect(quizCard).toBeVisible({ timeout: 10000 });
      await quizCard.click();

      const overlay = page.getByTestId("quiz-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 10000 });

      await expect(page.getByTestId("quiz-arena-overview")).toBeVisible({
        timeout: 10000,
      });
      await clickNoNav(page, { testId: "quiz-arena-begin" });

      await expect(page.getByTestId("quiz-arena-questions")).toBeVisible({
        timeout: 10000,
      });

      await expect(
        overlay.getByText("Collective decision-making", { exact: true })
      ).toBeVisible({ timeout: 10000 });
      await overlay
        .getByText("Collective decision-making", { exact: true })
        .click();

      await clickNoNav(page, { testId: "quiz-arena-submit" });

      await expect(page.getByTestId("quiz-arena-overlay")).toBeHidden({
        timeout: 30000,
      });

      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  test("User1: Complete normal poll", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const pollCard = page.locator('[data-type="poll"]').first();
      await expect(pollCard).toBeVisible({ timeout: 10000 });
      await pollCard.click();

      const overlay = page.getByTestId("poll-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 15000 });

      await clickNoNav(page, { testId: "poll-arena-begin" });

      await expect(
        overlay.getByText("Mobile app", { exact: true })
      ).toBeVisible({ timeout: 10000 });
      await overlay.getByText("Mobile app", { exact: true }).click();

      await clickNoNav(page, { testId: "poll-submit" });
      await clickNoNav(page, { testId: "poll-confirm-submit" });

      await expect(overlay).toBeHidden({ timeout: 30000 });

      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  test("User1: Comment on discussion", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const discCard = page.locator('[data-type="discuss"]').first();
      await expect(discCard).toBeVisible({ timeout: 10000 });
      await page.waitForTimeout(500);
      await discCard.click();

      await expect(page.getByTestId("discussion-arena-overlay")).toBeVisible({
        timeout: 10000,
      });

      const textarea = page.locator(".comment-input__textarea");
      await expect(textarea).toBeVisible({ timeout: 10000 });
      await textarea.fill(
        "I think we should prioritize mobile app development for wider reach."
      );
      await page.locator(".comment-input__submit").click();

      await expect(
        page.locator(".comment-item__text", {
          hasText: "prioritize mobile app",
        })
      ).toBeVisible({ timeout: 10000 });

      await context.storageState({ path: newUserStoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 12. User2: Login + participate (after start, check prereq only) ────

  test("User2: Login and complete prerequisite (after start)", async ({
    browser,
  }) => {
    const { context, page } = await loginUser2FromSpace(browser, spaceUrl);
    try {
      await participateAndCompletePoll(page, "Science");

      user2StoragePath = `e2e-lifecycle-user2-${Date.now()}.json`;
      await context.storageState({ path: user2StoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 13. User2: Complete each action ─────────────────────────────────────

  test("User2: Complete follow action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const followBtn = page.getByRole("button", { name: "Follow" }).first();
      await expect(followBtn).toBeVisible({ timeout: 10000 });
      await followBtn.click();
      await page.waitForLoadState("load");

      await context.storageState({ path: user2StoragePath });
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
      await pauseAnimations(page);

      const quizCard = page.locator('[data-type="quiz"]').first();
      await expect(quizCard).toBeVisible({ timeout: 10000 });
      await quizCard.click();

      const overlay = page.getByTestId("quiz-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 10000 });

      await expect(page.getByTestId("quiz-arena-overview")).toBeVisible({
        timeout: 10000,
      });
      await clickNoNav(page, { testId: "quiz-arena-begin" });

      await expect(page.getByTestId("quiz-arena-questions")).toBeVisible({
        timeout: 10000,
      });

      await expect(
        overlay.getByText("Collective decision-making", { exact: true })
      ).toBeVisible({ timeout: 10000 });
      await overlay
        .getByText("Collective decision-making", { exact: true })
        .click();

      await clickNoNav(page, { testId: "quiz-arena-submit" });

      await expect(page.getByTestId("quiz-arena-overlay")).toBeHidden({
        timeout: 30000,
      });

      await context.storageState({ path: user2StoragePath });
    } finally {
      await context.close();
    }
  });

  test("User2: Complete normal poll", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const pollCard = page.locator('[data-type="poll"]').first();
      await expect(pollCard).toBeVisible({ timeout: 10000 });
      await pollCard.click();

      const overlay = page.getByTestId("poll-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 15000 });

      await clickNoNav(page, { testId: "poll-arena-begin" });

      await expect(
        overlay.getByText("API improvements", { exact: true })
      ).toBeVisible({ timeout: 10000 });
      await overlay.getByText("API improvements", { exact: true }).click();

      await clickNoNav(page, { testId: "poll-submit" });
      await clickNoNav(page, { testId: "poll-confirm-submit" });

      await expect(overlay).toBeHidden({ timeout: 30000 });

      await context.storageState({ path: user2StoragePath });
    } finally {
      await context.close();
    }
  });

  test("User2: Comment on discussion", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user2StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const discCard = page.locator('[data-type="discuss"]').first();
      await expect(discCard).toBeVisible({ timeout: 10000 });
      await page.waitForTimeout(500);
      await discCard.click();

      await expect(page.getByTestId("discussion-arena-overlay")).toBeVisible({
        timeout: 10000,
      });

      const textarea = page.locator(".comment-input__textarea");
      await expect(textarea).toBeVisible({ timeout: 10000 });
      await textarea.fill(
        "API improvements would unlock many integration opportunities for developers."
      );
      await page.locator(".comment-input__submit").click();

      await expect(
        page.locator(".comment-item__text", {
          hasText: "API improvements would unlock",
        })
      ).toBeVisible({ timeout: 10000 });
    } finally {
      await context.close();
    }
  });

  // ─── 14. Creator: Finish space ─────────────────────────────────────────

  test("Creator: Finish the space", async ({ page }) => {
    await goto(page, spaceUrl);
    await click(page, { testId: "btn-switch-creator" });

    await click(page, { text: "Finish" });
    await click(page, { testId: "end-space-button" });
    await page.waitForLoadState("load");
  });

  // ─── 15. Post-finish: discussion comment blocked ─────────────────────────

  test("User1: Cannot comment on discussion after finish", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      storageState: newUserStoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      const discCard = page.locator('[data-type="discuss"]').first();
      await expect(discCard).toBeVisible({ timeout: 10000 });
      await page.waitForTimeout(500);
      await discCard.click();

      await expect(page.getByTestId("discussion-arena-overlay")).toBeVisible({
        timeout: 10000,
      });

      // Comment input should be hidden or disabled after space is finished
      const textarea = page.locator(".comment-input__textarea");
      const submitBtn = page.locator(".comment-input__submit");

      const textareaHidden = await textarea
        .isVisible({ timeout: 3000 })
        .catch(() => false);
      const submitHidden = await submitBtn
        .isVisible({ timeout: 3000 })
        .catch(() => false);

      // Either textarea is hidden, or submit is disabled/hidden
      expect(
        !textareaHidden || !submitHidden,
        "Comment input should be hidden or submit disabled after space finish",
      ).toBeTruthy();
    } finally {
      await context.close();
    }
  });

  // ─── 16. Post-finish: viewer sees actions but cannot participate ─────────

  test("Viewer: Actions visible but participate blocked after finish", async ({
    browser,
  }) => {
    // Fresh context — anonymous viewer
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      // Viewer should see action cards (read-only results)
      const pollCard = page.locator('[data-type="poll"]');
      const quizCard = page.locator('[data-type="quiz"]');
      const discCard = page.locator('[data-type="discuss"]');
      const followCard = page.locator('[data-type="follow"]');

      const pollVisible = await pollCard
        .first()
        .isVisible({ timeout: 5000 })
        .catch(() => false);
      const quizVisible = await quizCard
        .first()
        .isVisible({ timeout: 5000 })
        .catch(() => false);
      const discVisible = await discCard
        .first()
        .isVisible({ timeout: 5000 })
        .catch(() => false);
      const followVisible = await followCard
        .first()
        .isVisible({ timeout: 5000 })
        .catch(() => false);

      expect(
        pollVisible || quizVisible || discVisible || followVisible,
        "Viewer should see at least one action card after space is finished",
      ).toBeTruthy();

      // Participate button should NOT be visible — space is finished
      const participateBtn = page.getByTestId("btn-participate");
      const canParticipate = await participateBtn
        .isVisible({ timeout: 3000 })
        .catch(() => false);
      expect(
        canParticipate,
        "Participate button should not be visible after space is finished",
      ).toBeFalsy();
    } finally {
      await context.close();
    }
  });
});
