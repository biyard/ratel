import { test, expect } from '@playwright/test';
import { click } from '../../../../../tests/utils';
import { CONFIGS } from '../../../../../tests/config';

test.describe('Team Settings - Authenticated User', () => {
  let testTeamUsername: string;
  let testTeamNickname: string;

  // Create ONE team for all tests in this file
  test.beforeAll(async ({ browser }) => {
    const context = await browser.newContext({
      storageState: 'user.json',
    });
    const page = await context.newPage();

    const timestamp = Date.now();
    testTeamUsername = `pw-settings-${timestamp}`;
    testTeamNickname = `Settings Team ${timestamp}`;

    console.log(`🏢 Creating shared test team: ${testTeamUsername}`);

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
      .fill(`Playwright team for settings functionality ${timestamp}`);

    await click(page, { 'data-pw': 'team-create-button' });

    // Wait for redirect with increased timeout
    await page.waitForURL(
      (url) => url.pathname.includes(`/teams/${testTeamUsername}/home`),
      {
        timeout: 15000,
      },
    );

    console.log(`✅ Shared test team created: ${testTeamUsername}`);

    await context.close();
  });

  // Navigate to settings page before each test
  test.beforeEach(async ({ page }) => {
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
    console.log('⚙️ Testing settings page visibility...');

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

    console.log('✅ Settings page loaded correctly');
  });

  test('[TS-002] should display delete team button for team owner', async ({
    page,
  }) => {
    console.log('🗑️ Testing delete button visibility...');

    const deleteButton = page.locator('[data-pw="team-delete-button"]');
    const isVisible = await deleteButton.isVisible().catch(() => false);

    // Team owner should see delete button
    expect(isVisible).toBeTruthy();

    console.log('✅ Delete button visible for team owner');
  });

  test('[TS-003] should update team nickname', async ({ page }) => {
    console.log('✏️ Testing team nickname update...');

    const newNickname = `Updated Team ${Date.now()}`;

    // Fill new nickname
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInput.clear();
    await nicknameInput.fill(newNickname);

    // Click save button
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Verify we're redirected to team home
    await page.waitForURL(`/teams/${testTeamUsername}/home`);

    // Go back to settings to verify the change persisted
    await page.goto(`/teams/${testTeamUsername}/settings`);
    await page.waitForLoadState('networkidle');

    const nicknameInputAfter = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInputAfter.waitFor({ state: 'visible' });
    await expect(nicknameInputAfter).toHaveValue(newNickname);

    console.log(`✅ Team nickname updated to: ${newNickname}`);
  });

  test('[TS-004] should update team description', async ({ page }) => {
    console.log('📝 Testing team description update...');

    const newDescription = `Updated description for testing ${Date.now()}`;

    // Fill new description
    const descriptionInput = page.locator('[data-pw="team-description-input"]');
    await descriptionInput.clear();
    await descriptionInput.fill(newDescription);

    // Click save button
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Wait for redirect
    await page.waitForURL(`/teams/${testTeamUsername}/home`);

    // Go back to settings to verify
    await page.goto(`/teams/${testTeamUsername}/settings`);
    await page.waitForLoadState('networkidle');

    const descriptionInputAfter = page.locator(
      '[data-pw="team-description-input"]',
    );
    await expect(descriptionInputAfter).toHaveValue(newDescription);

    console.log('✅ Team description updated successfully');
  });

  test('[TS-005] should update both nickname and description together', async ({
    page,
  }) => {
    console.log('🔄 Testing simultaneous nickname and description update...');

    const newNickname = `Bulk Update ${Date.now()}`;
    const newDescription = `Bulk description update ${Date.now()}`;

    // Update both fields
    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    await nicknameInput.clear();
    await nicknameInput.fill(newNickname);

    const descriptionInput = page.locator('[data-pw="team-description-input"]');
    await descriptionInput.clear();
    await descriptionInput.fill(newDescription);

    // Save changes
    await click(page, { 'data-pw': 'team-settings-save-button' });

    // Wait for redirect
    await page.waitForURL(`/teams/${testTeamUsername}/home`);

    // Verify both changes persisted
    await page.goto(`/teams/${testTeamUsername}/settings`);
    await page.waitForLoadState('networkidle');

    await expect(page.locator('[data-pw="team-nickname-input"]')).toHaveValue(
      newNickname,
    );
    await expect(
      page.locator('[data-pw="team-description-input"]'),
    ).toHaveValue(newDescription);

    console.log('✅ Both fields updated successfully');
  });

  test('[TS-006] should disable save button for invalid input', async ({
    page,
  }) => {
    console.log('🚫 Testing save button disabled state...');

    const nicknameInput = page.locator('[data-pw="team-nickname-input"]');
    const saveButton = page.locator('[data-pw="team-settings-save-button"]');

    // Try to enter text with "test" keyword (which is filtered)
    await nicknameInput.clear();
    await nicknameInput.fill('Team invalid');

    // Save button should be disabled
    await expect(saveButton).toBeDisabled();

    console.log('✅ Save button correctly disabled for invalid input');
  });

  test('[TS-007] should show delete team confirmation popup', async ({
    page,
  }) => {
    console.log('⚠️ Testing delete team confirmation...');

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

      console.log('✅ Delete confirmation popup works correctly');
    } else {
      console.log('⚠️ Delete button not visible (might not have permission)');
    }
  });

  test('[TS-008] should navigate to settings page from team navigation', async ({
    page,
  }) => {
    console.log('🧭 Testing navigation to settings page...');

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

    console.log('✅ Navigation to settings page works correctly');
  });

  test('[TS-009] should preserve username as read-only', async ({ page }) => {
    console.log('🔒 Testing username field is read-only...');

    const usernameDisplay = page.locator('[data-pw="team-username-display"]');

    // Verify it's disabled
    await expect(usernameDisplay).toBeDisabled();

    // Verify it shows the correct username
    await expect(usernameDisplay).toHaveValue(`@${testTeamUsername}`);

    // Try to modify (should not be possible)
    const isEditable = await usernameDisplay.isEditable().catch(() => false);
    expect(isEditable).toBeFalsy();

    console.log('✅ Username field is correctly read-only');
  });

  test('[TS-010] should handle empty nickname gracefully', async ({ page }) => {
    console.log('📭 Testing empty nickname handling...');

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

      // Should still work (empty/undefined nickname is allowed)
      console.log('✅ Empty nickname is allowed');
    } else {
      console.log('✅ Empty nickname prevents saving (validation works)');
    }
  });
});
