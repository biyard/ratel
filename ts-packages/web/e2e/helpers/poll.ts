import { expect, Page } from '@playwright/test';
import { TIMEOUT } from './auth';
import { setEndTimeOneHourLater } from './post';

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

export async function createPollQuestions(
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

  // Click on the default poll to edit it
  const pollCard = page.getByTestId('poll-card').first();
  await pollCard.waitFor({ state: 'visible', timeout: TIMEOUT });
  await pollCard.click();

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
