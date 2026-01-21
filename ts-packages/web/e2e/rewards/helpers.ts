import { expect, Page } from '@playwright/test';

// Re-export common helpers from deliberation
export {
  login,
  logout,
  createTeam,
  createDeliberationPost,
  createPrePollSurvey,
  goToMySpaces,
  goToSpace,
  conductSurvey,
  clickTeamSidebarMenu,
  TIMEOUT,
} from '../deliberation/helpers';

// Reward-specific helpers

export async function navigateToRewardsTab(page: Page) {
  // Navigate to rewards tab using side menu
  const rewardsMenu = page.locator('[data-testid^="space-sidemenu-"]').filter({
    hasText: /reward/i,
  });
  if (await rewardsMenu.isVisible()) {
    await rewardsMenu.click();
  } else {
    // Fallback: navigate by URL
    const currentUrl = page.url();
    const spacePk = currentUrl.match(/spaces\/([^/]+)/)?.[1];
    if (spacePk) {
      await page.goto(`/spaces/${spacePk}/rewards`);
    }
  }
  await page.waitForURL(/.*\/rewards$/, { timeout: 15000 });
  await page.waitForLoadState('networkidle');
}

export async function addPollReward(
  page: Page,
  credits: number,
  description?: string,
) {
  // Wait for page to fully load
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(1000);

  // Click Add Reward button
  const addButton = page.getByRole('button', { name: 'Add Reward' });
  await addButton.waitFor({ state: 'visible', timeout: 15000 });
  await addButton.click();

  // Wait for modal to open and animate
  await page.waitForTimeout(1000);

  // Fill credits - use spinbutton role since it's a number input
  const creditsInput = page.getByRole('spinbutton');
  await creditsInput.waitFor({ state: 'visible', timeout: 15000 });
  await creditsInput.click();
  await creditsInput.fill(credits.toString());

  // Fill description if provided
  if (description) {
    const descInput = page.getByPlaceholder(/reward description/i);
    await descInput.fill(description);
  }

  // Save reward
  const saveButton = page.getByRole('button', { name: 'Save' });
  await saveButton.waitFor({ state: 'visible', timeout: 10000 });
  await saveButton.click();

  // Wait for modal to close (dialog should disappear)
  const dialog = page.getByRole('dialog');
  await dialog.waitFor({ state: 'hidden', timeout: 15000 });

  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(1000);
}

export async function respondToPoll(page: Page, answerIndex: number) {
  // Wait for poll options to be visible
  const option = page.getByTestId('objective-viewer-option').nth(answerIndex);
  await option.waitFor({ state: 'visible', timeout: 15000 });
  await option.click();
  await page.waitForTimeout(500);

  // Submit response
  const submitBtn = page.getByTestId('survey-btn-submit');
  await submitBtn.waitFor({ state: 'visible', timeout: 15000 });
  await submitBtn.click();
  await page.waitForLoadState('networkidle');
}

export async function navigateToMyRewards(page: Page) {
  await page.goto('/rewards');
  await page.waitForLoadState('networkidle');
  const rewardsPage = page.getByTestId('my-rewards-page');
  await rewardsPage.waitFor({ state: 'visible', timeout: 15000 });
}

export async function verifyRewardReceived(page: Page) {
  // Check points summary card is visible
  const pointsCard = page.getByTestId('points-summary-card');
  await expect(pointsCard).toBeVisible();

  // Check user points value is visible
  const pointsValue = page.getByTestId('user-points-value');
  await expect(pointsValue).toBeVisible();

  // Check transaction list has at least one item (if transactions exist)
  const transactionList = page.getByTestId('transaction-list');
  const hasTransactions = await transactionList.isVisible();

  if (hasTransactions) {
    const transactionItem = page.getByTestId('transaction-item').first();
    await expect(transactionItem).toBeVisible();
  }
}

