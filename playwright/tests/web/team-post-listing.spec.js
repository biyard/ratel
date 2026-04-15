import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor } from "../utils";

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
    // Step 1: Create the team via API (the home-ui renewal removed the
    // profile-dropdown "Create Team" path this test used to exercise).
    const res = await page.request.post("/api/teams/create", {
      data: {
        body: {
          username: teamUsername,
          nickname: teamNickname,
          profile_url: "",
          description: "E2E test team for post listing",
        },
      },
    });
    expect(res.ok(), `create team: ${await res.text()}`).toBeTruthy();

    await goto(page, `/${teamUsername}/home`);
    await expect(page).toHaveURL(new RegExp(`/${teamUsername}/home`));

    // Wait for the team arena layout to hydrate and propagate the owner
    // role into the arena context — the Create Post button is gated by
    // `arena.can_edit` which flips to true after the layout's use_effect
    // runs (post-hydration).
    await expect(page.getByTestId("team-home-create-post")).toBeVisible({
      timeout: 15000,
    });

    // Step 2: Create a post from the team home page
    await click(page, { testId: "team-home-create-post" });
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    // Step 3: Fill in post content
    await fill(page, { placeholder: "Title your post…" }, postTitle);

    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Step 4: Publish the post. The post-edit renewal dropped the old
    // visibility modal — the top-bar Publish button now publishes directly
    // using the inline visibility selector (defaults to Public).
    await click(page, { role: "button", text: "Publish" });
    await page.waitForURL(/\/posts\/[^/]+$/, { waitUntil: "load" });

    // Step 5: Navigate back to team home
    await goto(page, `/${teamUsername}/home`);

    // Step 6: Verify the post appears in the team home list
    const postElement = page.getByText(postTitle);
    await expect(postElement).toBeVisible({ timeout: 10000 });
  });
});
