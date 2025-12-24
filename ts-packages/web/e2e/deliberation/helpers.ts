// Re-export all common helpers from shared location
export {
  login,
  logout,
  mobileLogin,
  TIMEOUT,
  verifyCredential,
  createTeam,
  clickTeamSidebarMenu,
  goToSpace,
  publishSpacePrivately,
  startDeliberation,
  inviteMembers,
  setupPanels,
  enableAnonymousParticipation,
  viewAnalysis,
  createDeliberationPost,
  replyToPost,
  writeNewPost,
  setEndTimeOneHourLater,
  createPrePollSurvey,
  createFinalSurvey,
  conductSurvey,
  goToFinalSurvey,
  createBoardPosts,
} from '../helpers';

import { expect, Page } from '@playwright/test';
import { POST_TITLE, TEAM_ID } from './data';
import {
  goToTeam as goToTeamBase,
  goToTeamSpace as goToTeamSpaceBase,
} from '../helpers';

// Deliberation-specific wrappers that use TEAM_ID from data
export async function goToTeam(page: Page) {
  await goToTeamBase(page, TEAM_ID);
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

  await page.waitForURL(/.*/, { waitUntil: 'networkidle', timeout: 10000 });

  await expect(page.getByTestId('space-card').first()).toBeVisible();
  await expect(page.getByTestId('space-card').first()).toContainText(
    POST_TITLE,
  );
}
