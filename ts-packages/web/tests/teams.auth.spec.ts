import { test, expect } from '@playwright/test';
import { click } from './utils';

test('create and navigate through team sections', async ({ page }) => {
  const timestamp = Date.now();
  const teamUsername = `pw-${timestamp}`;
  const teamNickname = `Playwright Squad ${timestamp}`;
  const teamDescription = `An automated squad for verification purposes ${timestamp}`;

  console.log(`🏢 Creating new squad: ${teamUsername}`);

  // Navigate to home
  await page.goto('/');

  // Open team selector dropdown
  await click(page, { 'data-pw': 'team-selector-trigger' });

  // Click create team option
  await click(page, { 'data-pw': 'open-team-creation-popup' });

  // Wait for popup to open
  await expect(page.locator('[data-pw="team-nickname-input"]')).toBeVisible();

  // Fill in team details
  await page.locator('[data-pw="team-nickname-input"]').fill(teamNickname);
  await page.locator('[data-pw="team-username-input"]').fill(teamUsername);
  await page
    .locator('[data-pw="team-description-input"]')
    .fill(teamDescription);

  // Click create button
  await click(page, { 'data-pw': 'team-create-button' });

  // Wait for team page to load
  await page.waitForURL(`/teams/${teamUsername}/home`, { timeout: 10000 });
  console.log(`✅ Squad created: ${teamUsername}`);

  // Verify we're on the team home page
  await expect(page.locator('[data-pw="team-nav-home"]')).toBeVisible();

  // Test navigation to Drafts
  console.log('📝 Clicking Drafts section...');
  await click(page, { 'data-pw': 'team-nav-drafts' });
  await page.waitForURL(`/teams/${teamUsername}/drafts`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/drafts`);
  console.log('✅ Navigated to Drafts');

  // Test navigation to Groups
  console.log('📁 Clicking Groups section...');
  await click(page, { 'data-pw': 'team-nav-groups' });
  await page.waitForURL(`/teams/${teamUsername}/groups`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/groups`);
  console.log('✅ Navigated to Groups');

  // Test navigation to Members
  console.log('👥 Clicking Members section...');
  await click(page, { 'data-pw': 'team-nav-members' });
  await page.waitForURL(`/teams/${teamUsername}/members`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/members`);
  console.log('✅ Navigated to Members');

  // Test navigation to Settings
  console.log('⚙️ Clicking Settings section...');
  await click(page, { 'data-pw': 'team-nav-settings' });
  await page.waitForURL(`/teams/${teamUsername}/settings`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/settings`);
  console.log('✅ Navigated to Settings');

  // Navigate back to Home
  console.log('🏠 Clicking Home section...');
  await click(page, { 'data-pw': 'team-nav-home' });
  await page.waitForURL(`/teams/${teamUsername}/home`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/home`);
  console.log('✅ Navigated back to Home');

  console.log('🎉 All navigation operations completed successfully!');
});
