import { expect, Page } from '@playwright/test';
import { SURVEY_QUESTIONS } from './data';

const TIMEOUT = 10000;

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
  await page.waitForLoadState('networkidle');
  await page.waitForURL('/');
  await expect(page.getByText('My Spaces', { exact: true })).toBeVisible();
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
    let currentUrl = page.url();
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

export async function publishSpacePrivately(page: Page) {
  await page.getByTestId('space-action-button').click();
  await page.getByTestId('selectable-card-private').click();
  await page.getByTestId('publish-space-modal-btn').click();
  await page.waitForLoadState('networkidle');
}

export async function goToMySpaces(page: Page) {
  await page.getByText('My Spaces', { exact: true }).click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(100);
}

export async function participateInSpace(page: Page, spaceName: string) {
  await goToMySpaces(page);

  // Find and click on the space
  const spaceCard = page.getByText(spaceName).first();
  await spaceCard.waitFor({ state: 'visible', timeout: TIMEOUT });
  await spaceCard.click();
  await page.waitForLoadState('networkidle');

  // Check URL to determine where we are
  let currentUrl = page.url();
  let spaceMatch = currentUrl.match(/\/spaces\/([^/]+)/);

  if (!spaceMatch) {
    // Not on a space page, nothing to do
    return;
  }

  const spaceId = spaceMatch[1];

  // First, try to navigate to boards directly
  // If we can access it, we're already a participant
  await page.goto(`/spaces/${spaceId}/boards`);
  await page.waitForLoadState('networkidle');

  // Check if we actually got to boards (sidemenu-boards exists)
  const sideMenuBoards = page.getByTestId('space-sidemenu-boards');
  const onBoardsPage = (await sideMenuBoards.count()) > 0;

  if (onBoardsPage) {
    // We're on the boards page, we're already a participant
    return;
  }

  // Check if we landed on a survey page (has Next button or question counter)
  const nextButton = page.getByRole('button', { name: 'Next' });
  const hasSurveyContent = await nextButton
    .isVisible({ timeout: 1000 })
    .catch(() => false);

  if (hasSurveyContent) {
    // We're on a survey page - need to complete it
    await conductPrePollSurvey(page);
    return;
  }

  // If we're not on boards or survey, we might be on overview
  // Go to overview to click participate button
  await page.goto(`/spaces/${spaceId}`);
  await page.waitForLoadState('networkidle');

  // Check if participate button exists
  const participateBtn = page.getByTestId('participate-space-btn');
  const buttonCount = await participateBtn.count();

  if (buttonCount > 0) {
    // First-time participant - click to join
    await participateBtn.click();
    await page.waitForLoadState('networkidle');

    // Wait for auto-participation to complete and reload
    await page.waitForTimeout(1000);
    await page.reload();
    await page.waitForLoadState('networkidle');

    // Update URL after reload
    currentUrl = page.url();

    // Check if we need to complete a survey
    const isOnPollPage = currentUrl.includes('/poll');
    const surveyNextBtn = page.getByRole('button', { name: 'Next' });
    const onSurveyPage = await surveyNextBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);

    if (isOnPollPage || onSurveyPage) {
      await conductPrePollSurvey(page);
    }
  }
}

export async function conductPrePollSurvey(page: Page) {
  page.getByText(SURVEY_QUESTIONS[0].options![0], { exact: true });
}

export async function startDeliberation(page: Page) {
  // Try clicking the space action button which might open a dropdown
  const actionButton = page.getByTestId('space-action-button');
  if (await actionButton.isVisible()) {
    await actionButton.click();
    await page.waitForTimeout(300);

    // Try to find start deliberation button in dropdown
    const startBtn = page.getByTestId('start-deliberation-btn');
    if (await startBtn.isVisible()) {
      await startBtn.click();
    } else {
      // Fallback: click the Start button directly by text
      const startByText = page.getByRole('button', { name: 'Start' });
      if (await startByText.isVisible()) {
        await startByText.click();
      }
    }
  } else {
    // Fallback: click the Start button directly by text
    const startByText = page.getByRole('button', { name: 'Start' });
    await startByText.waitFor({ state: 'visible', timeout: 5000 });
    await startByText.click();
  }
  await page.waitForLoadState('networkidle');
}

