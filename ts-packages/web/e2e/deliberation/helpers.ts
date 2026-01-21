import { expect, Page } from '@playwright/test';
import { POST_TITLE, TEAM_ID } from './data';

export const TIMEOUT = 10000;

// Helper functions
export async function login(
  page: Page,
  { email, password }: { email: string; password: string },
) {
  // Clear cookies to ensure a fresh session
  await page.context().clearCookies();

  // Navigate to the page first, then clear storage
  await page.goto('/');
  await page.waitForLoadState('networkidle');

  // Clear storage after navigation
  await page.evaluate(() => {
    localStorage.clear();
    sessionStorage.clear();
  });

  // Reload page to apply cleared storage
  await page.reload();
  await page.waitForLoadState('networkidle');

  // Now sign in
  const signInButton = page.getByRole('button', { name: /sign in/i });
  await signInButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await signInButton.click();
  await page.getByTestId('email-input').fill(email);
  await page.getByTestId('continue-button').click();
  await page.getByTestId('password-input').fill(password);
  await page.getByTestId('continue-button').click();

  await page.getByRole('dialog').waitFor({
    state: 'detached',
    timeout: TIMEOUT,
  });

  await page.waitForURL('/', {
    waitUntil: 'networkidle',
    timeout: TIMEOUT,
  });
}

export async function logout(page: Page) {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  // Click user menu and logout
  const userMenu = page.getByTestId('user-menu-trigger');
  if (await userMenu.isVisible()) {
    await userMenu.click();
    await page.getByTestId('logout-button').click();
    await page.waitForLoadState('networkidle');
  }
}

export async function verifyCredential(page: Page, code: string) {
  await page.goto('/credentials');
  await page.waitForLoadState('networkidle');

  // Click the verify button to open method selection modal
  const verifyButton = page.getByTestId('credential-verify-button');
  await verifyButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await verifyButton.click();

  // Select code verification option
  const codeVerificationOption = page.getByTestId('code-verification-option');
  await codeVerificationOption.waitFor({ state: 'visible', timeout: TIMEOUT });
  await codeVerificationOption.click();

  // Enter the code
  const codeInput = page.getByTestId('credential-code-input');
  await codeInput.waitFor({ state: 'visible', timeout: TIMEOUT });
  await codeInput.fill(code);

  // Submit the code
  const submitButton = page.getByTestId('credential-code-submit');
  await submitButton.click();

  // Wait for verification to complete
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(500);
}

export async function goToTeam(page: Page) {
  await page.goto(`/teams/${TEAM_ID}/home`, { waitUntil: 'networkidle' });
}

