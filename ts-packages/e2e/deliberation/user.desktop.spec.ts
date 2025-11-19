import { test, expect, Page } from "@playwright/test";
import { DESKTOP_VIEWPORT, TIMEOUT, USER_ACCOUNTS } from "../configs";
import { login } from "./helpers";
import { SURVEY_QUESTIONS } from "./constants";

test.describe("User - Deliberation E2E Tests", () => {
  test.use({ viewport: DESKTOP_VIEWPORT });

  test("[USER-001] User participates and completes pre-survey", async ({
    page,
  }) => {
    await login(page, USER_ACCOUNTS.USER1.email, USER_ACCOUNTS.USER1.password);

    const mySpaceButton = page.getByTestId("sidemenu-my-spaces");
    await mySpaceButton.waitFor({ state: "visible", timeout: TIMEOUT });
    await mySpaceButton.click();
    await page.waitForLoadState("networkidle");

    await page.getByTestId("space-card").last().click();
    await page.waitForLoadState("networkidle");

    for (const question of SURVEY_QUESTIONS) {
      if (question.type === "single_choice" && question.options) {
        const option = page.getByText(question.options[0], { exact: true });
        await option.waitFor({ state: "visible", timeout: TIMEOUT });
        await option.click();
        await page.waitForTimeout(500);
      } else if (question.type === "multiple_choice" && question.options) {
        for (const optionText of question.options.splice(0, 2)) {
          const option = page.getByText(optionText, { exact: true });
          await option.waitFor({ state: "visible", timeout: TIMEOUT });
          await option.click();
          await page.waitForTimeout(300);
        }
      } else if (question.type === "short_answer") {
        const input = page.getByRole("textbox");
        await input.waitFor({ state: "visible", timeout: TIMEOUT });
        await input.fill("This is a short answer response.");
      } else if (question.type === "subjective") {
        const textarea = page.getByRole("textbox");
        await textarea.waitFor({ state: "visible", timeout: TIMEOUT });
        await textarea.fill("This is a subjective response.");
      }
      if (
        question !== SURVEY_QUESTIONS[SURVEY_QUESTIONS.length - 1] &&
        question.type !== "single_choice"
      ) {
        await page.getByTestId("survey-btn-next").click();
        await page.waitForTimeout(500);
      }
    }

    await page.getByTestId("survey-btn-submit").click();
    await page.waitForLoadState("networkidle");

    await page.getByTestId("complete-survey-modal-btn-confirm").click();
  });
  test("[USER-002] User cannot participate in survey", async ({ page }) => {
    await login(page, USER_ACCOUNTS.USER8.email, USER_ACCOUNTS.USER8.password);

    const mySpaceButton = page.getByTestId("sidemenu-my-spaces");
    await mySpaceButton.waitFor({ state: "visible", timeout: TIMEOUT });
    await mySpaceButton.click();
    await page.waitForLoadState("networkidle");

    await page.getByTestId("space-card").last().click();
    await page.waitForLoadState("networkidle");

    await page
      .getByTestId("error-zone")
      .waitFor({ state: "visible", timeout: TIMEOUT });
  });
});
