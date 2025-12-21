import { expect, Page } from '@playwright/test';
import { TIMEOUT } from './auth';

export async function goToMySpaces(page: Page) {
  await page.getByText('My Spaces', { exact: true }).click();
  await page.waitForURL(/.*/, { waitUntil: 'networkidle', timeout: TIMEOUT });
  await expect(page.getByTestId('space-card').first()).toBeVisible();
}

export async function goToSpace(page: Page, spaceName: string) {
  const spaceCard = page.getByText(spaceName).first();
  await spaceCard.waitFor({ state: 'visible', timeout: TIMEOUT });
  await spaceCard.click();

  // Wait for URL to match space pattern with network idle
  await page.waitForURL(/\/spaces\/[^/]+/, {
    waitUntil: 'networkidle',
    timeout: TIMEOUT,
  });
}

export async function publishSpacePrivately(page: Page) {
  await page.getByTestId('space-action-button').click();
  await page.getByTestId('selectable-card-private').click();
  await page.getByTestId('publish-space-modal-btn').click();
  await page.waitForLoadState('networkidle');
  await expect(page.getByTestId('space-action-button')).toHaveText('Start');
}

export async function startDeliberation(page: Page) {
  const actionButton = page.getByTestId('space-action-button');
  await expect(actionButton).toHaveText('Start');
  await actionButton.click();

  const startSpaceButton = page.getByTestId('start-space-button');
  await expect(startSpaceButton).toBeVisible();
  await startSpaceButton.click();
  await expect(page.getByText('Success to start space.')).toBeVisible();
}

export async function inviteMembers(page: Page, emails: string[]) {
  await page.getByTestId('space-sidemenu-members').click();
  await page.waitForURL(/.*\/members$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  await page.getByTestId('invite-space-btn').click();
  await page.getByTestId('member-email-input').fill(emails.join(', '));
  await page.keyboard.press('Enter');
  await page.waitForTimeout(1000);
  await page.getByTestId('invite-member-send-btn').click();
  await page.waitForLoadState('networkidle');
}

export async function setupPanels(page: Page, quota: string) {
  await page.getByTestId('space-sidemenu-panels').click();
  await page.waitForURL(/.*\/panels$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(100);

  const quotaInput = page.getByTestId('panel-quota-input');
  await quotaInput.click();
  const textbox = quotaInput.locator('[data-slot="input"]');
  await textbox.waitFor({ state: 'visible', timeout: TIMEOUT });
  await textbox.fill(quota);

  // Press Enter to commit the quota value
  await page.keyboard.press('Enter');
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(100);

  await page.getByTestId('multi-select-trigger').click();
  await page.locator('[data-value="University"]').click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(100);

  await page.locator('[data-value="Gender"]').click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(100);
}

export async function enableAnonymousParticipation(page: Page) {
  await page.getByTestId('space-sidemenu-settings').click();
  await page.waitForURL(/.*\/settings$/);
  await page.waitForLoadState('networkidle');

  await page.getByTestId('anonymous-participation-label').click();
  // Wait for the API call to complete
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(500);
}

export async function viewAnalysis(page: Page) {
  // Wait for the analysis menu item to be visible
  const analysisMenuItem = page.getByTestId('space-sidemenu-analysis');
  await analysisMenuItem.waitFor({ state: 'visible', timeout: 15000 });
  await analysisMenuItem.click();
  await page.waitForLoadState('networkidle');

  // Verify analysis page loaded
  await expect(page.getByTestId('analysis-container')).toBeVisible({
    timeout: 15000,
  });
}
