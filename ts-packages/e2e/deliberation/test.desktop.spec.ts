import { test, expect, Page } from "@playwright/test";
import {
  ADMIN_ACCOUNTS,
  BASE_URL,
  DESKTOP_VIEWPORT,
  TIMEOUT,
  USER_ACCOUNTS,
} from "../configs";
// Team and post details
const TEAM_NAME = "입법처 실증 준비 팀";
const TEAM_ID = (timestamp: number) => `iitp-poc-1-${timestamp}`;
const TEAM_DESCRIPTION =
  "국회입법조사처와 함께 진행하는 공론조사를 준비하는 팀입니다.";

const POST_TITLE =
  "리워드 스페이스의 보상 방식, 참여도 중심이 맞을까 품질 중심이 맞을까?";
const POST_CONTENT = `1️⃣ 배경 설명
Ratel은 사용자의 참여를 보상하는 RewardSpace 기능을 제공합니다.
현재 대부분의 스페이스에서는 활동 횟수(참여도) 를 기준으로 포인트가 분배되고 있습니다.
하지만 최근 일부 크리에이터들은 "참여의 품질을 더 중시해야 한다"고 주장하고 있습니다.

2️⃣ 쟁점
A안: 참여도 중심 분배
B안: 품질 중심 분배

3️⃣ 질문
Q1. 당신은 어떤 보상 기준이 더 적절하다고 생각하나요?
Q2. 품질 평가를 도입한다면, 누가 평가하는 것이 적절할까요?`;

const YOUTUBE_LINK = "https://www.youtube.com/watch?v=R2X4BJ1KNM4";

// Survey questions
const SURVEY_QUESTIONS = [
  {
    type: "single_choice",
    displayName: "Single Choice",
    required: true,
    title:
      "Ratel과 같은 온라인 공론장(토론·투표 플랫폼)에 참여해본 적이 있습니까?",
    options: [
      "자주 참여한다",
      "가끔 참여한다",
      "이름은 들어봤지만 참여한 적은 없다",
      "전혀 참여해본 적이 없다",
    ],
  },
  {
    type: "single_choice",
    displayName: "Single Choice",
    required: true,
    title: "공론조사에 참여할 때 가장 중요하다고 생각하는 요소는 무엇입니까?",
    options: [
      "다양한 의견이 공정하게 반영되는 구조",
      "토론 주제의 공익성",
      "참여에 따른 보상",
      "편리한 참여 환경 (UI, 시간 등)",
      "익명성과 개인정보 보호",
    ],
  },
  {
    type: "multiple_choice",
    displayName: "Multiple Choice",
    required: true,
    title:
      "당신이 공론조사에 적극적으로 참여하게 되는 이유를 모두 선택해주세요.",
    options: [
      "사회적 의사결정에 영향을 미칠 수 있어서",
      "자신의 의견이 기록으로 남는 것이 좋아서",
      "토론을 통해 새로운 시각을 얻을 수 있어서",
      "포인트·리워드 등 보상이 있어서",
      "친구나 커뮤니티의 추천으로",
    ],
  },
  {
    type: "short_answer",
    displayName: "Short Answer",
    required: true,
    title:
      "온라인 공론조사나 토론 플랫폼을 신뢰하기 위해 가장 필요한 요소는 무엇이라고 생각하나요?",
  },
  {
    type: "subjective",
    displayName: "Subjective",
    required: true,
    title:
      "당신이 직접 공론조사 주제를 제안할 수 있다면, 어떤 주제를 제안하고 싶나요?",
  },
];

// Board posts
const BOARD_POSTS = [
  {
    category: "보상 기준의 공정성과 효율성",
    title: "활동량 기준 보상이 공정하다는 이유",
    content:
      "RewardSpace는 '참여' 그 자체를 장려하기 위해 설계된 시스템입니다.",
  },
  {
    category: "보상 기준의 공정성과 효율성",
    title: "품질 중심 보상이 커뮤니티를 성숙하게 만든다",
    content: "양적인 참여보다 중요한 것은 기여의 깊이입니다.",
  },
  {
    category: "AI 평가와 사용자 자율성의 균형",
    title: "AI 평가 도입, 공정성 향상의 첫걸음",
    content: "AI는 감정이나 사적 이해관계가 없습니다.",
  },
  {
    category: "AI 평가와 사용자 자율성의 균형",
    title: "AI 평가가 자율성과 창의성을 제한할 수도 있다",
    content:
      "AI가 모든 참여를 수치화하고 등급을 매기기 시작하면, 사람들은 '잘 보이기 위한 발언'만 하게 될 위험이 있습니다.",
  },
];

// Helper functions
export async function login(page: Page, email: string, password: string) {
  await page.goto(BASE_URL);
  await page.getByRole("button", { name: /sign in/i }).click();
  await page.getByTestId("email-input").fill(email);
  await page.getByTestId("continue-button").click();
  await page.getByTestId("password-input").fill(password);
  await page.getByTestId("continue-button").click();
  await page.waitForURL(BASE_URL, { timeout: TIMEOUT });
}

async function clickSidebarMenu(page: Page, menuName: string) {
  const menu = page.getByTestId(`sidemenu-team-${menuName}`);

  await menu.waitFor({ state: "visible", timeout: TIMEOUT });
  await menu.click();
  await page.waitForLoadState("networkidle");
}