export async function goToTeamSpace(page: Page) {
  await page.goto(`/teams/${TEAM_ID}/draft`, { waitUntil: 'networkidle' });
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

export async function createDeliberationPost(
  page: Page,
  title: string,
  content: string,
  youtubeLink?: string,
) {
  await clickTeamSidebarMenu(page, 'drafts');
  await page.getByTestId('create-post-button').click();

  const titleInput = page.getByPlaceholder('Title');
  await titleInput.fill(title);

  const editor = page.locator(
    '[data-pw="post-content-editor"] [contenteditable]',
  );
  await editor.waitFor({ state: 'visible' });
  await editor.click();
  await editor.fill(`${content}\n`);
  await page.keyboard.press('Enter');

  if (youtubeLink) {
    page.once('dialog', async (dialog) => {
      expect(dialog.type()).toBe('prompt');
      expect(dialog.message()).toBe('Input Link URL');
      await dialog.accept(youtubeLink);
    });
    await page.getByTestId('tiptap-toolbar-link').click();
    await page.waitForTimeout(1000);
  }

  // Enable deliberation space
  const skipSpaceCheckbox = page.locator('label[for="skip-space"]');
  const isChecked = await page.locator('#skip-space').isChecked();
  if (isChecked) {
    await skipSpaceCheckbox.click();
  }
  await page.waitForTimeout(100);

  await page
    .locator('[aria-label="space-setting-form-deliberation.label"]')
    .click();
  await page.waitForTimeout(100);

  await page.getByTestId('publish-post-button').click();
  await page.waitForLoadState('networkidle');
}

export async function createBoardPosts(
  page: Page,
  posts: Array<{ category: string; title: string; content: string }>,
) {
  await page.getByTestId('space-sidemenu-boards').click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  for (const post of posts) {
    // Try to find the create button by testid first, fallback to role
    let createButton = page.getByTestId('board-btn-create-board');
    if (!(await createButton.isVisible())) {
      createButton = page.getByRole('button', { name: /create new post/i });
    }
    await createButton.waitFor({ state: 'visible', timeout: TIMEOUT });

    // Click the button using multiple approaches to ensure it triggers
    await createButton.click({ force: true });
    await page.waitForTimeout(500);

    // Check if navigation happened, if not, try clicking via JavaScript
    const currentUrl = page.url();
    if (!currentUrl.includes('/create')) {
      // Try clicking via JavaScript
      await page.evaluate(() => {
        const btn = document.querySelector(
          '[data-testid="board-btn-create-board"]',
        ) as HTMLButtonElement;
        if (btn) btn.click();
      });
      await page.waitForTimeout(500);
    }

    // Wait for navigation
    await page.waitForURL(/.*\/boards\/create/, { timeout: 30000 });
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // Set end time (TimeRangeSetting should be visible)
    await setEndTimeOneHourLater(page);

    const title = page.getByTestId('board-title-input');
    await title.waitFor({ state: 'visible', timeout: TIMEOUT });
    await title.fill(post.title);
    await page.waitForTimeout(500);

    const categoryInput = page.getByTestId('board-category-input');
    await categoryInput.waitFor({ state: 'visible', timeout: TIMEOUT });
    await categoryInput.fill(post.category);
    await page.waitForTimeout(300);
    await page.keyboard.press('Enter');
    await page.waitForTimeout(300);
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);

    const editor = page.locator(
      '[data-pw="space-board-content-editor"] .ProseMirror',
    );
    await editor.waitFor({ state: 'visible', timeout: TIMEOUT });
    await editor.click();
    await page.waitForTimeout(300);
    await editor.fill(post.content);
    await page.waitForTimeout(500);

    const submitButton = page.getByTestId('board-btn-submit');
    await submitButton.waitFor({ state: 'visible', timeout: TIMEOUT });
    await submitButton.click();

    await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  }
}

