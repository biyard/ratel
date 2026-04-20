import { test, expect } from "../fixtures";
import {
  addPollQuestion,
  click,
  clickNoNav,
  commitAutosave,
  createAction,
  createTeamFromHome,
  createTeamPostFromHome,
  fill,
  fillPollQuestion,
  getEditor,
  getLocator,
  goto,
  setReward,
  togglePrerequisite,
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

const user3 = {
  email: "hi+user3@biyard.co",
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

async function loginFromSpace(browser, spaceUrl, { email, password }) {
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
  await fill(page, { placeholder: "Enter your email address" }, email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, password);
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
    // Arena TypePicker: creates the poll and navigates to PollCreatorPage.
    await createAction(page, spaceUrl, "poll", /\/actions\/polls\//);

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Prerequisite Poll: Interest Check"
    );
    await commitAutosave(page);

    // Arena poll editor exposes two option inputs by default (no "Add Option").
    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "What topic interests you most?",
      options: ["Technology", "Science"],
    });

    // Prerequisite + reward live inline on the ConfigCard — no more Settings tab.
    await togglePrerequisite(page);
    await setReward(page, 1);
  });

  // ─── 3. Create poll (normal) + reward ────────────────────────────────────

  test("Create normal poll with reward", async ({ page }) => {
    await createAction(page, spaceUrl, "poll", /\/actions\/polls\//);

    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Survey: Feature Priority"
    );
    await commitAutosave(page);

    await addPollQuestion(page, "single");
    await fillPollQuestion(page, 0, {
      title: "Which feature should we build next?",
      options: ["Mobile app", "API improvements"],
    });

    await setReward(page, 2);
  });

  // ─── 4. Create discussion + reward ───────────────────────────────────────

  test("Create discussion with reward", async ({ page }) => {
    // Arena discussion editor lives under `/edit` (list view has no suffix).
    await createAction(
      page,
      spaceUrl,
      "discuss",
      /\/actions\/discussions\/[^/]+\/edit/
    );

    await fill(
      page,
      { placeholder: "Enter discussion title..." },
      "Discussion: Roadmap Planning"
    );

    // Wait for the <tiptap-editor> web component to mount + initialize
    // its internal editor so setContent() actually takes effect.
    await getEditor(page);
    await page.waitForFunction(
      () => {
        const el = document.querySelector("tiptap-editor");
        return !!el && typeof el.setContent === "function" && !!el._editor;
      },
      null,
      { timeout: 15000 }
    );

    // Build a payload that exercises every format the disc-body viewer
    // styles — h1/h2 headings, bold/italic/inline-code/links, nested ul +
    // ol, blockquote, table, and <hr> — so the rendered space surfaces
    // any discussion-body styling regression at a glance. Constructed as
    // an array of fragments so no single literal looks like an HTML doc.
    const richHtml = [
      "<h1>1. Roadmap Planning Overview</h1>",
      "<p>Welcome to the <strong>roadmap planning</strong> discussion. ",
      "We use this thread to coordinate <em>quarterly priorities</em> and ",
      "surface blockers across teams. See the ",
      '<a href="https://ratel.foundation">Ratel Foundation site</a> for the ',
      "full charter.</p>",
      "<h2>2. Top Priorities (Q1 -&gt; Q2)</h2>",
      "<ul>",
      "<li><p>Mobile experience parity (iOS + Android)</p></li>",
      "<li><p>Backend hardening</p>",
      "<ul>",
      "<li><p>API <code>rate-limit</code> policy</p></li>",
      "<li><p>Migration path for legacy tables</p></li>",
      "</ul>",
      "</li>",
      "<li><p>New onboarding flow with reward gating</p></li>",
      "</ul>",
      "<h2>3. Sequencing Plan</h2>",
      "<ol>",
      "<li><p>Lock scope by week 2</p></li>",
      "<li><p>Engineering kickoff in week 3</p></li>",
      "<li><p>Internal beta by week 6</p></li>",
      "</ol>",
      "<blockquote><p>Decisions made in the open hold up better than ",
      "decisions made behind closed doors. -- community guideline</p>",
      "</blockquote>",
      "<h2>4. Track Comparison</h2>",
      "<table><tbody>",
      "<tr><td><p>Track</p></td><td><p>Owner</p></td><td><p>Status</p></td></tr>",
      "<tr><td><p>Mobile</p></td><td><p>Team A</p></td><td><p>In progress</p></td></tr>",
      "<tr><td><p>Backend</p></td><td><p>Team B</p></td><td><p>Planning</p></td></tr>",
      "<tr><td><p>Onboarding</p></td><td><p>Team C</p></td><td><p>Discovery</p></td></tr>",
      "</tbody></table>",
      "<hr>",
      "<p>Reach out in this thread with questions, edits, or links to ",
      "RFCs. Use <code>@team-name</code> to ping a specific group.</p>",
    ].join("");

    // The <tiptap-editor> web component exposes setContent() which runs
    // `editor.commands.setContent` internally. We call it and then also
    // dispatch the `change` CustomEvent the editor normally emits on
    // user input so Dioxus's onchange handler picks up the HTML and
    // triggers the standard 3s html_contents autosave.
    await page.evaluate((html) => {
      const el = document.querySelector("tiptap-editor");
      if (!el) throw new Error("tiptap-editor not found");
      el.setContent(html);
      el.dispatchEvent(
        new CustomEvent("change", {
          detail: html,
          bubbles: true,
          composed: true,
        })
      );
    }, richHtml);

    // Blur to commit any focused field, then wait for the html autosave
    // (3s debounce + server roundtrip) to flip the badge to "Saved".
    await commitAutosave(page);
    await expect(page.locator(".autosave--saved").first()).toBeVisible({
      timeout: 15000,
    });

    await setReward(page, 2);
  });

  // ─── 5. Create quiz + reward ─────────────────────────────────────────────

  test("Create quiz with reward", async ({ page }) => {
    await createAction(page, spaceUrl, "quiz", /\/actions\/quizzes\//);

    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Quiz: Platform Knowledge"
    );

    const editor = await getEditor(page);
    await editor.fill("Test your knowledge about our platform fundamentals.");
    await commitAutosave(page);

    // Arena QuestionsCard: inline add button + 2 option inputs fixed. Fill
    // title + both options on quiz-question-0 with a blur after each so the
    // per-field onblur autosave commits.
    await click(page, { testId: "quiz-question-add" });
    const q0 = page.getByTestId("quiz-question-0");
    const q0Inputs = q0.locator("input.input");
    const fills = [
      "What is the main purpose of spaces?",
      "Collective decision-making",
      "Social media posting",
    ];
    for (let i = 0; i < fills.length; i += 1) {
      await q0Inputs.nth(i).fill(fills[i]);
      await q0Inputs.nth(i).press("Tab");
      await page.waitForLoadState("load");
      await page.waitForTimeout(200);
    }

    // Mark the first option correct by clicking its radio span.
    await page
      .getByTestId("quiz-question-0-opt-0")
      .locator(".q-opt__radio")
      .click();
    await page.waitForLoadState("load");

    await setReward(page, 2);
  });

  // ─── 6. Create follow + reward ───────────────────────────────────────────

  test("Create follow with reward", async ({ page }) => {
    await createAction(page, spaceUrl, "follow", /\/actions\/follows\//);
    // Arena follow creator exposes TargetsCard + ConfigCard inline.
    await getLocator(page, { testId: "page-card-config" });

    await setReward(page, 2);
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
    // Arena HUD exposes Publish as btn-publish; SpaceVisibilityModal opens
    // in-place. The legacy /dashboard Publish button no longer drives this.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-publish" });
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
    // After publish, ArenaTopbar swaps btn-publish for btn-start; clicking
    // it opens SpaceStartModal where start-space-button fires the transition.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-start" });
    await click(page, { testId: "start-space-button" });
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
    const { context, page } = await loginFromSpace(browser, spaceUrl, user2);
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

  // ─── 13b. User2: Edit + delete own comment via context menu ──────────────

  test("User2: Edit and delete own comment", async ({ browser }) => {
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

      // Post a fresh comment that will be edited, then deleted.
      const textarea = page.locator(".comment-input__textarea");
      await expect(textarea).toBeVisible({ timeout: 10000 });
      const originalText = "Draft comment from User2 — editing test.";
      await textarea.fill(originalText);
      await page.locator(".comment-input__submit").click();

      const originalItem = page.locator(".comment-item", {
        hasText: originalText,
      });
      await expect(originalItem).toBeVisible({ timeout: 10000 });

      // Open the context menu on the just-posted comment and click Edit.
      // The ⋮ trigger renders only for comments whose author matches the
      // current user, so scoping via `hasText` uniquely targets the new one.
      await originalItem.getByTestId("comment-menu-trigger").click();
      await page.getByTestId("comment-menu-edit").click();

      // Only one comment is editable at a time, so page-wide testids resolve
      // to the single edit form without needing to re-scope the parent.
      const editInput = page.getByTestId("comment-edit-input");
      await expect(editInput).toBeVisible({ timeout: 5000 });
      const editedText = "Edited via context-menu action.";
      await editInput.fill(editedText);
      await page.getByTestId("comment-edit-save").click();

      // After save, comments_loader restarts and the new content renders.
      const editedItem = page.locator(".comment-item", { hasText: editedText });
      await expect(editedItem).toBeVisible({ timeout: 10000 });
      await expect(
        page.locator(".comment-item", { hasText: originalText })
      ).toBeHidden({ timeout: 10000 });

      // Delete the edited comment via the context menu.
      await editedItem.getByTestId("comment-menu-trigger").click();
      await page.getByTestId("comment-menu-delete").click();

      await expect(
        page.locator(".comment-item", { hasText: editedText })
      ).toBeHidden({ timeout: 10000 });
    } finally {
      await context.close();
    }
  });

  // ─── 13c. User2: Edit + delete own reply via context menu ────────────────

  test("User2: Edit and delete own reply", async ({ browser }) => {
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

      // Reply to the comment User2 posted in step 13. We scope by the
      // parent's unique text snippet because `.comment-item` is shared by
      // both the parent and each reply below it.
      const parentSnippet = "API improvements";
      const parentComment = page.locator(".comment-item", {
        hasText: parentSnippet,
      });
      await expect(parentComment).toBeVisible({ timeout: 10000 });

      // Open the reply input on the parent comment.
      await parentComment.locator(".comment-action--reply").click();

      const replyInput = page.locator(".reply-input__field");
      await expect(replyInput).toBeVisible({ timeout: 5000 });

      const replyOriginal = "Reply from User2 — editing test.";
      await replyInput.fill(replyOriginal);
      await page.locator(".reply-input__send").click();

      // New reply renders inside .comment-replies. Restricting the selector
      // to that container avoids accidentally matching the parent comment.
      const originalReply = page.locator(".comment-replies .comment-item", {
        hasText: replyOriginal,
      });
      await expect(originalReply).toBeVisible({ timeout: 10000 });

      // Open the context menu on the just-posted reply and click Edit.
      // The ⋮ trigger renders only for replies whose author matches the
      // current user, so scoping via the unique reply text is enough.
      await originalReply.getByTestId("comment-menu-trigger").click();
      await page.getByTestId("comment-menu-edit").click();

      // Only one comment/reply is editable at a time, so page-wide testids
      // resolve to the single edit form.
      const editInput = page.getByTestId("comment-edit-input");
      await expect(editInput).toBeVisible({ timeout: 5000 });
      const replyEdited = "Edited reply via context-menu action.";
      await editInput.fill(replyEdited);
      await page.getByTestId("comment-edit-save").click();

      // After save the parent patches its local replies signal in place;
      // the edited text renders without needing a loader restart.
      const editedReply = page.locator(".comment-replies .comment-item", {
        hasText: replyEdited,
      });
      await expect(editedReply).toBeVisible({ timeout: 10000 });
      await expect(
        page.locator(".comment-replies .comment-item", {
          hasText: replyOriginal,
        })
      ).toBeHidden({ timeout: 10000 });

      // Delete the edited reply via the context menu.
      await editedReply.getByTestId("comment-menu-trigger").click();
      await page.getByTestId("comment-menu-delete").click();

      await expect(
        page.locator(".comment-replies .comment-item", {
          hasText: replyEdited,
        })
      ).toBeHidden({ timeout: 10000 });
    } finally {
      await context.close();
    }
  });

  // ─── 13c. User3: Login + complete prerequisite (after start) ────────────

  let user3StoragePath;

  test("User3: Login and complete prerequisite (after start)", async ({
    browser,
  }) => {
    const { context, page } = await loginFromSpace(browser, spaceUrl, user3);
    try {
      await participateAndCompletePoll(page, "Science");

      user3StoragePath = `e2e-lifecycle-user3-${Date.now()}.json`;
      await context.storageState({ path: user3StoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 13d. User3: Complete quiz action ────────────────────────────────────
  // User3 submits the correct answer — retries are exhausted after one
  // passing attempt, so the server must reveal correct answers on re-open.

  test("User3: Complete quiz action", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user3StoragePath,
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

      await context.storageState({ path: user3StoragePath });
    } finally {
      await context.close();
    }
  });

  // ─── 13e. User3: Review correct answer via archive ──────────────────────
  // After the attempt is recorded, retries are exhausted (retry_count=0).
  // The server now returns `correct_answers` and the overlay highlights
  // the right option via data-correct on the option tile.

  test("User3: Review correct answer via archive", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: user3StoragePath,
      viewport: { width: 1440, height: 950 },
      locale: "en-US",
    });
    const page = await context.newPage();

    try {
      await goto(page, spaceUrl);
      await pauseAnimations(page);

      // Open the archive panel from the bottom bar.
      await clickNoNav(page, { testId: "btn-archive" });
      const archivePanel = page.getByTestId("archive-panel");
      await expect(archivePanel).toBeVisible({ timeout: 10000 });

      // Completed quiz row lives in the archive with a green check.
      const archivedQuiz = archivePanel
        .locator(".archive-item")
        .filter({ hasText: /Quiz/i })
        .first();
      await expect(archivedQuiz).toBeVisible({ timeout: 10000 });
      await archivedQuiz.click();

      const overlay = page.getByTestId("quiz-arena-overlay");
      await expect(overlay).toBeVisible({ timeout: 10000 });

      await clickNoNav(page, { testId: "quiz-arena-begin" });
      await expect(page.getByTestId("quiz-arena-questions")).toBeVisible({
        timeout: 10000,
      });

      // Correct option highlighted via data-correct.
      const correctOption = overlay.locator(
        '.option-tile[data-correct="true"]'
      );
      await expect(correctOption).toBeVisible({ timeout: 10000 });
      await expect(correctOption).toContainText("Collective decision-making");

      // Submit stays disabled because retries are exhausted.
      const submitBtn = page.getByTestId("quiz-arena-submit");
      if (await submitBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
        await expect(submitBtn).toBeDisabled();
      }
    } finally {
      await context.close();
    }
  });

  // ─── 13f. Creator: Quiz remains editable until the action ends ───────────
  // The creator can still tweak questions/answers while the quiz is running,
  // even after participants have submitted responses. Lock kicks in only
  // once the quiz's ended_at has passed (covered by post-finish tests).

  test("Creator: Can edit quiz answer while quiz is still running", async ({
    page,
  }) => {
    await goto(page, spaceUrl);
    await pauseAnimations(page);

    const quizCard = page.locator('[data-type="quiz"]').first();
    await expect(quizCard).toBeVisible({ timeout: 10000 });

    const editBtn = quizCard.locator('[data-testid^="quest-edit-btn-"]');
    await expect(editBtn).toBeVisible({ timeout: 10000 });
    await editBtn.click();

    await page.waitForURL(/\/actions\/quizzes\//, { waitUntil: "load" });

    const questionsPage = page.locator('section.pager__page[data-page="1"]');
    await questionsPage.scrollIntoViewIfNeeded();
    await page.waitForTimeout(500);

    const q1Options = questionsPage.locator(
      '[data-testid="quiz-question-0"] .q-opt'
    );
    await expect(q1Options.first()).toBeVisible({ timeout: 10000 });
    await q1Options.nth(1).locator(".q-opt__radio").click();
    await page.waitForLoadState("load");

    // The lock-after-responses toast should NOT appear — the edit is
    // permitted as long as the quiz hasn't finished.
    await expect(
      page.getByText(/quiz cannot be edited after it has finished/i).first()
    ).toBeHidden({ timeout: 3000 });
  });

  // ─── 14. Creator: Finish space ─────────────────────────────────────────

  test("Creator: Finish the space", async ({ page }) => {
    // After space is Ongoing, ArenaTopbar shows btn-finish. Its "Finish"
    // label lives only in the :hover tooltip (opacity:0) so we cannot click
    // by text — use the stable testid instead. Clicking opens SpaceEndModal.
    await goto(page, spaceUrl);
    await hideFab(page);
    await click(page, { testId: "btn-finish" });
    await click(page, { testId: "end-space-button" });
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
        "Comment input should be hidden or submit disabled after space finish"
      ).toBeTruthy();
    } finally {
      await context.close();
    }
  });
});
