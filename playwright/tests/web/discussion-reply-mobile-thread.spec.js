import { expect, test, devices } from "@playwright/test";
import {
  click,
  createTeamFromHome,
  createTeamPostFromHome,
  goto,
} from "../utils";
import { CONFIGS } from "../config";

/**
 * Regression guard for the mobile-only "in-sheet thread drill-down".
 * Desktop keeps the inline toggle; on mobile the reply button swaps the
 * comments bottom-sheet content over to a YouTube/Reddit-style thread
 * view (parent + replies + composer) WITHOUT changing the URL, so the
 * sheet handle stays reachable and Back returns to the comments list
 * without popping the route stack.
 *
 * Setup (team → post → space → discussion → comment) is done on the
 * default desktop context (matches the working deep-link spec), and the
 * mobile-specific assertions run in a fresh iPhone 14 context that
 * shares the same auth state.
 */

// Matches the 500px CSS breakpoint used by the arena composer styles.
// iPhone 14 is 390×844, well under the threshold.
const MOBILE_DEVICE = devices["iPhone 14"];

async function dismissDevToast(page) {
  await page.evaluate(() => {
    document.querySelectorAll(".dx-toast").forEach((el) => {
      if (el.id !== "dx-toast-template") {
        el.style.display = "none";
        el.style.pointerEvents = "none";
      }
    });
  });
}

