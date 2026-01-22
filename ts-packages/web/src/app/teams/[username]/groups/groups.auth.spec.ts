import { test, expect } from '@playwright/test';
import { click } from '../../../../../tests/utils';
import { CONFIGS } from '../../../../../tests/config';

test.describe('Team Groups - Authenticated User', () => {
  let testTeamUsername: string;
  let testTeamCreated = false;

  // Create ONE team for all tests in this file
  test.beforeAll(async () => {
    const timestamp = Date.now();
    testTeamUsername = `pw-groups-${timestamp}`;
  });

  // Navigate to groups page before each test, creating team on first run
  test.beforeEach(async ({ page }) => {
    if (!testTeamCreated) {
      const teamNickname = `Groups Team ${Date.now()}`;

      await page.goto('/');
      await page.waitForLoadState('networkidle');

      await click(page, { 'data-pw': 'team-selector-trigger' });
      await click(page, { 'data-pw': 'open-team-creation-popup' });

      await page.waitForSelector('[data-pw="team-nickname-input"]', {
        timeout: CONFIGS.PAGE_WAIT_TIME,
      });

      await page.locator('[data-pw="team-nickname-input"]').fill(teamNickname);
      await page
        .locator('[data-pw="team-username-input"]')
        .fill(testTeamUsername);
      await page
        .locator('[data-pw="team-description-input"]')
        .fill(`Playwright team for groups functionality ${Date.now()}`);

      await click(page, { 'data-pw': 'team-create-button' });

      // Wait for redirect
      await page.waitForURL(`/teams/${testTeamUsername}/home`, {
        timeout: 15000,
      });

      testTeamCreated = true;
    }

    // Navigate to groups page
    await page.goto(`/teams/${testTeamUsername}/groups`);
    await page.waitForLoadState('networkidle');
  });

  test('[TG-001] should display groups page with create group button', async ({
    page,
  }) => {
    // Verify create group button is visible (only for team owners)
    const createGroupButton = page.locator('[data-pw="create-group-button"]');
    await expect(createGroupButton).toBeVisible();

    // Verify invite member button is visible
    const inviteMemberButton = page.locator('[data-pw="invite-member-button"]');
    await expect(inviteMemberButton).toBeVisible();
  });

  test('[TG-002] should create a group and verify it in the list', async ({
    page,
  }) => {
    const groupName = `E2E Group ${Date.now()}`;
    const groupDescription = 'Group for E2E';

    // Click create group button
    await click(page, { 'data-pw': 'create-group-button' });

    // Wait for popup to open
    await page
      .locator('[data-pw="create-group-name-input"]')
      .waitFor({ state: 'visible' });

    // Fill group name
    await page.locator('[data-pw="create-group-name-input"]').fill(groupName);

    // Fill group description
    await page
      .locator('[data-pw="create-group-description-input"]')
      .fill(groupDescription);

    // Wait for permission toggle to be visible before clicking
    const readPostsToggle = page.locator('[data-pw="permission-toggle-0"]');
    await readPostsToggle.waitFor({ state: 'visible', timeout: 10000 });
    await readPostsToggle.click();

    // Click submit button
    await click(page, { 'data-pw': 'create-group-submit-button' });

    // Wait for popup to close and group to appear in list
    // The group should appear after the onCreate callback refetches data
    await page.waitForLoadState('networkidle');

    // Verify group name is visible in the list
    const groupNameLocator = page.getByText(groupName);
    await expect(groupNameLocator).toBeVisible();

    // Verify the group has a data-pw attribute with its ID
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const count = await groupItems.count();
    expect(count).toBeGreaterThan(0);

    // Find the specific group item by text
    const createdGroupItem = page
      .locator('[data-pw^="group-item-"]')
      .filter({ hasText: groupName });
    await expect(createdGroupItem).toBeVisible();
  });

  test('[TG-003] should display group with member count', async ({ page }) => {
    // Check if there are any groups
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const count = await groupItems.count();

    if (count > 0) {
      // Verify first group has member count displayed
      const firstGroup = groupItems.first();
      await expect(firstGroup).toBeVisible();

      // Check for member count text (could be "0 member" or "1 member", etc.)
      const hasMembers = await firstGroup
        .locator('text=/\\d+ member/i')
        .isVisible()
        .catch(() => false);

      expect(hasMembers).toBeTruthy();
    }
  });

  test('[TG-004] should show group options menu for team owner', async ({
    page,
  }) => {
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const count = await groupItems.count();

    if (count > 0) {
      // Get first group's ID from data-pw attribute
      const firstGroupPw = await groupItems.first().getAttribute('data-pw');
      const groupPk = firstGroupPw?.replace('group-item-', '');

      if (groupPk) {
        // Click options button
        const optionsButton = page.locator(
          `[data-pw="group-options-${groupPk}"]`,
        );
        await expect(optionsButton).toBeVisible();
        await optionsButton.click();

        // Verify delete button is visible in dropdown
        const deleteButton = page.locator(
          `[data-pw="delete-group-${groupPk}"]`,
        );
        await expect(deleteButton).toBeVisible();

        // Close the menu by clicking elsewhere
        await page.keyboard.press('Escape');
      }
    }
  });

  test('[TG-005] should delete a group', async ({ page }) => {
    // First create a group to delete
    const groupName = `Delete Group ${Date.now()}`;

    await click(page, { 'data-pw': 'create-group-button' });

    // Wait for popup
    await page
      .locator('[data-pw="create-group-name-input"]')
      .waitFor({ state: 'visible' });

    await page.locator('[data-pw="create-group-name-input"]').fill(groupName);
    await page
      .locator('[data-pw="create-group-description-input"]')
      .fill('This group will be deleted');

    // Select at least one permission and wait for it to be visible
    const readPostsToggle = page.locator('[data-pw="permission-toggle-0"]');
    await readPostsToggle.waitFor({ state: 'visible', timeout: 15000 });
    await readPostsToggle.click();

    await click(page, { 'data-pw': 'create-group-submit-button' });

    // Verify group was created
    await expect(page.getByText(groupName)).toBeVisible();

    // Get the count before deletion
    const groupItemsBefore = page.locator('[data-pw^="group-item-"]');
    const countBefore = await groupItemsBefore.count();

    // Find the group we just created
    const targetGroupItem = page
      .locator('[data-pw^="group-item-"]')
      .filter({ hasText: groupName });
    const groupPw = await targetGroupItem.getAttribute('data-pw');
    const groupPk = groupPw?.replace('group-item-', '');

    if (groupPk) {
      // Click options and delete
      await page.locator(`[data-pw="group-options-${groupPk}"]`).click();

      // Wait for delete button to appear and click
      const deleteButton = page.locator(`[data-pw="delete-group-${groupPk}"]`);
      await expect(deleteButton).toBeVisible();
      await deleteButton.click();

      // Wait for deletion to complete by checking the group disappears
      await expect(page.getByText(groupName)).not.toBeVisible();

      // Verify group is removed from the list
      const groupItemsAfter = page.locator('[data-pw^="group-item-"]');
      const countAfter = await groupItemsAfter.count();

      expect(countAfter).toBe(countBefore - 1);

      // Verify the specific group name is no longer visible
      const isGroupVisible = await page
        .getByText(groupName)
        .isVisible()
        .catch(() => false);
      expect(isGroupVisible).toBeFalsy();
    }
  });
});
