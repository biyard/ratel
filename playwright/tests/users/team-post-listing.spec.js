import { test, expect } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor } from "../utils";

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
    // Step 1: Navigate to home and create a team
    await goto(page, "/");
    await click(page, { label: "User Profile" });
    await click(page, { text: "Create Team" });

    // Fill in team creation form
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInput.fill(teamNickname);

    const usernameInput = page.locator('[data-pw="team-username-input"]');
    await usernameInput.fill(teamUsername);

    const descInput = page.locator('[data-pw="team-description-input"]');
    await descInput.fill("E2E test team for post listing");

    await click(page, { text: "Create" });

    // Wait for navigation to team home
    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "load",
    });

    // Step 2: Create a post from the team home page
    await click(page, { text: "Create" });
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    // Step 3: Fill in post content
    await fill(page, { placeholder: "Title" }, postTitle);

    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Step 4: Publish the post
    await click(page, { text: "Publish" });
    await page.waitForLoadState("load");

    // Step 5: Navigate back to team home
    await goto(page, `/${teamUsername}/home`);

    // Step 6: Verify the post appears in the team home list
    const postElement = page.getByText(postTitle);
    await expect(postElement).toBeVisible({ timeout: 10000 });
  });
});
