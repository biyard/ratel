import { test } from "@playwright/test";
import { MOBILE_VIEWPORT, TIMEOUT, USER_ACCOUNTS } from "../configs";
import { login, mobileLogin } from "./helpers";
import { SURVEY_QUESTIONS } from "./constants";

test.describe("User - Deliberation E2E Tests", () => {
  test.use({ viewport: MOBILE_VIEWPORT });

  test("[USER-001] User participates and completes pre-survey", async ({
    page,
  }) => {
    await mobileLogin(
      page,
      USER_ACCOUNTS.USER2.email,
      USER_ACCOUNTS.USER2.password
    );

    await page.getByTestId("mobile-profile-icon").click();
    const mobileMenuMySpacesLink = page.getByTestId("my-spaces-link");
    await mobileMenuMySpacesLink.waitFor({
      state: "visible",
      timeout: TIMEOUT,
    });
    await mobileMenuMySpacesLink.click();
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
});
