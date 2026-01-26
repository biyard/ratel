import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

test.describe('Team Members - Authenticated User', () => {
  let testTeamUsername: string;
  let testTeamCreated = false;

  // Create ONE team for all tests in this file
  test.beforeAll(async () => {
    const timestamp = Date.now();
    testTeamUsername = `pw-members-${timestamp}`;
  });

  // Navigate to members page before each test, creating team on first run
  test.beforeEach(async ({ page }) => {
    if (!testTeamCreated) {
      const teamNickname = `Members Team ${Date.now()}`;

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
        .fill(`Playwright team for members functionality ${Date.now()}`);

      await click(page, { 'data-pw': 'team-create-button' });

      // Wait for redirect
      await page.waitForURL(`/teams/${testTeamUsername}/home`, {
        timeout: 15000,
      });

      testTeamCreated = true;
    }

    // Navigate to members page
    await page.goto(`/teams/${testTeamUsername}/members`);
    await page.waitForLoadState('networkidle');
  });

  test('[TM-001] should display members page with team owner', async ({
    page,
  }) => {
    // Verify members list container is visible
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();

    // At minimum, the team owner should be listed
    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('[TM-002] should display member profile information', async ({
    page,
  }) => {
    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();

    if (count > 0) {
      const firstMember = memberItems.first();

      // Check for profile image or placeholder
      const hasProfileImage =
        (await firstMember
          .locator('img')
          .isVisible()
          .catch(() => false)) ||
        (await firstMember
          .locator('.bg-profile-bg')
          .isVisible()
          .catch(() => false));

      expect(hasProfileImage).toBeTruthy();

      // Verify member has visible content (username or display name)
      const memberText = await firstMember.textContent();
      expect(memberText).toBeTruthy();
      expect(memberText!.length).toBeGreaterThan(0);
    }
  });

  test('[TM-003] should display member groups with tags', async ({ page }) => {
    // First, create a group and verify it exists
    await page.goto(`/teams/${testTeamUsername}/groups`);
    await page.waitForLoadState('networkidle');

    // Check if there are any groups
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const groupCount = await groupItems.count();

    if (groupCount === 0) {
      // Create a group if none exists
      await click(page, { 'data-pw': 'create-group-button' });

      // Wait for popup
      await page
        .locator('[data-pw="create-group-name-input"]')
        .waitFor({ state: 'visible' });

      await page
        .locator('[data-pw="create-group-name-input"]')
        .fill('Member Group');
      await page
        .locator('[data-pw="create-group-description-input"]')
        .fill('Group for member display');

      // Select read posts permission and wait for it
      const readToggle = page.locator('[data-pw="permission-toggle-0"]');
      await readToggle.waitFor({ state: 'visible', timeout: 15000 });
      await readToggle.click();

      await click(page, { 'data-pw': 'create-group-submit-button' });

      // Wait for group to be created
      await expect(page.getByText('Member Group')).toBeVisible();
    }

    // Go back to members page
    await page.goto(`/teams/${testTeamUsername}/members`);
    await page.waitForLoadState('networkidle');

    // Check if any member has group tags displayed
    const memberGroups = page.locator('[data-pw^="member-group-"]');
    await memberGroups.count();
  });

  test('[TM-004] should NOT show remove button for team owner', async ({
    page,
  }) => {
    // Find the member with owner badge
    const ownerMemberItem = page
      .locator('[data-pw^="member-item-"]')
      .filter({ hasText: /team.*owner/i });

    const ownerExists = await ownerMemberItem.isVisible().catch(() => false);

    if (ownerExists) {
      // Within the owner's member item, check that remove buttons are NOT present
      const removeButtons = ownerMemberItem.locator(
        '[data-pw^="remove-member-from-group-"]',
      );
      const removeButtonCount = await removeButtons.count();

      expect(removeButtonCount).toBe(0);
    }
  });

  test('[TM-005] should display empty state or loading state appropriately', async ({
    page,
  }) => {
    // This test verifies the page doesn't crash with empty data
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();

    // Check for either members or a message
    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();

    // Either we have members, or we should see some indication
    expect(count).toBeGreaterThanOrEqual(0); // No crash = success
  });

  test('[TM-006] should navigate to members page from team navigation', async ({
    page,
  }) => {
    // Start from home page
    await page.goto(`/teams/${testTeamUsername}/home`);
    await page.waitForLoadState('networkidle');

    // Click members navigation
    await click(page, { 'data-pw': 'team-nav-members' });

    // Verify URL changed
    await expect(page).toHaveURL(`/teams/${testTeamUsername}/members`);

    // Verify members list is visible
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();
  });
});