export async function replyToPost(page: Page, replyContent: string) {
  await page.getByTestId('space-sidemenu-boards').click();
  await page.waitForLoadState('networkidle');

  // Click on first board post
  const firstPost = page.getByTestId('board-post-item').first();
  await firstPost.click();
  await page.waitForLoadState('networkidle');

  // Write reply
  const replyEditor = page.getByTestId('reply-editor');
  await replyEditor.click();
  await replyEditor.fill(replyContent);

  // Submit reply
  const submitReply = page.getByTestId('submit-reply-btn');
  await submitReply.click();
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

export async function conductFinalSurvey(page: Page) {
  // Check if we're already on the survey page (question counter visible)
  const questionCounter = page.locator('text=/\\d+ \\/ \\d+/');
  const isOnSurvey = await questionCounter.isVisible().catch(() => false);

  if (!isOnSurvey) {
    await page.getByTestId('space-sidemenu-polls').click();
    await page.waitForLoadState('networkidle');

    // Click on final survey
    const finalSurveyCard = page.getByTestId('final-survey-card');
    await finalSurveyCard.waitFor({ state: 'visible', timeout: TIMEOUT });
    await finalSurveyCard.click();
    await page.waitForLoadState('networkidle');
  }

  // Answer all questions in the survey
  // Similar to pre-poll survey
  let maxIterations = 15;
  while (maxIterations > 0) {
    maxIterations--;

    // Check for error state (e.g., "Cannot participate in survey")
    const cannotParticipate = page.getByText('Cannot participate in survey');
    const noPermission = page.getByText('do not have permission');
    if (
      (await cannotParticipate.isVisible().catch(() => false)) ||
      (await noPermission.isVisible().catch(() => false))
    ) {
      // User cannot participate in this survey - exit gracefully
      return;
    }

    // Check available navigation buttons
    const submitBtn = page.getByTestId('survey-btn-submit');
    const nextBtn = page.getByRole('button', { name: 'Next' });
    const isLastQuestion = await submitBtn.isVisible();
    const hasNextButton = await nextBtn.isVisible().catch(() => false);

    // Try to click an unselected option first
    let clickedOption = false;

    // Find all buttons that might be survey options
    const allButtons = await page.locator('button').all();
    for (const btn of allButtons) {
      try {
        const ariaPressed = await btn.getAttribute('aria-pressed');
        // Also check the 'pressed' attribute as some buttons use this directly
        const pressed = await btn.getAttribute('pressed');
        // Check if button is pressed using either attribute
        // Note: ariaPressed could be 'true', 'false', or null (not set)
        const isPressed = ariaPressed === 'true' || pressed !== null;

        if (!isPressed) {
          const text = await btn.textContent();
          const isNavButton =
            text &&
            /(Next|Prev|EN|Light|Home|Network|Membership|Report|Storybook|Log out|Create a team)/i.test(
              text,
            );

          if (!isNavButton) {
            // Survey option buttons have no text content
            const isEmpty = !text || text.trim() === '';

            if (isEmpty) {
              await btn.click();
              clickedOption = true;
              await page.waitForTimeout(500);
              break;
            }
          }
        }
      } catch {
        // Button may have been removed, continue
      }
    }

    // If no option was clicked, try clicking Next button
    if (!clickedOption && hasNextButton) {
      await nextBtn.click();
      await page.waitForTimeout(300);
      continue;
    }

    // If it's the last question, click submit and exit
    if (isLastQuestion) {
      await page.waitForTimeout(300);
      const finalSubmit = page.getByTestId('survey-btn-submit');
      if (await finalSubmit.isEnabled()) {
        await finalSubmit.click();
        await page.waitForLoadState('networkidle');
      }
      break;
    }

    // If no progress was made, try clicking Next anyway
    if (!clickedOption && !isLastQuestion) {
      const nextBtnRetry = page.getByRole('button', { name: 'Next' });
      if (await nextBtnRetry.isVisible().catch(() => false)) {
        await nextBtnRetry.click();
        await page.waitForTimeout(300);
      }
    }

    await page.waitForTimeout(200);
  }

  // Wait for and close the completion modal
  try {
    const confirmBtn = page.getByTestId('complete-survey-modal-btn-confirm');
    await confirmBtn.waitFor({ state: 'visible', timeout: 5000 });
    await confirmBtn.click();
    await page.waitForLoadState('networkidle');
    // Wait a bit more to ensure the survey completion is recorded
    await page.waitForTimeout(500);
  } catch {
    // Modal didn't appear, survey was already completed or different flow
  }
}

export async function viewAnalysis(page: Page) {
  await page.getByTestId('space-sidemenu-analysis').click();
  await page.waitForLoadState('networkidle');

  // Verify analysis page loaded
  await expect(page.getByTestId('analysis-container')).toBeVisible();
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
  await page.waitForURL('/');
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
  await endTimeButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await endTimeButton.click();
  await page.waitForTimeout(500);

  // Select the time option
  const timeOption = page.getByText(endTimeText, { exact: true });
  await timeOption.waitFor({ state: 'visible', timeout: TIMEOUT });
  await timeOption.click();
  await page.waitForTimeout(500);

  // Verify the time was set
  await expect(endTimeButton).toContainText(endTimeText);
}