export async function createPostWithPollSpace(
  page: Page,
  teamId: string,
  title: string,
  content: string,
) {
  // Navigate to team page first
  await page.goto(`/teams/${teamId}/home`, { waitUntil: 'networkidle' });

  // Navigate to drafts using sidebar menu
  const draftsMenu = page.getByTestId('sidemenu-team-drafts');
  await draftsMenu.waitFor({ state: 'visible', timeout: 15000 });
  await draftsMenu.click();
  await page.waitForLoadState('networkidle');

  // Click create post button
  await page.getByTestId('create-post-button').click();
  await page.waitForLoadState('networkidle');

  // Fill title
  const titleInput = page.getByPlaceholder('Title');
  await titleInput.waitFor({ state: 'visible', timeout: 15000 });
  await titleInput.fill(title);

  // Fill content
  const editor = page.locator(
    '[data-pw="post-content-editor"] [contenteditable]',
  );
  await editor.waitFor({ state: 'visible' });
  await editor.click();
  await editor.fill(`${content}\n`);
  await page.keyboard.press('Enter');

  // Enable space creation (uncheck skip-space if checked)
  const skipSpaceCheckbox = page.locator('label[for="skip-space"]');
  const isChecked = await page.locator('#skip-space').isChecked();
  if (isChecked) {
    await skipSpaceCheckbox.click();
  }
  await page.waitForTimeout(100);

  // Select Poll space type
  await page.locator('[aria-label="space-setting-form-poll.label"]').click();
  await page.waitForTimeout(100);

  // Publish post
  await page.getByTestId('publish-post-button').click();
  await page.waitForLoadState('networkidle');

  // Wait for redirect to space
  await page.waitForURL(/.*\/spaces\/.*/, { timeout: 15000 });

  return page.url();
}

export async function createSimplePoll(
  page: Page,
  questions: Array<{
    type: string;
    displayName: string;
    required: boolean;
    title: string;
    options?: string[];
  }>,
) {
  // Navigate to polls
  const pollMenu = page.locator('[data-testid^="space-sidemenu-"]').filter({
    hasText: /poll/i,
  });
  if (await pollMenu.isVisible()) {
    await pollMenu.click();
  } else {
    // Try clicking poll tab by URL navigation
    const currentUrl = page.url();
    const spacePk = currentUrl.match(/spaces\/([^/]+)/)?.[1];
    if (spacePk) {
      const pollPk = `SPACE_POLL%23${spacePk.split('%23')[1] || spacePk.split('#')[1]}`;
      await page.goto(`/spaces/${spacePk}/polls/${pollPk}`);
    }
  }
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(500);

  // Click edit button to start editing poll
  const editBtn = page.getByTestId('poll-btn-edit');
  await editBtn.waitFor({ state: 'visible', timeout: 15000 });
  await editBtn.click();
  await page.waitForTimeout(500);

  for (const question of questions) {
    // Add question
    await page.getByTestId('poll-btn-add-question').click();
    await page.waitForTimeout(300);

    // Select question type
    const trigger = page.getByTestId('poll-question-type-selector').last();
    await trigger.waitFor({ state: 'visible', timeout: 15000 });
    await trigger.click();

    const option = page.getByRole('option', { name: question.displayName });
    await option.waitFor({ state: 'visible', timeout: 15000 });
    await option.click();

    // Fill question title
    const questionTitleInput = page
      .getByTestId('poll-question-title-input')
      .last();
    await questionTitleInput.fill(question.title);

    // Set required if needed
    if (question.required) {
      const requiredCheckbox = page
        .getByTestId('poll-question-required')
        .last();
      await requiredCheckbox.click();
    }

    // Add options for choice questions
    if (question.options) {
      for (const opt of question.options) {
        const optionInput = page.getByTestId('question-option-input').last();
        await optionInput.fill(opt);
        if (opt !== question.options[question.options.length - 1]) {
          await page.getByTestId('poll-answer-btn-add-option').last().click();
        }
      }
    }
  }

  // Save poll
  await page.getByTestId('poll-btn-save').click();
  await page.waitForLoadState('networkidle');
}
