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

  test("Create a team and post with space, then verify dashboard", async ({
    page,
  }) => {
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
    // Routes use /{username}/home (no /teams/ prefix)
    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "networkidle",
    });

    // Step 4: Create a post via the Create button on the team home page
    await click(page, { text: "Create" });

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
    await getLocator(page, { text: "Dashboard" });

    const url = new URL(page.url());
    spaceUrl = url.pathname.replace(/\/dashboard$/, "");
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

    // --- Add a Single Choice question on the Questions tab (default tab) ---
    await click(page, { testId: "poll-add-question" });
    await click(page, { text: "Single Choice" });

    // Fill the question title and options
    const textInputs = page.locator('input[type="text"]:visible');
    await textInputs.nth(0).fill("How should the team allocate the Q2 budget?");
    await textInputs.nth(1).fill("Increase marketing spend");
    await textInputs.nth(2).fill("Invest in R&D");

    // Add a third option
    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("networkidle");
    await textInputs.nth(3).fill("Save for reserves");

    // Trigger blur to save question
    await page.keyboard.press("Tab");
    await page.waitForLoadState("networkidle");

    // --- Switch to the Settings tab and enable Prerequisite ---
    await page.getByRole("tab", { name: "Settings" }).click();
    await page.waitForLoadState("networkidle");

    // The Prerequisite row: paragraph "Prerequisite" is inside a div,
    // and the Switch button is a sibling of that div. Go up two levels to the Card.
    const prerequisiteCard = page.locator("text=Prerequisite").locator("../..");
    await prerequisiteCard.locator("button").click();
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

    // Save the overview tab
    await click(page, { text: "Save" });

    // Switch to the Quiz tab to add questions.
    // Use role="tab" to avoid ambiguity with the "Quiz" page heading.
    await page.getByRole("tab", { name: "Quiz" }).click();
    await page.waitForLoadState("networkidle");

    // --- Add first question (Single Choice) ---
    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Single Choice" });

    // A question card appears with header "Question 1", a title input
    // (placeholder "Input"), and two default options ("Option 1", "Option 2").
    //
    // On the Quiz tab the visible text inputs (type="text") are ordered:
    //   [0] Q1 title (placeholder "Input")
    //   [1] Q1 option 1
    //   [2] Q1 option 2
    // Number inputs (Pass Score, Retry Count) are type="number" and excluded.
    const textInputs = page.locator('input[type="text"]:visible');

    // Fill the question title
    await textInputs.nth(0).fill(
      "What is the primary purpose of governance in a DAO?",
    );

    // Edit the default option texts
    await textInputs.nth(1).fill("To centralize power");
    await textInputs.nth(2).fill("To enable collective decision-making");

    // Add a third option via the "Add Option" button
    await page.getByRole("button", { name: "Add Option" }).first().click();
    await page.waitForLoadState("networkidle");

    // The new option appears as text input index 3
    await textInputs.nth(3).fill("To maximize profits only");

    // Mark the correct answer by clicking the checkbox label next to option 2
    // (index 1, "To enable collective decision-making").
    // Each option row has a <label> wrapping a hidden checkbox.
    const checkboxLabels = page.locator('label:has(input[type="checkbox"])');
    await checkboxLabels.nth(1).click();
    await page.waitForLoadState("networkidle");

    // --- Add second question (Multiple Choice) ---
    await click(page, { testId: "quiz-add-question" });
    await click(page, { text: "Multiple Choice" });

    // After adding Q2 the text input ordering becomes:
    //   [0] Q1 title, [1-3] Q1 options (3 total),
    //   [4] Q2 title (placeholder "Input"), [5] Q2 option 1, [6] Q2 option 2
    await textInputs.nth(4).fill(
      "Which of the following are benefits of decentralized governance?",
    );
    await textInputs.nth(5).fill("Transparency");
    await textInputs.nth(6).fill("Community participation");

    // Add a third option for question 2 (use the second "Add Option" button)
    await page.getByRole("button", { name: "Add Option" }).nth(1).click();
    await page.waitForLoadState("networkidle");

    // New option is text input index 7
    await textInputs.nth(7).fill("Single point of failure");

    // Mark correct answers for the multiple-choice question.
    // Q1 has 3 checkbox labels (indices 0-2).
    // Q2's checkbox labels start at index 3.
    // Check options 1 and 2 ("Transparency", "Community participation").
    await checkboxLabels.nth(3).click();
    await checkboxLabels.nth(4).click();

    // Trigger save by pressing Tab to blur the last active element
    await page.keyboard.press("Tab");
    await page.waitForLoadState("networkidle");
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
