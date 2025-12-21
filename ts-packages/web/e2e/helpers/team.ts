import { expect, Page } from '@playwright/test';
import { TIMEOUT } from './auth';

export async function goToTeam(page: Page, teamId: string) {
  await page.goto(`/teams/${teamId}/home`, { waitUntil: 'networkidle' });
}

export async function goToTeamSpace(page: Page, teamId: string) {
  await page.goto(`/teams/${teamId}/draft`, { waitUntil: 'networkidle' });
}

export async function createTeam(
  page: Page,
  teamName: string,
  teamId: string,
  description: string,
) {
  await page.locator('[data-pw="team-selector-trigger"]').click();
  await page.locator('[data-pw="open-team-creation-popup"]').click();

  await page.locator('[data-pw="team-nickname-input"]').fill(teamName);
  await page.locator('[data-pw="team-username-input"]').fill(teamId);
  await page.locator('[data-pw="team-description-input"]').fill(description);
  await page.locator('[data-pw="team-create-button"]').click();

  await page.waitForURL(`/teams/${teamId}/home`, { timeout: TIMEOUT });
  await expect(page.getByRole('button', { name: teamName })).toBeVisible();
}

export async function clickTeamSidebarMenu(page: Page, menuName: string) {
  const menu = page.getByTestId(`sidemenu-team-${menuName}`);
  await menu.waitFor({ state: 'visible' });
  await menu.click();
  await page.waitForLoadState('networkidle');
}
