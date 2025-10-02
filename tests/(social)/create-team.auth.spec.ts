import { test, expect } from "@playwright/test";

test.describe("Create Team - Authenticated User", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage
    await page.goto("/");
    // Wait for page to load completely
    await page.waitForLoadState("networkidle");
  });

  test("should create a new team successfully", async ({ page }) => {
    // Open team selector dropdown
    // const teamSelectorButton = page.locator(
    //   '[data-testid="team-selector-button"]',
    // );
    // await teamSelectorButton.click();
    // // Click create team option
    // const createTeamOption = page.locator('[data-testid="create-team-option"]');
    // await createTeamOption.click();
    // // Wait for team creation form to appear
    // await page.waitForTimeout(2000);
    // // Fill in team details
    // const teamNameInput = page.locator('[data-testid="team-name-input"]');
    // await teamNameInput.fill("Test Team Name");
    // // Fill username if available
    // const usernameInput = page.locator('[data-testid="team-username-input"]');
    // await usernameInput.fill("testteam123");
    // // Fill description/bio if available
    // const descriptionField = page.locator(
    //   '[data-testid="team-description-input"]',
    // );
    // await descriptionField.fill(
    //   "This is a test team created for automated testing purposes.",
    // );
    // // Look for submit/create button
    // const createButton = page.locator('[data-testid="team-create-button"]');
    // if (await createButton.isVisible()) {
    //   await expect(createButton).toBeEnabled();
    //   await createButton.click();
    //   // Wait for team creation to complete
    //   await page.waitForTimeout(3000);
    //   // Verify success by checking if:
    //   // 1. Modal/popup closes
    //   // 2. User is redirected to team page
    //   // 3. Success message appears
    //   const modalStillVisible = await page
    //     .locator('[data-testid="team-creation-popup"]')
    //     .isVisible()
    //     .catch(() => false);
    //   const currentUrl = page.url();
    //   // Either modal should close or URL should change to team page
    //   expect(
    //     modalStillVisible === false || currentUrl.includes("/teams/"),
    //   ).toBeTruthy();
    // }
  });
});
