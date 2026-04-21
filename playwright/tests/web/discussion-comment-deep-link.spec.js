import { expect, test } from "@playwright/test";
import {
  click,
  createTeamFromHome,
  createTeamPostFromHome,
  goto,
} from "../utils";

// Dismiss the `dx serve` rebuild toast that overlays the page whenever the
// dev server rebuilds mid-suite. The template is always in the HTML (hidden),
// but runtime rebuilds activate a `.dx-toast` node that steals focus/clicks
// until it's dismissed by the user. Tests don't care about the toast, so we
// hide any active copies on every navigation.
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

// Covers the `/spaces/:sid/discussions/:did/comments/:comment_id` deep-link
// route added for mention notification CTAs. Comment id is in the path
// (not query/fragment) because Dioxus Router strips both query strings
// and fragments during URL normalization on hydration. The admin
// creates a discussion (which starts Designing/InProgress immediately
// on an unpublished space), posts a comment as Creator, then navigates
// directly to the deep-link URL and asserts the target `.comment-item`
// carries `data-deep-link="true"` (which drives the CSS pulse).
test.describe.serial("Discussion comment deep-link", () => {
  let spaceUrl;
  let spaceId;
  let discussionId;
  let commentId;

  const teamUsername = `e2e_dl_${Date.now()}`;
  const teamNickname = "Deep-link Team";
  const postTitle = "Deep-link test space";
  const postContents =
    "Space for testing comment fragment deep-link navigation.";
  const commentText = `Deep-link target comment ${Date.now()}`;

  test("Setup: create team + post + space", async ({ page }) => {
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "Deep-link e2e test",
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
    // After creation the creator lands on the discussion editor. The URL
    // is the canonical source of the discussion UUID we need for the
    // deep-link target.
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

    // Open the discussion overlay from the admin quest carousel.
    const discCard = page.locator('[data-type="discuss"]').first();
    await expect(discCard).toBeVisible({ timeout: 10000 });
    await page.waitForTimeout(500);
    await discCard.click();
    await expect(page.getByTestId("discussion-arena-overlay")).toBeVisible({
      timeout: 15000,
    });

    // Creator role + fresh (unpublished) discussion → `can_comment = true`
    // since `status != Processing/Finished` and `started_at == now`.
    const textarea = page.locator(".comment-input__textarea");
    await expect(textarea).toBeVisible({ timeout: 15000 });
    await textarea.fill(commentText);
    await page.locator(".comment-input__submit").click();

    const posted = page.locator(".comment-item", { hasText: commentText });
    await expect(posted).toBeVisible({ timeout: 15000 });

    // The `id` attribute is the UUID portion of `comment.sk` — match it
    // against the URL fragment we're about to construct.
    commentId = await posted.getAttribute("id");
    expect(commentId, "comment-item should carry a DOM id").toBeTruthy();
    expect(commentId.length).toBeGreaterThan(0);
  });

  test("Deep-link URL scrolls to + highlights the target comment", async ({
    page,
  }) => {
    const deepLinkUrl = `${spaceUrl}/discussions/${discussionId}/comments/${commentId}`;
    await goto(page, deepLinkUrl);
    await dismissDevToast(page);

    // Path param survives Dioxus Router (unlike query/fragment) — sanity
    // check the URL didn't get rewritten away from us.
    const observedPath = await page.evaluate(() => window.location.pathname);
    expect(observedPath).toContain(`/comments/${commentId}`);

    await expect(page.locator(".discussion-arena")).toBeVisible({
      timeout: 20000,
    });

    const target = page.locator(`[id="${commentId}"]`);
    await expect(target).toBeVisible({ timeout: 15000 });
    await expect(target).toContainText(commentText);

    // `DiscussionArenaPage`'s deep-link effect flags the matching comment
    // via a Dioxus signal → `data-deep-link="true"` attribute → CSS pulse.
    await expect(target).toHaveAttribute("data-deep-link", "true", {
      timeout: 15000,
    });
  });
});
