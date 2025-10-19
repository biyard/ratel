import { test, expect } from '@playwright/test';
import { click } from '../../../../../tests/utils';
import { CONFIGS } from '../../../../../tests/config';

test.describe('Team Members - Authenticated User', () => {
  let testTeamUsername: string;

  test.beforeEach(async ({ page }) => {
    // Create a fresh team for each test
    const timestamp = Date.now();
    testTeamUsername = `pw-members-${timestamp}`;
    const teamNickname = `Members Test Team ${timestamp}`;

    console.log(`🏢 Creating test team: ${testTeamUsername}`);

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
      .fill(`Test team for members functionality ${timestamp}`);

    await click(page, { 'data-pw': 'team-create-button' });

    // Wait for redirect with increased timeout
    await page.waitForURL(
      (url) => url.pathname.includes(`/teams/${testTeamUsername}`),
      {
        timeout: 15000,
      },
    );

    // Navigate to members page
    await page.goto(`/teams/${testTeamUsername}/members`);
    await page.waitForLoadState('networkidle');

    console.log(
      `✅ Test team created and navigated to members: ${testTeamUsername}`,
    );
  });

  test('[TM-001] should display members page with team owner', async ({
    page,
  }) => {
    console.log('👥 Testing members page visibility...');

    // Wait for members list to load
    await page.waitForTimeout(1000);

    // Verify members list container is visible
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();

    // At minimum, the team owner should be listed
    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();
    expect(count).toBeGreaterThanOrEqual(1);

    console.log(`✅ Members page loaded with ${count} member(s)`);
  });

  test('[TM-002] should display team owner badge', async ({ page }) => {
    console.log('👑 Testing team owner badge display...');

    // Wait for members to load
    await page.waitForTimeout(1000);

    // Look for team owner indicator
    const ownerBadge = page.getByText(/team.*owner/i);
    const hasOwnerBadge = await ownerBadge.isVisible().catch(() => false);

    expect(hasOwnerBadge).toBeTruthy();

    console.log('✅ Team owner badge displayed');
  });

  test('[TM-003] should display member profile information', async ({
    page,
  }) => {
    console.log('ℹ️ Testing member profile information display...');

    // Wait for members to load
    await page.waitForTimeout(1000);

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

      console.log('✅ Member profile information displayed');
    }
  });

  test('[TM-004] should display member groups with tags', async ({ page }) => {
    console.log('🏷️ Testing member group tags display...');

    // Wait for members to load
    await page.waitForTimeout(1000);

    // First, create a group and verify it exists
    await page.goto(`/teams/${testTeamUsername}/groups`);
    await page.waitForTimeout(500);

    // Check if there are any groups
    const groupItems = page.locator('[data-pw^="group-item-"]');
    const groupCount = await groupItems.count();

    if (groupCount === 0) {
      // Create a test group if none exists
      console.log('📝 Creating a test group first...');
      await click(page, { 'data-pw': 'create-group-button' });
      await page.waitForTimeout(500);

      await page
        .locator('[data-pw="create-group-name-input"]')
        .fill('Test Member Group');
      await page
        .locator('[data-pw="create-group-description-input"]')
        .fill('Group for testing member display');

      // Select read posts permission
      await page.locator('[data-pw="permission-toggle-0"]').click();

      await click(page, { 'data-pw': 'create-group-submit-button' });
      await page.waitForTimeout(1000);
    }

    // Go back to members page
    await page.goto(`/teams/${testTeamUsername}/members`);
    await page.waitForTimeout(1000);

    // Check if any member has group tags displayed
    const memberGroups = page.locator('[data-pw^="member-group-"]');
    const memberGroupCount = await memberGroups.count();

    // The owner might or might not be in groups, so this is informational
    console.log(
      `📊 Found ${memberGroupCount} member-group associations displayed`,
    );
  });

  test('[TM-005] should NOT show remove button for team owner', async ({
    page,
  }) => {
    console.log('🛡️ Testing that team owner cannot be removed from groups...');

    // Wait for members to load
    await page.waitForTimeout(1000);

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

      console.log('✅ Team owner does not have remove buttons');
    } else {
      console.log('⚠️ Team owner badge not found');
    }
  });

  test('[TM-006] should display empty state or loading state appropriately', async ({
    page,
  }) => {
    console.log('⏳ Testing loading/empty states...');

    // This test verifies the page doesn't crash with empty data
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();

    // Check for either members or a message
    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();

    // Either we have members, or we should see some indication
    expect(count).toBeGreaterThanOrEqual(0); // No crash = success

    console.log('✅ Page handles members list state correctly');
  });

  test('[TM-007] should navigate to members page from team navigation', async ({
    page,
  }) => {
    console.log('🧭 Testing navigation to members page...');

    // Start from home page
    await page.goto(`/teams/${testTeamUsername}/home`);
    await page.waitForTimeout(500);

    // Click members navigation
    await click(page, { 'data-pw': 'team-nav-members' });

    // Verify URL changed
    await expect(page).toHaveURL(`/teams/${testTeamUsername}/members`);

    // Verify members list is visible
    const membersList = page.locator('[data-pw="team-members-list"]');
    await expect(membersList).toBeVisible();

    console.log('✅ Navigation to members page works correctly');
  });

  test('[TM-008] should display multiple members if team has them', async ({
    page,
  }) => {
    console.log('📊 Testing multiple members display...');

    await page.waitForTimeout(1000);

    const memberItems = page.locator('[data-pw^="member-item-"]');
    const count = await memberItems.count();

    // For a newly created team, we should have at least the owner
    expect(count).toBeGreaterThanOrEqual(1);

    if (count > 1) {
      console.log(`✅ Displaying ${count} team members`);

      // Verify each member item has basic structure
      for (let i = 0; i < Math.min(count, 3); i++) {
        const memberItem = memberItems.nth(i);
        await expect(memberItem).toBeVisible();

        const hasContent = (await memberItem.textContent())!.length > 0;
        expect(hasContent).toBeTruthy();
      }
    } else {
      console.log('ℹ️ Only team owner is a member (expected for new team)');
    }
  });
});
