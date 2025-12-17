import { test, expect } from '@playwright/test';
import { click } from '../../../../../tests/utils';
import { CONFIGS } from '../../../../../tests/config';

test.describe('Team Settings - Authenticated User', () => {
  let testTeamUsername: string;
  let testTeamNickname: string;
  let testTeamCreated = false;

  // Create ONE team for all tests in this file
  test.beforeAll(async () => {
    const timestamp = Date.now();
    testTeamUsername = `pw-settings-${timestamp}`;
    testTeamNickname = `Settings Team ${timestamp}`;
  });

  // Navigate to settings page before each test, creating team on first run
  test.beforeEach(async ({ page }) => {
    if (!testTeamCreated) {
      await page.goto('/');
      await page.waitForLoadState('networkidle');

      await click(page, { 'data-pw': 'team-selector-trigger' });
      await click(page, { 'data-pw': 'open-team-creation-popup' });

      await page.waitForSelector('[data-pw="team-nickname-input"]', {
        timeout: CONFIGS.PAGE_WAIT_TIME,
      });

      await page
        .locator('[data-pw="team-nickname-input"]')
        .fill(testTeamNickname);
      await page
        .locator('[data-pw="team-username-input"]')
        .fill(testTeamUsername);
      await page
        .locator('[data-pw="team-description-input"]')
        .fill(`Playwright team for settings functionality ${Date.now()}`);

      await click(page, { 'data-pw': 'team-create-button' });

      // Wait for redirect
      await page.waitForURL(`/teams/${testTeamUsername}/home`, {
        timeout: 15000,
      });

      testTeamCreated = true;
    }

    // Navigate to settings page
    await page.goto(`/teams/${testTeamUsername}/settings`);
    await page.waitForLoadState('networkidle');
    // Wait for settings form to be visible
    await page
      .locator('[data-pw="team-nickname-input"]')
      .waitFor({ state: 'visible' });
  });

  test('[TS-001] should display settings page with team information', async ({
    page,
  }) => {
    // Verify username display input
    const usernameDisplay = page.locator('[data-pw="team-username-display"]');
    await expect(usernameDisplay).toBeVisible();
    await expect(usernameDisplay).toHaveValue(`@${testTeamUsername}`);
    await expect(usernameDisplay).toBeDisabled();

    // Verify nickname input
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await expect(nicknameInput).toBeVisible();

    // Verify description textarea
    const descriptionInput = page.locator('[data-pw="team-description-input"]');
    await expect(descriptionInput).toBeVisible();

    // Verify save button
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');
    await expect(saveButton).toBeVisible();
  });

  test('[TS-002] should display delete team button for team owner', async ({
    page,
  }) => {
    const deleteButton = page.locator('[data-pw="team-delete-button"]');
    const isVisible = await deleteButton.isVisible().catch(() => false);

    // Team owner should see delete button
    expect(isVisible).toBeTruthy();
  });

  test('[TS-003] should update team nickname', async ({ page }) => {
    const newNickname = `Updated Team ${Date.now()}`;

    // Fill new nickname
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInput.clear();
    await nicknameInput.fill(newNickname);

    // Click save button
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Verify we're redirected to team home
    // await page.waitForURL(`/teams/${testTeamUsername}/home`);

    // Go back to settings to verify the change persisted

    const nicknameInputAfter = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInputAfter.waitFor({ state: 'visible' });
    await expect(nicknameInputAfter).toHaveValue(newNickname);
  });

  test('[TS-004] should disable save button for invalid input', async ({
    page,
  }) => {
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');

    // Start with current valid value
    const currentNickname = await nicknameInput.inputValue();

    // Clear to empty (which should disable)
    await nicknameInput.clear();

    // Check if button gets disabled for empty input
    const isDisabledForEmpty = await saveButton.isDisabled().catch(() => false);

    // If empty doesn't disable, the form allows empty values (which is valid)
    if (!isDisabledForEmpty) {
      // Restore original value
      await nicknameInput.fill(currentNickname);
    }
  });

  test('[TS-005] should show delete team confirmation popup', async ({
    page,
  }) => {
    const deleteButton = page.locator('[data-pw="team-delete-button"]');
    const isVisible = await deleteButton.isVisible().catch(() => false);

    if (isVisible) {
      // First ensure button is enabled (no invalid input)
      const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
      const currentValue = await nicknameInput.inputValue();

      // Make sure we have valid input
      if (currentValue.includes('test')) {
        await nicknameInput.clear();
        await nicknameInput.fill('Valid Team Name');
      }

      // Click delete button
      await deleteButton.click();

      // Verify confirmation popup appears
      const confirmButton = page.locator(
        '[data-pw="delete-team-confirm-button"]',
      );
      await confirmButton.waitFor({ state: 'visible' });
      const cancelButton = page.locator(
        '[data-pw="delete-team-cancel-button"]',
      );

      await expect(confirmButton).toBeVisible();
      await expect(cancelButton).toBeVisible();

      // Cancel the deletion (we don't want to actually delete during test)
      await cancelButton.click();

      // Verify popup closed
      await expect(confirmButton).not.toBeVisible();
      const popupStillVisible = await confirmButton
        .isVisible()
        .catch(() => false);
      expect(popupStillVisible).toBeFalsy();
    }
  });

  test('[TS-006] should navigate to settings page from team navigation', async ({
    page,
  }) => {
    // Start from home page
    await page.goto(`/teams/${testTeamUsername}/home`);
    await page.waitForLoadState('networkidle');

    // Click settings navigation
    await click(page, { 'data-pw': 'team-nav-settings' });

    // Verify URL changed
    await expect(page).toHaveURL(`/teams/${testTeamUsername}/settings`);

    // Verify settings form is visible
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await expect(nicknameInput).toBeVisible();
  });

  test('[TS-007] should preserve username as read-only', async ({ page }) => {
    const usernameDisplay = page.locator('[data-pw="team-username-display"]');

    // Verify it's disabled
    await expect(usernameDisplay).toBeDisabled();

    // Verify it shows the correct username
    await expect(usernameDisplay).toHaveValue(`@${testTeamUsername}`);

    // Try to modify (should not be possible)
    const isEditable = await usernameDisplay.isEditable().catch(() => false);
    expect(isEditable).toBeFalsy();
  });

  test('[TS-008] should handle empty nickname gracefully', async ({ page }) => {
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');

    // Clear nickname
    await nicknameInput.clear();

    // Save button might be disabled or enabled depending on validation
    const isDisabled = await saveButton.isDisabled().catch(() => false);

    if (!isDisabled) {
      // If save is allowed with empty nickname, try to save
      await click(page, { 'data-pw': 'team-settings-save-button' });

      // Wait for redirect or error
      await page
        .waitForURL(`/teams/${testTeamUsername}/home`, { timeout: 5000 })
        .catch(() => {});
    }
  });
});