export async function createPrePollSurvey(
  page: Page,
  questions: Array<{
    type: string;
    displayName: string;
    required: boolean;
    title: string;
    options?: string[];
  }>,
) {
  await page.getByTestId('space-sidemenu-polls').click();
  await page.waitForURL(/.*\/polls$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  await page.getByTestId('create-pre-poll-survey').click();
  await page.getByTestId('poll-btn-edit').click();

  for (const question of questions) {
    await page.getByTestId('poll-btn-add-question').click();

    const trigger = page.getByTestId('poll-question-type-selector').last();
    await trigger.waitFor({ state: 'visible', timeout: TIMEOUT });
    await trigger.click();

    const option = page.getByRole('option', { name: question.displayName });
    await option.waitFor({ state: 'visible', timeout: TIMEOUT });
    await option.click();

    const questionTitleInput = page
      .getByTestId('poll-question-title-input')
      .last();
    await questionTitleInput.fill(question.title);

    if (question.required) {
      const requiredCheckbox = page
        .getByTestId('poll-question-required')
        .last();
      await requiredCheckbox.click();
    }

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

  await page.getByTestId('poll-btn-save').click();
  await page.waitForLoadState('networkidle');
}

export async function createFinalSurvey(
  page: Page,
  questions: Array<{
    type: string;
    displayName: string;
    required: boolean;
    title: string;
    options?: string[];
  }>,
) {
  await page.getByTestId('space-sidemenu-polls').click();
  await page.waitForURL(/.*\/polls$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(1000);

  // Wait for poll creation button to be visible
  const createPollButton = page.getByTestId('create-poll-button');
  await expect(createPollButton).toBeVisible({ timeout: TIMEOUT });
  await createPollButton.click();

  await page.getByTestId('create-final-survey').click();
  await page.getByTestId('poll-btn-edit').click();

  for (const question of questions) {
    await page.getByTestId('poll-btn-add-question').click();

    const trigger = page.getByTestId('poll-question-type-selector').last();
    await trigger.waitFor({ state: 'visible', timeout: TIMEOUT });
    await trigger.click();

    const option = page.getByRole('option', { name: question.displayName });
    await option.waitFor({ state: 'visible', timeout: TIMEOUT });
    await option.click();

    const questionTitleInput = page
      .getByTestId('poll-question-title-input')
      .last();
    await questionTitleInput.fill(question.title);

    if (question.required) {
      const requiredCheckbox = page
        .getByTestId('poll-question-required')
        .last();
      await requiredCheckbox.click();
    }

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

  await page.getByTestId('poll-btn-save').click();
  await page.waitForLoadState('networkidle');
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

  const trigger = page.getByTestId('multi-select-trigger');
  const popover = page.locator('[data-state="open"]').first();
  const triggerBox = await trigger.boundingBox();
  await expect(popover).toBeVisible();
  await page.waitForFunction(
    ({ el, w }) => {
      const node = document.querySelector(el) as HTMLElement | null;
      if (!node) return false;
      const cw = node.getBoundingClientRect().width;
      return Math.abs(cw - w) < 2;
    },
    { el: '[data-state="open"]', w: triggerBox?.width ?? 0 },
  );

  await page.waitForTimeout(150);
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

export async function publishSpacePrivately(page: Page) {
  await page.getByTestId('space-action-button').click();
  await page.getByTestId('selectable-card-private').click();
  await page.getByTestId('publish-space-modal-btn').click();
  await page.waitForLoadState('networkidle');
  await expect(page.getByTestId('space-action-button')).toHaveText('Start');
}

export async function goToMySpaces(page: Page) {
  await page.getByText('My Spaces', { exact: true }).click();

  await page.waitForURL(/.*/, { waitUntil: 'networkidle', timeout: TIMEOUT });

  await expect(page.getByTestId('space-card').first()).toBeVisible();
  await expect(page.getByTestId('space-card').first()).toContainText(
    POST_TITLE,
  );
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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function conductSurvey(page: Page, answers: any[]) {
  // Wait for survey to load
  const firstOption = page.getByTestId('objective-viewer-option').first();
  await expect(firstOption).toBeVisible({ timeout: TIMEOUT });
  await page.waitForTimeout(500);

  await page.getByTestId('objective-viewer-option').nth(answers[0]).click();
  await page.waitForTimeout(500);

  await page.getByTestId('objective-viewer-option').nth(answers[1]).click();
  await page.waitForTimeout(500);

  await page.getByTestId('objective-viewer-option').nth(answers[2]).click();
  await page.waitForTimeout(500);
  await page.getByTestId('objective-viewer-option').nth(answers[3]).click();
  await page.waitForTimeout(500);
  await page.getByTestId('survey-btn-next').click();
  await page.waitForTimeout(500);

  await page
    .getByPlaceholder('Please share your opinion.', { exact: true })
    .fill(answers[4]);
  await page.waitForTimeout(500);
  await page.getByTestId('survey-btn-next').click();
  await page.waitForTimeout(500);

  await page
    .getByPlaceholder('Please share your opinion.', { exact: true })
    .fill(answers[5]);
  await page.waitForTimeout(500);
  await page.getByTestId('survey-btn-submit').click();
  await page.waitForTimeout(500);
}

export async function startDeliberation(page: Page) {
  // Try clicking the space action button which might open a dropdown

  const actionButton = page.getByTestId('space-action-button');
  await expect(actionButton).toHaveText('Start');
  await actionButton.click();

  const startSpaceButton = page.getByTestId('start-space-button');
  await expect(startSpaceButton).toBeVisible();
  await startSpaceButton.click();
  await expect(page.getByText('Success to start space.')).toBeVisible();
}

export async function replyToPost(page: Page, replyContent: string) {
  // Wait for the boards menu item to be visible
  const boardsMenuItem = page.getByTestId('space-sidemenu-boards');
  await boardsMenuItem.waitFor({ state: 'visible', timeout: 15000 });
  await boardsMenuItem.click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  // Force a reload to ensure boards list loads fresh data
  await page.reload({ waitUntil: 'networkidle' });
  await page.waitForTimeout(1000);

  // Wait for board posts to load with a longer timeout
  const firstPost = page.getByTestId('board-post-item').first();
  await expect(firstPost).toBeVisible({ timeout: 15000 });

  // Add a small delay to ensure the post is fully interactive
  await page.waitForTimeout(500);

  await firstPost.click();
  await page.waitForLoadState('networkidle');

  // Write reply
  await page.getByTestId('open-new-comment-box-button').click();
  await page.waitForTimeout(300);

  const commentEditor = page.locator('[data-pw="comment-editor"]');
  await commentEditor
    .getByTestId('tiptap-editor-content')
    .first()
    .locator('[contenteditable="true"]')
    .first()
    .fill(replyContent);

  // Submit reply
  await page.getByLabel('Publish', { exact: true }).click();
  await page.waitForLoadState('networkidle');
}

export async function writeNewPost(
  page: Page,
  title: string,
  content: string,
  category: string,
) {
  await page.getByTestId('space-sidemenu-boards').click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');

  const createButton = page.getByTestId('board-btn-create-board');
  await createButton.click();

  await page.waitForURL(/.*\/create$/, { timeout: TIMEOUT });
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(1000);

  await setEndTimeOneHourLater(page);

  const titleInput = page.getByTestId('board-title-input');
  await titleInput.fill(title);

  const categoryInput = page.getByTestId('board-category-input');
  await categoryInput.fill(category);
  await page.keyboard.press('Enter');
  await page.keyboard.press('Enter');

  const editor = page.locator(
    '[data-pw="space-board-content-editor"] .ProseMirror',
  );
  await editor.click();
  await editor.fill(content);

  await page.getByTestId('board-btn-submit').click();
  await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
}

export async function goToFinalSurvey(page: Page) {
  await page.getByTestId('space-sidemenu-polls').click();

  // Wait for FINAL SURVEY text to be visible
  const finalSurveyCard = page.getByTestId('FINAL SURVEY');
  await expect(finalSurveyCard).toBeVisible();

  // Click the first Enter button (FINAL SURVEY is listed first)
  const enterButton = finalSurveyCard
    .getByRole('button', { name: 'Enter' })
    .first();
  await expect(enterButton).toBeVisible();
  await enterButton.click();
  await expect(
    page.getByTestId('objective-viewer-option').first(),
  ).toBeVisible();
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

export async function mobileLogin(page: Page, email: string, password: string) {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  const menuButton = page.getByTestId('mobile-menu-toggle');
  await menuButton.waitFor({ state: 'visible' });
  await menuButton.click();

  const signInButton = page.getByRole('button', { name: /sign in/i });
  await signInButton.waitFor({ state: 'visible' });
  await signInButton.click();

  await page.getByTestId('email-input').fill(email);
  await page.getByTestId('continue-button').click();
  await page.getByTestId('password-input').fill(password);
  await page.getByTestId('continue-button').click();

  // Wait for login dialog to be completely removed from DOM
  await page.getByRole('dialog').waitFor({
    state: 'detached',
    timeout: TIMEOUT,
  });

  // Wait for navigation to complete with network idle
  await page.waitForURL('/', {
    waitUntil: 'networkidle',
    timeout: TIMEOUT,
  });
}

export async function clickTeamSidebarMenu(page: Page, menuName: string) {
  const menu = page.getByTestId(`sidemenu-team-${menuName}`);
  await menu.waitFor({ state: 'visible' });
  await menu.click();
  await page.waitForLoadState('networkidle');
}

export async function setEndTimeOneHourLater(page: Page) {
  // Calculate current time and 1 hour later
  const now = new Date();
  const oneHourLater = new Date(now.getTime() + 60 * 60 * 1000);

  // Format time for selection (e.g., "02:00 PM")
  const endHour = oneHourLater.getHours();
  const endHour12 = endHour % 12 || 12;
  const endPeriod = endHour < 12 ? 'AM' : 'PM';
  const endTimeText = `${endHour12.toString().padStart(2, '0')}:00 ${endPeriod}`;

  // Click end time dropdown - wait with explicit timeout
  const endTimeButton = page.getByTestId('time-end-dropdown');
  await endTimeButton.waitFor({ state: 'visible', timeout: 15000 });
  await endTimeButton.click();
  await page.waitForTimeout(500);

  // Select the time option
  const timeOption = page.getByText(endTimeText, { exact: true });
  await timeOption.waitFor({ state: 'visible', timeout: 15000 });
  await timeOption.click();
  await page.waitForTimeout(500);

  // Verify the time was set
  await expect(endTimeButton).toContainText(endTimeText);
}