test.describe("Desktop - Deliberation E2E Tests", () => {
  test.use({ viewport: DESKTOP_VIEWPORT });

  test("[DELIB-DESK-001] Admin creates team", async ({ page }) => {
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
    await clickSidebarMenu(page, "drafts");
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

  // test("[DELIB-DESK-002] User participates and completes pre-survey", async ({
  //   page,
  // }) => {
  //   // 1. Login as user
  //   await login(page, USER_ACCOUNTS.USER1.email, USER_ACCOUNTS.USER1.password);

  //   // 2. Verify attribute code (assume already authenticated)
  //   // TODO: Navigate to attribute verification if needed

  //   // 3. Navigate to My Space
  //   await clickSidebarMenu(page, "my-space");

  //   // 4. Find and click pending space
  //   const pendingSpace = page.locator('[data-pw="pending-space"]').first();
  //   await pendingSpace.waitFor({ state: "visible", timeout: TIMEOUT });
  //   await expect(pendingSpace).toBeVisible();
  //   await pendingSpace.click();

  //   // 5. Complete pre-survey
  //   await expect(page.getByText(/사전조사/i)).toBeVisible();

  //   // Answer question 1 (single choice)
  //   await page.getByText("가끔 참여한다").click();

  //   // Answer question 2 (single choice)
  //   await page.getByText("다양한 의견이 공정하게 반영되는 구조").click();

  //   // Answer question 3 (multiple choice)
  //   await page.getByText("사회적 의사결정에 영향을 미칠 수 있어서").click();
  //   await page.getByText("토론을 통해 새로운 시각을 얻을 수 있어서").click();

  //   // Answer question 4 (short answer)
  //   const shortAnswerInput = page.locator('[data-pw="short-answer-input"]');
  //   await shortAnswerInput.fill("투명한 운영과 검증 가능한 시스템");

  //   // Answer question 5 (long answer)
  //   const longAnswerInput = page.locator('[data-pw="long-answer-input"]');
  //   await longAnswerInput.fill(
  //     "기후변화 대응을 위한 탄소세 도입에 대한 공론조사를 제안합니다."
  //   );

  //   // Submit survey
  //   await page.getByRole("button", { name: /제출/i }).click();

  //   // 6. Verify participation
  //   await expect(page.getByText(/참여 완료/i)).toBeVisible();

  //   // 7. Check "나의 응답" in Poll menu
  //   await page.getByRole("link", { name: /Poll/i }).click();
  //   await expect(page.getByText(/나의 응답/i)).toBeVisible();

  //   // 8. Verify board posts
  //   await page.getByRole("link", { name: /게시판/i }).click();
  //   await expect(
  //     page.getByText("활동량 기준 보상이 공정하다는 이유")
  //   ).toBeVisible();
  //   await expect(
  //     page.getByText("품질 중심 보상이 커뮤니티를 성숙하게 만든다")
  //   ).toBeVisible();
  //   await expect(
  //     page.getByText("AI 평가 도입, 공정성 향상의 첫걸음")
  //   ).toBeVisible();
  //   await expect(
  //     page.getByText("AI 평가가 자율성과 창의성을 제한할 수도 있다")
  //   ).toBeVisible();
  // });

  // test("[DELIB-DESK-004] Admin starts deliberation and new user cannot join", async ({
  //   page,
  // }) => {
  //   // 1. Login as admin
  //   await login(
  //     page,
  //     ADMIN_ACCOUNTS.ADMIN1.email,
  //     ADMIN_ACCOUNTS.ADMIN1.password
  //   );

  //   // 2. Select team
  //   await page.locator('[data-pw="team-selector-trigger"]').click();
  //   await page.getByRole("button", { name: TEAM_NAME }).click();

  //   // 3. Navigate to the post
  //   await clickSidebarMenu(page, "drafts");
  //   await page.getByText(POST_TITLE).click();

  //   // 4. Click "시작하기" button
  //   const startButton = page.getByRole("button", { name: /시작하기/i });
  //   await startButton.waitFor({ state: "visible", timeout: TIMEOUT });
  //   await expect(startButton).toBeVisible();
  //   await startButton.click();

  //   // Confirm start if modal appears
  //   const confirmButton = page.getByRole("button", { name: /확인|시작/i });
  //   if (await confirmButton.isVisible({ timeout: 2000 })) {
  //     await confirmButton.click();
  //   }

  //   // Verify space status changed to Active
  //   await expect(page.getByText(/진행 중|Active/i)).toBeVisible();

  //   // 5. Logout and login as new user (user3 - did not participate in pre-survey)
  //   await page.getByRole("button", { name: /로그아웃|Logout/i }).click();
  //   await login(page, USER_ACCOUNTS.USER3.email, USER_ACCOUNTS.USER3.password);

  //   // 6. Try to access the space
  //   await clickSidebarMenu(page, "my-space");

  //   // 7. Try to join
  //   const spaceItem = page.getByText(POST_TITLE);
  //   if (await spaceItem.isVisible({ timeout: 2000 })) {
  //     await spaceItem.click();

  //     // Try to click "참여하기" button
  //     const joinButton = page.getByRole("button", { name: /참여하기/i });

  //     if (await joinButton.isVisible({ timeout: 2000 })) {
  //       await joinButton.click();

  //       // Verify error message or disabled state
  //       await expect(
  //         page.getByText(/참여할 수 없습니다|이미 시작|마감/i)
  //       ).toBeVisible();
  //     } else {
  //       // If join button doesn't exist, verify space is not accessible
  //       await expect(page.getByText(/접근 권한이 없습니다/i)).toBeVisible();
  //     }
  //   } else {
  //     // Space should not appear in the list for non-participants
  //     await expect(page.getByText(POST_TITLE)).not.toBeVisible();
  //   }
  // });
});
