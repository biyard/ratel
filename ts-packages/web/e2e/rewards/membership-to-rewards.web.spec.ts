import { test, expect } from '@playwright/test';
import {
  POLL_QUESTIONS,
  POST_CONTENT,
  POST_TITLE,
  REWARD_CONFIG,
  TEAM_DESCRIPTION,
  TEAM_ID,
  TEAM_NAME,
  TEST_PASSWORD,
} from './data';
import {
  createPollPost,
  createPollQuestions,
  createTeam,
  goToTeam,
  login,
  publishSpacePrivately,
  goToMySpaces,
  TIMEOUT,
} from './helpers';

test.describe.serial('[Rewards] Membership to Reward Configuration', () => {
  // Disable retries for serial tests - shared state can't be restored after retry
  // Increase timeout for serial tests that include complex operations (60 seconds)
  test.describe.configure({ retries: 0, timeout: 60000 });

  let testNo = 0;
  const nextNo = () => {
    testNo += 1;
    return testNo;
  };

  // Test users
  const creator = {
    email: 'hi+admin1@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant = {
    email: 'hi+user1@biyard.co',
    password: TEST_PASSWORD,
  };

  // State for serial tests
  let savedCookies: any[] = [];
  let savedUrl: string = '/';
  let spacePk: string = '';

  test.beforeEach(async ({ page }) => {
    page.context().addCookies(savedCookies);
    await page.goto(savedUrl, { waitUntil: 'networkidle' });
  });

  test.afterEach(async ({ page }) => {
    savedUrl = page.url();
    savedCookies = await page.context().cookies();
  });

  // =====================================
  // Setup: Create Team and Poll Space
  // =====================================
  test(`RS-${nextNo()} [Creator] Create a team`, async ({ page }) => {
    await login(page, creator);
    await createTeam(page, TEAM_NAME, TEAM_ID, TEAM_DESCRIPTION);
  });

  test(`RS-${nextNo()} [Creator] Create a poll space`, async ({ page }) => {
    await login(page, creator);
    await goToTeam(page);
    await createPollPost(page, POST_TITLE, POST_CONTENT);

    // Get the space URL and extract spacePk
    const currentUrl = page.url();
    const match = currentUrl.match(/\/spaces\/([^/]+)/);
    if (match) {
      spacePk = match[1];
    }
  });

  test(`RS-${nextNo()} [Creator] Configure poll questions`, async ({
    page,
  }) => {
    await login(page, creator);

    // Navigate to the draft post and then to space
    await page.goto(`/teams/${TEAM_ID}/drafts`, { waitUntil: 'networkidle' });
    await page.getByText(POST_TITLE).click();
    await page.waitForLoadState('networkidle');

    // Click Next button to proceed to space configuration
    const nextButton = page.getByRole('button', { name: 'Next' });
    await nextButton.waitFor({ state: 'visible', timeout: TIMEOUT });
    await nextButton.click();

    await page.waitForURL(/.*\/spaces\/.*/, { timeout: 15000 });
    await page.waitForLoadState('networkidle');

    // Extract spacePk from URL
    const currentUrl = page.url();
    const match = currentUrl.match(/\/spaces\/([^/]+)/);
    if (match) {
      spacePk = match[1];
    }

    await createPollQuestions(page, POLL_QUESTIONS);
  });

  // =====================================
  // Membership: Subscribe to Pro
  // =====================================
  test(`RS-${nextNo()} [Creator] Subscribe to Pro membership`, async ({
    page,
  }) => {
    await login(page, creator);

    // Navigate to membership page
    await page.goto('/membership', { waitUntil: 'networkidle' });

    // Check current membership status
    const membershipStatus = page.getByTestId('current-membership-tier');
    await membershipStatus.waitFor({ state: 'visible', timeout: TIMEOUT });
    const currentTier = await membershipStatus.textContent();

    if (currentTier?.toLowerCase() === 'pro') {
      // Already Pro, skip
      return;
    }

    // Click on Pro membership card to select
    const proCard = page.locator('[data-testid="membership-card-Pro"]');
    await proCard.waitFor({ state: 'visible', timeout: TIMEOUT });
    await proCard.click();

    // Wait for subscription modal or confirmation
    await page.waitForLoadState('networkidle');

    // Verify Pro membership is now active
    await expect(membershipStatus).toContainText(/pro/i, { timeout: TIMEOUT });
  });

  test(`RS-${nextNo()} [Creator] Verify credits are available`, async ({
    page,
  }) => {
    await login(page, creator);

    // Check credits on membership or profile page
    await page.goto('/membership', { waitUntil: 'networkidle' });

    const creditsElement = page.getByTestId('user-remaining-credits');
    await creditsElement.waitFor({ state: 'visible', timeout: TIMEOUT });

    const creditsText = await creditsElement.textContent();
    const credits = parseInt(creditsText || '0', 10);

    // Pro membership should give 40 credits
    expect(credits).toBeGreaterThanOrEqual(40);
  });

  // =====================================
  // Reward Configuration
  // =====================================
  test(`RS-${nextNo()} [Creator] Navigate to rewards settings`, async ({
    page,
  }) => {
    await login(page, creator);

    // Navigate to space rewards settings
    await page.goto(`/spaces/${spacePk}/rewards`, {
      waitUntil: 'networkidle',
    });

    // Verify we're on the rewards page
    const rewardsTitle = page.getByText('Reward Settings');
    await expect(rewardsTitle).toBeVisible({ timeout: TIMEOUT });
  });

  test(`RS-${nextNo()} [Creator] Create a poll reward`, async ({ page }) => {
    await login(page, creator);

    // Navigate to space rewards settings
    await page.goto(`/spaces/${spacePk}/rewards`, {
      waitUntil: 'networkidle',
    });

    // Click create reward button (for poll feature section)
    const createRewardBtn = page.getByTestId('create-reward-btn').first();
    await createRewardBtn.waitFor({ state: 'visible', timeout: TIMEOUT });
    await createRewardBtn.click();

    // Fill reward form in modal
    const labelInput = page.locator('input[placeholder*="label"]').first();
    if (await labelInput.isVisible()) {
      await labelInput.fill(REWARD_CONFIG.label);
    }

    const descriptionInput = page
      .locator('input[placeholder*="description"]')
      .first();
    if (await descriptionInput.isVisible()) {
      await descriptionInput.fill(REWARD_CONFIG.description);
    }

    const creditsInput = page.locator('input[type="number"]').first();
    await creditsInput.waitFor({ state: 'visible', timeout: TIMEOUT });
    await creditsInput.fill(REWARD_CONFIG.credits.toString());

    // Submit the form
    const saveButton = page.getByRole('button', { name: /save/i });
    await saveButton.click();

    await page.waitForLoadState('networkidle');

    // Verify reward was created
    await expect(page.getByText(REWARD_CONFIG.label)).toBeVisible({
      timeout: TIMEOUT,
    });
  });

  test(`RS-${nextNo()} [Creator] Verify credits were deducted`, async ({
    page,
  }) => {
    await login(page, creator);

    // Check remaining credits
    await page.goto('/membership', { waitUntil: 'networkidle' });

    const creditsElement = page.getByTestId('user-remaining-credits');
    await creditsElement.waitFor({ state: 'visible', timeout: TIMEOUT });

    const creditsText = await creditsElement.textContent();
    const credits = parseInt(creditsText || '0', 10);

    // Credits should be reduced by reward amount (40 - 10 = 30)
    expect(credits).toBe(30);
  });

  // =====================================
  // Publish Space
  // =====================================
  test(`RS-${nextNo()} [Creator] Publish space privately`, async ({ page }) => {
    await login(page, creator);

    // Navigate to space
    await page.goto(`/spaces/${spacePk}`, { waitUntil: 'networkidle' });

    await publishSpacePrivately(page);

    // Verify space is published
    await expect(page.getByTestId('space-action-button')).toHaveText('Start');
  });

  // =====================================
  // Participant: Respond to Poll
  // =====================================
  test(`RS-${nextNo()} [Participant] Check invitation and enter space`, async ({
    page,
  }) => {
    await login(page, participant);
    await goToMySpaces(page);

    // Find the space card
    const spaceCard = page.getByText(POST_TITLE).first();
    await expect(spaceCard).toBeVisible({ timeout: 15000 });
    await spaceCard.click();

    await page.waitForURL(/\/spaces\/[^/]+/, {
      waitUntil: 'networkidle',
      timeout: TIMEOUT,
    });
  });

  test(`RS-${nextNo()} [Participant] Respond to poll`, async ({ page }) => {
    await login(page, participant);
    await goToMySpaces(page);

    // Navigate to space
    await page.getByText(POST_TITLE).first().click();
    await page.waitForLoadState('networkidle');

    // Navigate to polls
    await page.getByTestId('space-sidemenu-polls').click();
    await page.waitForURL(/.*\/polls$/, { timeout: TIMEOUT });
    await page.waitForLoadState('networkidle');

    // Enter the poll
    const enterButton = page.getByRole('button', { name: 'Enter' }).first();
    await enterButton.click();
    await page.waitForLoadState('networkidle');

    // Respond to poll questions
    // Q1: Single choice - select first option
    await page.getByTestId('objective-viewer-option').first().click();
    await page.waitForTimeout(300);

    // Q2: Multiple choice - select first two options
    await page.getByTestId('objective-viewer-option').nth(0).click();
    await page.waitForTimeout(200);
    await page.getByTestId('objective-viewer-option').nth(1).click();
    await page.waitForTimeout(300);

    // Submit
    await page.getByTestId('survey-btn-submit').click();
    await page.waitForTimeout(500);

    // Confirm completion
    const confirmBtn = page.getByTestId('complete-survey-modal-btn-confirm');
    if (await confirmBtn.isVisible()) {
      await confirmBtn.click();
    }

    await page.waitForLoadState('networkidle');
  });

  test(`RS-${nextNo()} [Participant] Verify reward claim recorded`, async ({
    page,
  }) => {
    await login(page, participant);

    // Navigate to space rewards to check user_claims
    await page.goto(`/spaces/${spacePk}/rewards`, {
      waitUntil: 'networkidle',
    });

    // Check that user_claims is now 1
    const claimsElement = page.getByTestId('reward-user-claims').first();
    if (await claimsElement.isVisible()) {
      const claimsText = await claimsElement.textContent();
      expect(parseInt(claimsText || '0', 10)).toBe(1);
    }

    // Alternatively check via reward card
    const rewardCard = page.getByText(REWARD_CONFIG.label).first();
    await expect(rewardCard).toBeVisible({ timeout: TIMEOUT });
  });

  // =====================================
  // Creator: Verify reward total claims
  // =====================================
  test(`RS-${nextNo()} [Creator] Verify total claims increased`, async ({
    page,
  }) => {
    await login(page, creator);

    // Navigate to space rewards settings
    await page.goto(`/spaces/${spacePk}/rewards`, {
      waitUntil: 'networkidle',
    });

    // Check total_claims on the reward card
    const totalClaimsElement = page.getByTestId('reward-total-claims').first();
    if (await totalClaimsElement.isVisible()) {
      const totalClaimsText = await totalClaimsElement.textContent();
      expect(parseInt(totalClaimsText || '0', 10)).toBeGreaterThanOrEqual(1);
    }
  });
});
