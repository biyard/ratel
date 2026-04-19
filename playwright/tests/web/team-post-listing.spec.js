import { test, expect } from "@playwright/test";
import {
  click,
  createTeamFromHome,
  createTeamPostFromHome,
  goto,
} from "../utils";

// This test verifies the fix for issue #1311:
// Team posts should appear in the Team Home list after creation.
//
// Requires backend built with --features bypass for signup flows.

test.describe.serial("Team post listing (issue-1311)", () => {
  const teamNickname = `Post Team`;
  const teamUsername = `e2e_post_${Date.now()}`;
  const postTitle = `Team Post ${Date.now()}`;
  const postContents =
    "This is a team post created to verify that team posts appear " +
    "correctly in the team home list. The content needs to be long " +
    "enough to pass the minimum character requirement for content " +
    "validation on the server side.";

  test("Create a team and publish a post, then verify it appears in team home", async ({
    page,
  }) => {
    // Step 1: Create the team through the home → Teams HUD → "Create Team"
    // popup UI flow (same path a user exercises in production).
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "E2E test team for post listing",
    });

    // Step 2: Drive post creation through the production UI flow:
    // home (`/`) → Teams HUD dropdown → pick team → team home → Create Post.
    // createTeamPostFromHome fills title + body and waits for autosave.
    await createTeamPostFromHome(page, teamUsername, postTitle, postContents);

    // Step 3: Publish the post. The post-edit renewal dropped the old
    // visibility modal — the top-bar Publish button publishes directly
    // using the inline visibility selector (defaults to Public).
    await click(page, { role: "button", text: "Publish" });
    await page.waitForURL(/\/posts\/[^/]+$/, { waitUntil: "load" });

    // Step 4: Navigate directly to team home to confirm the published
    // post surfaces in the team home list. We drive this as a direct URL
    // navigation instead of routing through the home Teams HUD dropdown:
    // after the Publish flow the Dioxus arena is left in a desynced
    // state where home-btn-teams toggles aria-expanded=true but the
    // dropdown element stays CSS-hidden, making `openTeamFromHome`'s
    // internal retry loop hit its 15s timeout. This test's goal is to
    // verify the listing content, not the HUD UI, so a direct goto is
    // the appropriate path.
    await goto(page, `/${teamUsername}/home`);
    const postElement = page.getByText(postTitle);
    await expect(postElement).toBeVisible({ timeout: 10000 });
  });
});
