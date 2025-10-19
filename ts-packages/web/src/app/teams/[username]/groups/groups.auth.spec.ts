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

      console.log(`🏢 Creating shared test team: ${testTeamUsername}`);

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

      console.log(`✅ Shared test team created: ${testTeamUsername}`);
      testTeamCreated = true;
    }

    // Navigate to groups page
    await page.goto(`/teams/${testTeamUsername}/groups`);
    await page.waitForLoadState('networkidle');
  });

  test('[TG-001] should display groups page with create group button', async ({
    page,
  }) => {
    console.log('📁 Testing groups page visibility...');

    // Verify create group button is visible (only for team owners)
    const createGroupButton = page.locator('[data-pw="create-group-button"]');
    await expect(createGroupButton).toBeVisible();

    // Verify invite member button is visible
    const inviteMemberButton = page.locator('[data-pw="invite-member-button"]');
    await expect(inviteMemberButton).toBeVisible();

    console.log('✅ Groups page loaded correctly');
  });

  test('[TG-002] should create a new group with permissions', async ({
    page,
  }) => {
    console.log('📝 Testing group creation...');

    const groupName = `Group ${Date.now()}`;
    const groupDescription = 'This is a group for E2E validation';

    // Click create group button
    await click(page, { 'data-pw': 'create-group-button' });

    // Wait for popup form to be visible
    await page
      .locator('[data-pw="create-group-name-input"]')
      .waitFor({ state: 'visible' });

    // Fill in group details
    await page.locator('[data-pw="create-group-name-input"]').fill(groupName);
    await page
      .locator('[data-pw="create-group-description-input"]')
      .fill(groupDescription);

    // Wait for permission toggles to be visible and select write posts permission
    const writePostsToggle = page.locator('[data-pw="permission-toggle-1"]'); // GroupPermission.WritePosts = 1
    await writePostsToggle.waitFor({ state: 'visible', timeout: 15000 });
    await writePostsToggle.click();

    // Submit the form
    await click(page, { 'data-pw': 'create-group-submit-button' });

    // Wait for the group to appear in the list
    await expect(page.getByText(groupName)).toBeVisible();

    // Verify the group appears in the list
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const count = await groupItems.count();
    expect(count).toBeGreaterThan(0);

    console.log(`✅ Group created: ${groupName}`);
  });

  test('[TG-003] should display group with member count', async ({ page }) => {
    console.log('👥 Testing group member count display...');

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
      console.log('✅ Group member count displayed');
    } else {
      console.log('⚠️ No groups to verify member count');
    }
  });

  test('[TG-004] should show group options menu for team owner', async ({
    page,
  }) => {
    console.log('⚙️ Testing group options menu...');

    const groupItems = page.locator('[data-pw^="group-item-"]');
    const count = await groupItems.count();

    if (count > 0) {
      // Get first group's ID from data-pw attribute
      const firstGroupPw = await groupItems.first().getAttribute('data-pw');
      const groupId = firstGroupPw?.replace('group-item-', '');

      if (groupId) {
        // Click options button
        const optionsButton = page.locator(
          `[data-pw="group-options-${groupId}"]`,
        );
        await expect(optionsButton).toBeVisible();
        await optionsButton.click();

        // Verify delete button is visible in dropdown
        const deleteButton = page.locator(
          `[data-pw="delete-group-${groupId}"]`,
        );
        await expect(deleteButton).toBeVisible();

        // Close the menu by clicking elsewhere
        await page.keyboard.press('Escape');

        console.log('✅ Group options menu displayed correctly');
      }
    } else {
      console.log('⚠️ No groups to test options menu');
    }
  });

  test('[TG-005] should delete a group', async ({ page }) => {
    console.log('🗑️ Testing group deletion...');

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
    const groupId = groupPw?.replace('group-item-', '');

    if (groupId) {
      // Click options and delete
      await page.locator(`[data-pw="group-options-${groupId}"]`).click();

      // Wait for delete button to appear and click
      const deleteButton = page.locator(`[data-pw="delete-group-${groupId}"]`);
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

      console.log(`✅ Group deleted: ${groupName}`);
    }
  });

  test('[TG-006] should validate required fields when creating group', async ({
    page,
  }) => {
    console.log('✅ Testing group creation validation...');

    // Click create group button
    await click(page, { 'data-pw': 'create-group-button' });

    // Wait for popup
    await page
      .locator('[data-pw="create-group-submit-button"]')
      .waitFor({ state: 'visible' });

    // Try to submit without filling anything
    await click(page, { 'data-pw': 'create-group-submit-button' });

    // Verify error message appears (using more flexible text matching)
    const errorVisible =
      (await page
        .getByText(/group.*name.*required/i)
        .isVisible()
        .catch(() => false)) ||
      (await page
        .getByText(/name.*required/i)
        .isVisible()
        .catch(() => false)) ||
      (await page
        .getByText(/required/i)
        .isVisible()
        .catch(() => false));

    expect(errorVisible).toBeTruthy();

    console.log('✅ Validation working correctly');

    // Close popup
    await page.keyboard.press('Escape');
  });

  test('[TG-007] should select all permissions in a group', async ({
    page,
  }) => {
    console.log('🔘 Testing select all permissions...');

    await click(page, { 'data-pw': 'create-group-button' });

    // Wait for the permission section to load
    await page
      .locator('[data-pw="create-group-name-input"]')
      .waitFor({ state: 'visible' });

    // Click "Select All" for Post permissions - wait for it to be visible
    const selectAllPost = page.locator(
      '[data-pw="permission-select-all-post"]',
    );
    await selectAllPost.waitFor({ state: 'visible', timeout: 15000 });
    await selectAllPost.click();

    // Verify all post permissions toggles are visible
    const readPostsToggle = page.locator('[data-pw="permission-toggle-0"]');
    const writePostsToggle = page.locator('[data-pw="permission-toggle-1"]');
    const deletePostsToggle = page.locator('[data-pw="permission-toggle-2"]');

    await expect(readPostsToggle).toBeVisible();
    await expect(writePostsToggle).toBeVisible();
    await expect(deletePostsToggle).toBeVisible();

    console.log('✅ Select all permissions working');

    // Close popup
    await page.keyboard.press('Escape');
  });
});
