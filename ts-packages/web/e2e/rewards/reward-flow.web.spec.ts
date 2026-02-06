import { test, expect } from '@playwright/test';
import {
  POLL_QUESTIONS,
  POLL_TITLE,
  POLL_CONTENT,
  REWARD_CONFIG,
  REWARD_TEST_PASSWORD,
  TEAM_NAME,
  TEAM_ID,
  TEAM_DESCRIPTION,
} from './data';
import {
  login,
  createTeam,
  createPostWithPollSpace,
  createSimplePoll,
  navigateToRewardsTab,
  addPollReward,
  respondToPoll,
  navigateToMyRewards,
  verifyRewardReceived,
  TIMEOUT,
} from './helpers';

test.describe.serial('[Reward] Poll Space Reward Flow', () => {
  test.describe.configure({ retries: 0, timeout: 60000 });

  const userA = {
    email: 'hi+admin1@biyard.co',
    password: REWARD_TEST_PASSWORD,
  };

  const userB = {
    email: 'hi+user1@biyard.co',
    password: REWARD_TEST_PASSWORD,
  };

  // Shared state between tests
  let spaceUrl: string = '';
  let savedCookies: any[] = [];
  let savedUrl: string = '/';

  test.beforeEach(async ({ page }) => {
    page.context().addCookies(savedCookies);
    await page.goto(savedUrl, { waitUntil: 'networkidle' });
  });

  test.afterEach(async ({ page }) => {
    savedUrl = page.url();
    savedCookies = await page.context().cookies();
  });

  // =====================================
  // User A: Create Team
  // =====================================
  test('RW-1 [User A] Create a team', async ({ page }) => {
    await login(page, userA);
    await createTeam(page, TEAM_NAME, TEAM_ID, TEAM_DESCRIPTION);
  });

  // =====================================
  // User A: Create Poll Space
  // =====================================
  test('RW-2 [User A] Create a post with Poll Space', async ({ page }) => {
    await login(page, userA);

    // Create post with poll space
    spaceUrl = await createPostWithPollSpace(
      page,
      TEAM_ID,
      POLL_TITLE,
      POLL_CONTENT,
    );

    // Verify we're on the space page
    expect(page.url()).toContain('/spaces/');
  });

  // =====================================
  // User A: Add Poll Question
  // =====================================
  test('RW-3 [User A] Add poll question to the space', async ({ page }) => {
    await login(page, userA);
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');

    // Create poll with questions
    await createSimplePoll(page, POLL_QUESTIONS);

    // Verify poll was created by checking for question title
    await expect(page.getByText(POLL_QUESTIONS[0].title)).toBeVisible({
      timeout: TIMEOUT,
    });
  });

  // =====================================
  // User A: Add Reward to Poll
  // =====================================
  test('RW-4 [User A] Add reward to the poll', async ({ page }) => {
    await login(page, userA);
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');

    // Navigate to Rewards tab
    await navigateToRewardsTab(page);

    // Add reward
    await addPollReward(page, REWARD_CONFIG.credits, REWARD_CONFIG.description);

    // Verify reward card is displayed by looking for reward content
    const rewardTitle = page.getByText('Poll Response').first();
    await expect(rewardTitle).toBeVisible({ timeout: TIMEOUT });

    // Verify credits are displayed
    const creditsText = page.getByText(`Credits:`).first();
    await expect(creditsText).toBeVisible();

    // Publish the poll so User B can access it
    const publishButton = page.getByRole('button', { name: 'Publish' });
    await publishButton.click();

    // Wait for publish modal and select Public Publish
    const publicPublishOption = page.getByText('Public Publish').first();
    await publicPublishOption.waitFor({ state: 'visible', timeout: TIMEOUT });
    await publicPublishOption.click();

    // Click the Publish button in the modal
    const modalPublishButton = page
      .getByRole('dialog')
      .getByRole('button', { name: 'Publish' });
    await modalPublishButton.click();

    // Wait for modal to close and publish to complete
    await page
      .getByRole('dialog')
      .waitFor({ state: 'hidden', timeout: TIMEOUT });
    await page.waitForLoadState('networkidle');
  });

  // =====================================
  // User B: Access and Respond to Poll
  // =====================================
  test('RW-5 [User B] Respond to the poll', async ({ page }) => {
    await login(page, userB);

    // Navigate to the poll space
    expect(spaceUrl).not.toBe('');
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Navigate to poll tab
    const pollMenu = page.locator('[data-testid^="space-sidemenu-"]').filter({
      hasText: /poll/i,
    });
    if (await pollMenu.isVisible({ timeout: 5000 }).catch(() => false)) {
      await pollMenu.click();
      await page.waitForLoadState('networkidle');
    }

    // Check if we need to click Enter button to start the poll
    const enterButton = page.getByRole('button', { name: 'Enter' }).first();
    if (await enterButton.isVisible({ timeout: 3000 }).catch(() => false)) {
      await enterButton.click();
      await page.waitForLoadState('networkidle');
    }

    // Respond to the poll
    await respondToPoll(page, 0); // Select first option

    // Handle confirmation modal if it appears
    const confirmBtn = page.getByTestId('complete-survey-modal-btn-confirm');
    if (await confirmBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await confirmBtn.click();
    }

    await page.waitForLoadState('networkidle');
  });

  // =====================================
  // User B: Verify Reward in My Rewards
  // =====================================
  test('RW-6 [User B] Verify reward was received', async ({ page }) => {
    await login(page, userB);

    // Navigate to My Rewards page
    await navigateToMyRewards(page);

    // Verify the reward was received
    await verifyRewardReceived(page);
  });
});
