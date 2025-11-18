import { test, expect, Page } from "@playwright/test";
import {
  ADMIN_ACCOUNTS,
  BASE_URL,
  DESKTOP_VIEWPORT,
  TIMEOUT,
  USER_ACCOUNTS,
} from "../configs";
import { clickTeamSidebarMenu, login, setEndTimeOneHourLater } from "./helpers";
import {
  BOARD_POSTS,
  POST_CONTENT,
  POST_TITLE,
  SURVEY_QUESTIONS,
  TEAM_DESCRIPTION,
  TEAM_ID,
  TEAM_NAME,
  YOUTUBE_LINK,
} from "./constants";
test.describe("Admin - Deliberation E2E Tests", () => {
  test.use({ viewport: DESKTOP_VIEWPORT });

  test("[DELIB-ADMIN-001] Admin Create Deliberation", async ({ page }) => {
    // 1. Login as admin
    await login(
      page,
      ADMIN_ACCOUNTS.ADMIN1.email,
      ADMIN_ACCOUNTS.ADMIN1.password
    );
    const timeId = TEAM_ID(Date.now());
    // 2. Create team
    await page.locator('[data-pw="team-selector-trigger"]').click();
    await page.locator('[data-pw="open-team-creation-popup"]').click();

    await page.locator('[data-pw="team-nickname-input"]').fill(TEAM_NAME);

    await page.locator('[data-pw="team-username-input"]').fill(timeId);
    await page
      .locator('[data-pw="team-description-input"]')
      .fill(TEAM_DESCRIPTION);
    await page.locator('[data-pw="team-create-button"]').click();

    // Wait for team page
    await page.waitForURL(`/teams/${timeId}/home`, { timeout: TIMEOUT });
    await expect(page.getByRole("button", { name: TEAM_NAME })).toBeVisible();

    // // 3. Invite team member
    // await clickSidebarMenu(page, "groups");
    // const inviteButton = page.locator('[data-pw="invite-member-button"]');
    // await inviteButton.waitFor({ state: "visible", timeout: TIMEOUT });
    // await inviteButton.click();
    // const inviteInput = page.getByTestId("invite-member-search-input");
    // await inviteInput.waitFor({ state: "visible", timeout: TIMEOUT });
    // await inviteInput.fill(ADMIN_ACCOUNTS.ADMIN2.email);
    // await page.keyboard.press("Enter");

    // await page
    //   .getByText(ADMIN_ACCOUNTS.ADMIN2.username)
    //   .waitFor({ state: "visible", timeout: TIMEOUT });

    // const inviteSendButton = page.getByTestId("send-invite-button");
    // await inviteSendButton.click();

    // const popupCloseButton = page.getByTestId("popup-close-button");
    // await popupCloseButton.click();

    // 4. Create post in team
    // sidemenu-team-drafts
    await clickTeamSidebarMenu(page, "drafts");
    await page.getByTestId("create-post-button").click();

    // Fill post details
    const titleInput = page.getByPlaceholder("Title");
    await titleInput.fill(POST_TITLE);

    const editor = page.locator(
      '[data-pw="post-content-editor"] [contenteditable]'
    );
    await editor.waitFor({ state: "visible" });
    await editor.click();
    await editor.fill(`${POST_CONTENT}\n`);
    await page.keyboard.press("Enter");

    page.once("dialog", async (dialog) => {
      expect(dialog.type()).toBe("prompt");
      expect(dialog.message()).toBe("Input Link URL");
      await dialog.accept(YOUTUBE_LINK);
    });

    // Open Alert
    await page.getByTestId("tiptap-toolbar-link").click();
    await page.waitForTimeout(3000);

    const skipSpaceCheckbox = page.locator('label[for="skip-space"]');
    const isChecked = await page.locator("#skip-space").isChecked();
    if (isChecked) {
      await skipSpaceCheckbox.click();
    }

    await page
      .locator('[aria-label="space-setting-form-deliberation.label"]')
      .click();

    // Create Post
    await page.getByTestId("publish-post-button").click();

    // 5. Create board posts
    await page.getByTestId("space-sidemenu-boards").click();
    await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
    await page.waitForLoadState("networkidle");

    for (const post of BOARD_POSTS) {
      const createButton = page.getByTestId("board-btn-create-board");
      await createButton.waitFor({ state: "visible", timeout: TIMEOUT });
      await createButton.click({ force: true });

      // Wait for navigation to create page
      await page.waitForURL(/.*\/create$/, { timeout: TIMEOUT });
      await page.waitForLoadState("networkidle");

      await setEndTimeOneHourLater(page);

      const title = page.getByTestId("board-title-input");
      await title.waitFor({ state: "visible", timeout: TIMEOUT });
      await title.fill(post.title);

      // Wait for title to be filled
      await page.waitForTimeout(500);

      const categoryInput = page.getByTestId("board-category-input");
      await categoryInput.waitFor({ state: "visible", timeout: TIMEOUT });
      await categoryInput.fill(post.category);
      await page.waitForTimeout(300);
      await page.keyboard.press("Enter");
      await page.waitForTimeout(300);
      await page.keyboard.press("Enter");

      // Wait for category to be set
      await page.waitForTimeout(500);

      const editor = page.locator(
        '[data-pw="space-board-content-editor"] .ProseMirror'
      );
      await editor.waitFor({ state: "visible", timeout: TIMEOUT });
      await editor.click();
      await page.waitForTimeout(300);
      await editor.fill(post.content);

      // Wait for content to be filled
      await page.waitForTimeout(500);

      const submitButton = page.getByTestId("board-btn-submit");
      await submitButton.waitFor({ state: "visible", timeout: TIMEOUT });
      await submitButton.click();

      // Wait for navigation back to boards list
      await page.waitForURL(/.*\/boards$/, { timeout: TIMEOUT });
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(1000);
    }

    // 6. Create poll
    await page.getByTestId("space-sidemenu-polls").click();
    await page.waitForURL(/.*\/polls$/, { timeout: TIMEOUT });
    await page.waitForLoadState("networkidle");

    await page.getByTestId("create-pre-poll-survey").click();
    await page.getByTestId("poll-btn-edit").click();

    // Add survey questions
    for (const question of SURVEY_QUESTIONS) {
      await page.getByTestId("poll-btn-add-question").click();

      // Select question type using Radix UI Select
      const trigger = page.getByTestId("poll-question-type-selector").last();
      await trigger.waitFor({ state: "visible", timeout: TIMEOUT });
      await trigger.click();

      const option = page.getByRole("option", { name: question.displayName });
      await option.waitFor({ state: "visible", timeout: TIMEOUT });
      await option.click();

      const questionTitleInput = page
        .getByTestId("poll-question-title-input")
        .last();
      await questionTitleInput.fill(question.title);

      if (question.required) {
        const requiredCheckbox = page
          .getByTestId("poll-question-required")
          .last();
        await requiredCheckbox.click();
      }
      if (question.options) {
        for (const option of question.options) {
          const optionInput = page.getByTestId("question-option-input").last();
          await optionInput.fill(option);
          if (option !== question.options[question.options.length - 1]) {
            await page.getByTestId("poll-answer-btn-add-option").last().click();
          }
        }
      }
    }

    await page.getByTestId("poll-btn-save").click();

    // 7. Invite members
    await page.getByTestId("space-sidemenu-members").click();
    await page.waitForURL(/.*\/members$/, { timeout: TIMEOUT });
    await page.waitForLoadState("networkidle");

    await page.getByTestId("invite-space-btn").click();

    const emails = [
      USER_ACCOUNTS.USER1.email,
      USER_ACCOUNTS.USER2.email,
      USER_ACCOUNTS.USER3.email,
      USER_ACCOUNTS.USER4.email,
      USER_ACCOUNTS.USER5.email,
      USER_ACCOUNTS.USER6.email,
      USER_ACCOUNTS.USER7.email,
      USER_ACCOUNTS.USER8.email,
    ];
    await page.getByTestId("member-email-input").fill(emails.join(", "));
    await page.keyboard.press("Enter");
    await page
      .getByText(USER_ACCOUNTS.USER8.username)
      .waitFor({ state: "visible", timeout: TIMEOUT });
    await page.getByTestId("invite-member-send-btn").click();

    // 8. Panel settings
    await page.getByTestId("space-sidemenu-panels").click();
    await page.waitForURL(/.*\/panels$/, { timeout: TIMEOUT });
    await page.waitForLoadState("networkidle");

    // Set maximum people quota
    const quotaInput = page.getByTestId("panel-quota-input");
    await quotaInput.click();
    quotaInput.getByRole("textbox").fill("60");

    const multiSelectTrigger = page.getByTestId("multi-select-trigger");
    await multiSelectTrigger.waitFor({ state: "visible", timeout: TIMEOUT });
    await multiSelectTrigger.click();

    // Wait for dropdown to open
    await page.waitForTimeout(500);

    // Select Gender
    const genderOption = page.getByTestId("multi-select-option-gender");
    await genderOption.waitFor({ state: "visible", timeout: TIMEOUT });
    await genderOption.click();
    await page.waitForTimeout(300);

    // Select University
    const universityOption = page.getByTestId("multi-select-option-university");
    await universityOption.waitFor({ state: "visible", timeout: TIMEOUT });
    await universityOption.click();
    await page.waitForTimeout(300);

    // Close the dropdown by clicking outside or pressing Escape
    await page.keyboard.press("Escape");
    await page.waitForTimeout(500);

    // 10. Enable anonymous participation
    await page.getByTestId("space-sidemenu-settings").click();
    await page.waitForURL(/.*\/settings$/);
    await page.waitForLoadState("networkidle");

    await page.getByTestId("anonymous-participation-label").click();
    await page.waitForTimeout(500);

    // 11. Publish the space
    await page.getByTestId("space-action-button").click();
    await page.getByTestId("selectable-card-private").click();

    await page.getByTestId("publish-space-modal-btn").click();
  });
});