test.describe.serial("Mobile reply thread navigation", () => {
  let spaceUrl;
  let spaceId;
  let discussionId;
  let commentId;

  const teamUsername = `e2e_mr_${Date.now()}`;
  const teamNickname = "Mobile-reply Team";
  const postTitle = "Mobile reply thread test space";
  const postContents =
    "Space for testing the mobile dedicated reply thread page.";
  const commentText = `Mobile reply target comment ${Date.now()}`;

  test("Setup: create team + post + space", async ({ page }) => {
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "Mobile reply e2e test",
    });
    const postId = await createTeamPostFromHome(
      page,
      teamUsername,
      postTitle,
      postContents,
    );
    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(
      spaceRes.ok(),
      `create space: ${await spaceRes.text()}`,
    ).toBeTruthy();
    spaceId = (await spaceRes.json()).space_id;
    spaceUrl = `/spaces/${spaceId}`;
  });

  test("Setup: create discussion action + capture discussion_id", async ({
    page,
  }) => {
    await goto(page, spaceUrl);
    await dismissDevToast(page);
    await click(page, { testId: "admin-add-action-card" });
    await click(page, { testId: "type-option-discuss" });
    await page.waitForURL(/\/actions\/discussions\/[^/]+\/edit/, {
      waitUntil: "load",
      timeout: 60000,
    });
    const m = page.url().match(/\/actions\/discussions\/([^/]+)\/edit/);
    expect(m, "discussion id should be parseable from editor URL").not.toBeNull();
    discussionId = m[1];
  });

  test("Setup: post a comment via the discussion overlay", async ({ page }) => {
    await goto(page, spaceUrl);
    await dismissDevToast(page);

    const discCard = page.locator('[data-type="discuss"]').first();
    await expect(discCard).toBeVisible({ timeout: 10000 });
    await page.waitForTimeout(500);
    await discCard.click();
    await expect(page.getByTestId("discussion-arena-overlay")).toBeVisible({
      timeout: 15000,
    });

    const textarea = page.locator(".comment-input__textarea");
    await expect(textarea).toBeVisible({ timeout: 15000 });
    await textarea.fill(commentText);
    await page.locator(".comment-input__submit").click();

    const posted = page.locator(".comment-item", { hasText: commentText });
    await expect(posted).toBeVisible({ timeout: 15000 });

    commentId = await posted.getAttribute("id");
    expect(commentId, "comment-item should carry a DOM id").toBeTruthy();
    expect(commentId.length).toBeGreaterThan(0);
  });

  test("Desktop: tapping Reply keeps the inline toggle (no navigation)", async ({
    page,
  }) => {
    const discussionUrl = `${spaceUrl}/discussions/${discussionId}`;
    await goto(page, discussionUrl);
    await dismissDevToast(page);

    // Pin to the specific comment by DOM id (the sk uuid) so we don't race
    // other `comment-action-reply` buttons on unrelated comments.
    const target = page.locator(`[id="${commentId}"]`);
    await expect(target).toBeVisible({ timeout: 20000 });
    const replyBtn = target.getByTestId("comment-action-reply");
    await expect(replyBtn).toBeVisible();
    await replyBtn.click();

    // Desktop expects the inline composer to surface under the comment —
    // URL must NOT change to the replies page.
    await page.waitForTimeout(500);
    expect(page.url()).not.toMatch(/\/replies$/);
  });

  test("Mobile: tapping Reply swaps the sheet over to the thread view", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      ...MOBILE_DEVICE,
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      const discussionUrl = `${spaceUrl}/discussions/${discussionId}`;
      await goto(page, discussionUrl);
      await dismissDevToast(page);

      // Mobile discussion arena presents comments in a collapsed bottom
      // sheet — the comment DOM exists under there but is translated off
      // screen until the user taps the handle. Tap the handle first so
      // the reply button inside becomes interactable.
      const sheetHandle = page.locator(".sheet-handle");
      await expect(sheetHandle).toBeVisible({ timeout: 20000 });
      await sheetHandle.click();
      await expect(page.locator(".comments-panel[data-expanded="true"]")).toBeVisible({
        timeout: 5000,
      });

      const target = page.locator(`[id="${commentId}"]`);
      await expect(target).toBeVisible({ timeout: 20000 });
      const replyBtn = target.getByTestId("comment-action-reply");
      await expect(replyBtn).toBeVisible();
      await replyBtn.click();

      // Thread view renders IN-PLACE inside the comments panel — the
      // URL stays on the discussion page so the sheet handle keeps its
      // context. Assert the in-panel swap rather than a navigation.
      const thread = page.getByTestId("reply-thread");
      await expect(thread).toBeVisible({ timeout: 15000 });
      expect(page.url()).not.toMatch(/\/replies$/);

      // Parent comment sits at the top of the thread view with the
      // original comment text intact.
      const parent = page.locator(".reply-thread__parent");
      await expect(parent).toBeVisible();
      await expect(parent).toContainText(commentText);

      // Composer is pinned at the bottom of the thread view with the
      // reply textarea ready.
      const composer = page.locator(".reply-thread .reply-input__field");
      await expect(composer).toBeVisible();
    } finally {
      await context.close();
    }
  });

  test("Mobile: back button returns to the comments list", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      ...MOBILE_DEVICE,
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      // Re-open the thread view from the discussion page (same flow as
      // the previous test, kept standalone so this test doesn't depend
      // on lingering state from a different browser context).
      const discussionUrl = `${spaceUrl}/discussions/${discussionId}`;
      await goto(page, discussionUrl);
      await dismissDevToast(page);

      await page.locator(".sheet-handle").click();
      await expect(page.locator(".comments-panel[data-expanded="true"]")).toBeVisible({
        timeout: 5000,
      });

      const target = page.locator(`[id="${commentId}"]`);
      await target.getByTestId("comment-action-reply").click();

      await expect(page.getByTestId("reply-thread")).toBeVisible({
        timeout: 15000,
      });

      // Wait for Dioxus to bind the back button's onclick — the thread
      // view is dynamically mounted (use_loader resolves async), so the
      // button can be visible before its handler is attached and a click
      // fired too early would be silently dropped.
      await page.waitForFunction(() => {
        const btn = document.querySelector('[data-testid="reply-thread-back"]');
        return !!btn && btn.hasAttribute("data-dioxus-id");
      });
      // Dev server can surface a toast overlay on top of the sheet that
      // swallows clicks. `goto()` dismisses the initial one but new toasts
      // can appear mid-test (router warnings, rebuild banners), so sweep
      // them again right before the click.
      await dismissDevToast(page);

      // Back clears the active thread signal, restoring the comments
      // list (composer + list visible again) — no route change. `force`
      // bypasses Playwright's hit-check retry loop since Dioxus's own
      // signal update (not DOM hit-target changes) is what we're waiting
      // on; without force, late mutations to the thread view can cancel
      // the click until we hit the test timeout.
      await page.getByTestId("reply-thread-back").click({ force: true });
      await expect(page.getByTestId("reply-thread")).toBeHidden({
        timeout: 10000,
      });
      await expect(page.locator(".comment-list")).toBeVisible();
    } finally {
      await context.close();
    }
  });
});
