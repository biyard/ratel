// Re-export all common helpers from shared location
export {
  login,
  logout,
  TIMEOUT,
  createTeam,
  clickTeamSidebarMenu,
  goToMySpaces,
  goToSpace,
  publishSpacePrivately,
  createPollPost,
  createPollQuestions,
} from '../helpers';

import { expect, Page } from '@playwright/test';
import { TEAM_ID } from './data';
import { goToTeam as goToTeamBase } from '../helpers';

const TIMEOUT = 10000;

// Rewards-specific wrapper that uses TEAM_ID from data
export async function goToTeam(page: Page) {
  await goToTeamBase(page, TEAM_ID);
}

// Rewards-specific helpers
export async function goToMembership(page: Page) {
  await page.goto('/membership', { waitUntil: 'networkidle' });
}

export async function subscribeToPro(page: Page) {
  await goToMembership(page);

  // Click on Pro membership card
  const proCard = page.getByTestId('membership-card-pro');
  await proCard.waitFor({ state: 'visible', timeout: TIMEOUT });
  await proCard.click();

  // Confirm subscription
  const subscribeButton = page.getByTestId('subscribe-button');
  await subscribeButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await subscribeButton.click();

  // Wait for success
  await page.waitForLoadState('networkidle');
}

export async function goToRewardsSettings(page: Page, spacePk: string) {
  await page.goto(`/spaces/${spacePk}/rewards`, { waitUntil: 'networkidle' });
}

export async function createReward(
  page: Page,
  config: { label: string; description: string; credits: number },
) {
  // Click create reward button
  const createButton = page.getByTestId('create-reward-button');
  await createButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await createButton.click();

  // Fill in the form
  const labelInput = page.getByTestId('reward-label-input');
  await labelInput.waitFor({ state: 'visible', timeout: TIMEOUT });
  await labelInput.fill(config.label);

  const descriptionInput = page.getByTestId('reward-description-input');
  await descriptionInput.fill(config.description);

  const creditsInput = page.getByTestId('reward-credits-input');
  await creditsInput.fill(config.credits.toString());

  // Submit
  const submitButton = page.getByTestId('reward-submit-button');
  await submitButton.click();

  await page.waitForLoadState('networkidle');
}

export async function respondToPoll(
  page: Page,
  answers: Array<number | number[]>,
) {
  // Wait for poll to load
  const firstOption = page.getByTestId('objective-viewer-option').first();
  await expect(firstOption).toBeVisible({ timeout: TIMEOUT });
  await page.waitForTimeout(500);

  for (let i = 0; i < answers.length; i++) {
    const answer = answers[i];
    if (Array.isArray(answer)) {
      // Multiple choice
      for (const idx of answer) {
        await page.getByTestId('objective-viewer-option').nth(idx).click();
        await page.waitForTimeout(300);
      }
    } else {
      // Single choice
      await page.getByTestId('objective-viewer-option').nth(answer).click();
    }
    await page.waitForTimeout(500);

    // Click next or submit based on question index
    if (i < answers.length - 1) {
      await page.getByTestId('survey-btn-next').click();
    } else {
      await page.getByTestId('survey-btn-submit').click();
    }
    await page.waitForTimeout(500);
  }

  // Confirm completion
  const confirmButton = page.getByTestId('complete-survey-modal-btn-confirm');
  if (await confirmButton.isVisible()) {
    await confirmButton.click();
  }
}

export async function checkUserCredits(page: Page): Promise<number> {
  // Navigate to profile or membership page to check credits
  await page.goto('/profile', { waitUntil: 'networkidle' });
  const creditsElement = page.getByTestId('user-credits');
  await creditsElement.waitFor({ state: 'visible', timeout: TIMEOUT });
  const creditsText = await creditsElement.textContent();
  return parseInt(creditsText || '0', 10);
}

export async function checkRewardClaims(
  page: Page,
  spacePk: string,
): Promise<number> {
  await page.goto(`/spaces/${spacePk}/rewards`, { waitUntil: 'networkidle' });
  const claimsElement = page.getByTestId('reward-user-claims').first();
  await claimsElement.waitFor({ state: 'visible', timeout: TIMEOUT });
  const claimsText = await claimsElement.textContent();
  return parseInt(claimsText || '0', 10);
}
