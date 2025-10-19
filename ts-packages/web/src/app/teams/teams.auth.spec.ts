import { test, expect } from '@playwright/test';
import { click } from '../../../tests/utils';

test('create and navigate through team sections', async ({ page }) => {
  const timestamp = Date.now();
  const teamUsername = `pw-${timestamp}`;
  const teamNickname = `Playwright Squad ${timestamp}`;
  const teamDescription = `An automated squad for verification purposes ${timestamp}`;

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

  // Verify we're on the team home page
  await expect(page.locator('[data-pw="team-nav-home"]')).toBeVisible();

  // Test navigation to Drafts
  await click(page, { 'data-pw': 'team-nav-drafts' });
  await page.waitForURL(`/teams/${teamUsername}/drafts`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/drafts`);

  // Test navigation to Groups
  await click(page, { 'data-pw': 'team-nav-groups' });
  await page.waitForURL(`/teams/${teamUsername}/groups`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/groups`);

  // Test navigation to Members
  await click(page, { 'data-pw': 'team-nav-members' });
  await page.waitForURL(`/teams/${teamUsername}/members`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/members`);

  // Test navigation to Settings
  await click(page, { 'data-pw': 'team-nav-settings' });
  await page.waitForURL(`/teams/${teamUsername}/settings`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/settings`);

  // Navigate back to Home
  await click(page, { 'data-pw': 'team-nav-home' });
  await page.waitForURL(`/teams/${teamUsername}/home`, { timeout: 5000 });
  await expect(page).toHaveURL(`/teams/${teamUsername}/home`);
});
