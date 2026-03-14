import { test } from "@playwright/test";
import { click, fill, goto, getLocator, getEditor } from "../utils";

// This test covers the full flow:
// 1. Create a team via the profile dropdown
// 2. Navigate to the team's draft page
// 3. Create a post (with space) from the team's draft page
// 4. Create all four action types in the space (Discussion, Poll, Quiz, Follow)
// 5. Publish the space publicly

test.describe.serial("Space with actions created by a team", () => {
  let spaceUrl;

  const teamNickname = `Test Team`;
  const teamUsername = `e2e_team_${Date.now()}`;
  const postTitle = "Team Post for Space Actions E2E Test";
  const postContents =
    "This is a test post created by a team through Playwright E2E testing. " +
    "It verifies the full flow of team creation, post creation, space actions, " +
    "and publishing. The content is intentionally long enough to meet the minimum " +
    "character requirement for post content validation.";

  async function createTeamAndPostWithSpace(page) {
    // Step 1: Navigate to home and open profile dropdown
    await goto(page, "/");

    // Open profile dropdown by clicking the user profile button in the navbar
    // Target the button specifically by its accessible name (img alt + text)
    const profileTrigger = page.getByRole("button", {
      name: "User Profile",
    });
    await profileTrigger.click();
    await page.waitForLoadState("networkidle");

    // Step 2: Click "Create Team" in the dropdown
    await click(page, { text: "Create Team" });

    // Step 3: Fill in team creation form
    // Team Name (nickname)
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInput.fill(teamNickname);

    // Team ID (username)
    const usernameInput = page.locator('[data-pw="team-username-input"]');
    await usernameInput.fill(teamUsername);

    // Team description
    const descInput = page.locator('[data-pw="team-description-input"]');
    await descInput.fill("E2E test team for space actions");

    // Click Create button to submit the form
    await click(page, { text: "Create" });

    // Wait for navigation to the team home page
    await page.waitForURL(new RegExp(`/teams/${teamUsername}/home`), {
      waitUntil: "networkidle",
    });

    // Step 4: Navigate to the team's Drafts page via direct URL
    // (sidemenu may still be loading, so use goto instead of clicking sidemenu link)
    await goto(page, `/teams/${teamUsername}/drafts`);

    // Step 5: Click "Create Post" button on the draft page
    await click(page, { label: "Create Post" });

    // Wait for post edit page to load
    await page.waitForURL(/\/posts\/.*\/edit/, {
      waitUntil: "networkidle",
    });

    // Step 6: Fill in the post
    await fill(page, { placeholder: "Title" }, postTitle);

    // Uncheck "Skip creating space" to enable space creation
    await click(page, { testId: "skip-space-checkbox" });

    const editor = await getEditor(page);
    await editor.fill(postContents);

    // Step 7: Click "Go to Space" to create the space
    await click(page, { text: "Go to Space" });

    // Wait for navigation to the space dashboard
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard/, {
      waitUntil: "networkidle",
    });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
  }

  test.beforeAll(async ({ browser }) => {
    const context = await browser.newContext({ storageState: "user.json" });
    const page = await context.newPage();

    await createTeamAndPostWithSpace(page);

    await context.close();
  });

  test("Verify space dashboard is accessible", async ({ page }) => {
    // Navigate to the space dashboard and verify it loaded
    await goto(page, `${spaceUrl}/dashboard`);
    await getLocator(page, { text: "Dashboard" });
  });

  test("Create a discussion action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // Open create action modal via the "Select Action Type" button
    await click(page, { text: "Select Action Type" });

    // Select Discussion type
    await click(page, { text: "Discussion" });

    // Hide FAB that may overlap modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    // Confirm creation
    await click(page, { text: "Create" });

    // Wait for discussion creator page
    await page.waitForURL(/\/actions\/discussions\//, {
      waitUntil: "networkidle",
    });

    // Fill discussion fields on the creator page
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

    // Fill rich text content
    const editor = await getEditor(page);
    await editor.fill(
      "This discussion was created by a team to explore governance frameworks and decision-making processes within the space.",
    );

    await click(page, { text: "Save" });
  });

  test("Create a poll action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // Open create action modal
    await click(page, { text: "Select Action Type" });

    // Select Poll type (Quiz is default, so explicitly click Poll)
    await click(page, { text: "Poll" });

    // Hide FAB that may overlap modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    // Confirm creation
    await click(page, { text: "Create" });

    // Wait for poll creator page
    await page.waitForURL(/\/actions\/polls\//, {
      waitUntil: "networkidle",
    });

    // Fill poll title
    await fill(
      page,
      { placeholder: "Enter poll title..." },
      "Team Poll: Budget Allocation",
    );

    // Trigger blur to save title
    await page.keyboard.press("Tab");
    await page.waitForLoadState("networkidle");
  });

  test("Create a quiz action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // Open create action modal
    await click(page, { text: "Select Action Type" });

    // Quiz is selected by default, no need to click it
    // Hide FAB that may overlap modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    // Confirm creation
    await click(page, { text: "Create" });

    // Wait for quiz creator page
    await page.waitForURL(/\/actions\/quizzes\//, {
      waitUntil: "networkidle",
    });

    // Fill quiz title on the Overview tab (default tab)
    await fill(
      page,
      { placeholder: "Enter quiz title..." },
      "Team Quiz: Protocol Knowledge Check",
    );

    // Fill rich text description
    const editor = await getEditor(page);
    await editor.fill(
      "This quiz tests knowledge about the governance protocol. Created by the team for participant engagement.",
    );

    await click(page, { text: "Save" });
  });

  test("Create a follow action in the space", async ({ page }) => {
    await goto(page, spaceUrl + "/actions");

    // Open create action modal
    await click(page, { text: "Select Action Type" });

    // Select Follow type (Quiz is default, so explicitly click Follow)
    await click(page, { text: "Follow" });

    // Hide FAB that may overlap modal buttons
    await page.evaluate(() => {
      const fab = document.querySelector('[class*="fixed right-4 bottom-4"]');
      if (fab) fab.style.display = "none";
    });

    // Confirm creation
    await click(page, { text: "Create" });

    // Wait for follow creator page
    await page.waitForURL(/\/actions\/follows\//, {
      waitUntil: "networkidle",
    });

    // Verify creator sees the General tab with follower settings
    await getLocator(page, { text: "General" });
  });

  test("Publish the space publicly", async ({ page }) => {
    await goto(page, spaceUrl + "/dashboard");

    // Click the Publish button in the SpaceTop bar
    await click(page, { text: "Publish" });

    // Select Public visibility option
    await click(page, { testId: "public-option" });

    // Confirm the visibility selection
    await click(page, { label: "Confirm visibility selection" });
  });
});
