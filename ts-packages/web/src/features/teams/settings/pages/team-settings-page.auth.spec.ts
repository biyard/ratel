import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

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

    // Verify nickname input is disabled in read-only mode
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await expect(nicknameInput).toBeVisible();
    await expect(nicknameInput).toBeDisabled();

    // Verify description textarea is disabled in read-only mode
    const descriptionInput = page.locator('[data-pw="team-description-input"]');
    await expect(descriptionInput).toBeVisible();
    await expect(descriptionInput).toBeDisabled();

    // Verify edit button is visible in read-only mode
    const editButton = page.locator('[data-pw="team-settings-edit-button"]');
    await expect(editButton).toBeVisible();

    // Verify save button is NOT visible in read-only mode
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');
    await expect(saveButton).not.toBeVisible();
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

    // Click edit button first
    await click(page, { 'data-pw': 'team-settings-edit-button' });

    // Wait for inputs to be enabled
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await expect(nicknameInput).toBeEnabled();

    // Fill new nickname
    await nicknameInput.clear();
    await nicknameInput.fill(newNickname);

    // Click save button
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Wait for save to complete and return to read-only mode
    await page.waitForTimeout(1000);

    // Verify we're back in read-only mode with updated value
    const nicknameInputAfter = page.locator('[data-pw="team-nickname-input"]');
    await expect(nicknameInputAfter).toBeDisabled();
    await expect(nicknameInputAfter).toHaveValue(newNickname);
  });

  test('[TS-004] should show validation error for invalid input', async ({
    page,
  }) => {
    // Click edit button first
    await click(page, { 'data-pw': 'team-settings-edit-button' });

    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');

    // Wait for edit mode
    await expect(nicknameInput).toBeEnabled();
    await expect(saveButton).toBeVisible();

    // Clear nickname (should trigger validation error on save)
    await nicknameInput.clear();

    // Try to save with empty nickname
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Should still be in edit mode (validation failed)
    await page.waitForTimeout(500);
    await expect(saveButton).toBeVisible();
    await expect(nicknameInput).toBeEnabled();
  });

  test('[TS-005] should show delete team confirmation popup', async ({
    page,
  }) => {
    // Delete button should be visible in read-only mode
    const deleteButton = page.locator('[data-pw="team-delete-button"]');
    const isVisible = await deleteButton.isVisible().catch(() => false);

    if (isVisible) {
      // Click delete button (in read-only mode)
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

  test('[TS-008] should cancel edit and discard changes', async ({ page }) => {
    // Get original nickname value
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    const originalNickname = await nicknameInput.inputValue();

    // Click edit button
    await click(page, { 'data-pw': 'team-settings-edit-button' });

    // Wait for edit mode
    await expect(nicknameInput).toBeEnabled();

    // Change nickname
    await nicknameInput.clear();
    await nicknameInput.fill('This should be discarded');

    // Click cancel button
    await click(page, { 'data-pw': 'team-settings-cancel-button' });

    // Should return to read-only mode
    await expect(nicknameInput).toBeDisabled();

    // Verify original value is restored
    await expect(nicknameInput).toHaveValue(originalNickname);
  });

  test('[TS-009] should hide delete button in edit mode', async ({ page }) => {
    const deleteButton = page.locator('[data-pw="team-delete-button"]');
    const editButton = page.locator('[data-pw="team-settings-edit-button"]');

    // Delete button should be visible in read-only mode
    const isVisibleBefore = await deleteButton.isVisible().catch(() => false);
    expect(isVisibleBefore).toBeTruthy();

    // Click edit button
    await click(page, { 'data-pw': 'team-settings-edit-button' });

    // Delete button should be hidden in edit mode
    await expect(deleteButton).not.toBeVisible();

    // Edit button should also be hidden
    await expect(editButton).not.toBeVisible();

    // Save and Cancel buttons should be visible
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');
    const cancelButton = page.locator('[data-pw="team-settings-cancel-button"]');
    await expect(saveButton).toBeVisible();
    await expect(cancelButton).toBeVisible();
  });
});
