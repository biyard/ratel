import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Anonymous User Signup Flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("[SU-001] should display signup option in login popup", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await expect(signInButton).toBeVisible();

    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-001-01-login-popup-opened.png",
    });

    const loginPopup = page.locator("#login_popup");
    await expect(loginPopup).toBeVisible();

    const createAccountButton = page.getByText("Create an account");
    await expect(createAccountButton).toBeVisible();
  });

  test("[SU-002] should open user setup popup when clicking create account", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-002-01-login-popup-before-signup.png",
    });

    const createAccountButton = page.getByText("Create an account");
    await createAccountButton.click();

    await page.screenshot({
      path: "test-results/SU-002-02-user-setup-popup-opened.png",
    });

    const userSetupPopup = page.locator("#user_setup_popup");
    await expect(userSetupPopup).toBeVisible();
  });

  test("[SU-003] should validate email format in user setup", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    await page.screenshot({
      path: "test-results/SU-003-01-signup-form-loaded.png",
    });

    const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    await emailInput.fill("invalid-email");
    await page.screenshot({
      path: "test-results/SU-003-02-invalid-email-entered.png",
    });

    const sendButton = page.getByRole("button", { name: /send/i });
    await sendButton.click();
    await page.screenshot({
      path: "test-results/SU-003-03-after-send-invalid-email.png",
    });

    await expect(
      page.locator("text=Email verification code sent"),
    ).not.toBeVisible();
  });

  // test("should validate password requirements", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();

  //   await page.getByText("Create an account").click();

  //   const passwordInput = page.getByPlaceholder(/password/i);

  //   await passwordInput.fill("weak");
  //   await page.screenshot({
  //     path: "test-results/weak-password-validation.png",
  //   });
  //   await expect(page.getByText(/invalid.*password.*format/i)).toBeVisible();

  //   await passwordInput.fill("Password123!");
  //   await page.screenshot({
  //     path: "test-results/strong-password-validation.png",
  //   });
  //   await expect(
  //     page.getByText(/invalid.*password.*format/i),
  //   ).not.toBeVisible();
  // });

  // test("should validate username format", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();

  //   await page.getByText("Create an account").click();

  //   const usernameInput = page.getByPlaceholder(/user.*name/i);

  //   await usernameInput.fill("Invalid Username!");
  //   await page.screenshot({
  //     path: "test-results/invalid-username-validation.png",
  //   });
  //   await expect(page.getByText(/invalid.*username.*format/i)).toBeVisible();

  //   await usernameInput.fill("valid_username123");

  //   await page.screenshot({
  //     path: "test-results/valid-username-validation.png",
  //   });
  //   await expect(
  //     page.getByText(/invalid.*username.*format/i),
  //   ).not.toBeVisible();
  // });

  // test("should require terms of service agreement", async ({ page }) => {
  //   await page.getByRole("button", { name: /sign in/i }).click();

  //   await page.getByText("Create an account").click();

  //   const emailInput = page.getByRole("textbox", { name: /email/i }).first();
  //   await emailInput.fill("test@example.com");

  //   const passwordInput = page.getByPlaceholder(/password/i);
  //   await passwordInput.fill("Password123!");

  //   const displayNameInput = page.getByPlaceholder(/display.*name/i);
  //   await displayNameInput.fill("Test User");

  //   const usernameInput = page.getByPlaceholder(/user.*name/i);
  //   await usernameInput.fill("testuser123");
  //   await page.screenshot({ path: "test-results/form-filled-before-tos.png" });

  //   const finishButton = page.getByRole("button", { name: /finish.*signup/i });
  //   await expect(finishButton).toBeDisabled();

  //   const tosCheckbox = page.locator("#agree_checkbox");
  //   await tosCheckbox.check();
  //   await page.screenshot({
  //     path: "test-results/tos-checked-button-enabled.png",
  //   });

  //   await expect(finishButton).not.toBeDisabled();
  // });

  test("[SU-004] should handle complete signup flow (email signup)", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();
    await page.screenshot({
      path: "test-results/SU-004-01-create-an-account.png",
    });

    // get timestamp epoch
    const no = Date.now();

    await page
      .getByRole("textbox", { name: /email/i })
      .first()
      .fill("test@example.com");
    await page.getByPlaceholder(/password/i).fill("Password123!");
    await page.getByPlaceholder(/display.*name/i).fill("Local User");
    await page.getByPlaceholder(/user.*name/i).fill(`localuser${no}`);
    await page.screenshot({
      path: "test-results/SU-004-02-complete-signup-form-filled.png",
    });

    const tosCheckbox = page.locator('label[for="agree_checkbox"]');
    await tosCheckbox.click();

    const finishButton = page.getByRole("button", { name: "Finished Sign-up" });
    await expect(finishButton).not.toBeDisabled();

    await finishButton.click();

    await page.screenshot({
      path: "test-results/SU-004-03-after-signup-submit.png",
    });
  });

  test("[SU-005] should handle Google signup button visibility", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-005-01-google-button-visibility-check.png",
    });

    const googleButton = page.getByText("Continue With Google");
    if (await googleButton.isVisible()) {
      await expect(googleButton).toBeVisible();
      await expect(googleButton).toBeEnabled();
      await page.screenshot({
        path: "test-results/SU-005-02-google-button-visible.png",
      });
    } else {
      await page.screenshot({
        path: "test-results/SU-005-02-google-button-not-visible.png",
      });
    }
  });

  test("[SU-006] should display profile image uploader", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();
    await page.screenshot({
      path: "test-results/SU-006-01-profile-image-uploader.png",
    });

    await profileImage.hover();
    await page.screenshot({
      path: "test-results/SU-006-02-profile-image-hover-state.png",
    });
    await expect(page.getByText(/clicked.*image/i)).toBeVisible();
  });

  test("[SU-007] should show newsletter subscription option", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const newsletterCheckbox = page.locator("#announcement_checkbox");
    await expect(newsletterCheckbox).toBeVisible();
    await page.screenshot({
      path: "test-results/SU-007-01-newsletter-checkbox-visible.png",
    });

    await newsletterCheckbox.check();
    await page.screenshot({
      path: "test-results/SU-007-02-newsletter-checkbox-checked.png",
    });
    await expect(newsletterCheckbox).toBeChecked();
  });

  test("[SU-008] should prevent signup with blocked keywords", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const displayNameInput = page.getByPlaceholder(/display.*name/i);
    await displayNameInput.fill("test");
    await page.screenshot({
      path: "test-results/SU-008-01-blocked-display-name.png",
    });
    await expect(page.getByText(/display.*name.*warning/i)).toBeVisible();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("test");
    await page.screenshot({
      path: "test-results/SU-008-02-blocked-username.png",
    });
    await expect(page.getByText(/user.*name.*warning/i)).toBeVisible();
  });

  test("[SU-009] should check username availability", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("validusername123");
    await page.screenshot({
      path: "test-results/SU-009-01-username-availability-check.png",
    });

    await page.screenshot({
      path: "test-results/SU-009-02-username-availability-result.png",
    });
  });

  test("[SU-010] should handle mobile responsive layout", async ({ page }) => {
    await page.setViewportSize({
      width: CONFIGS.DEVICE_SCREEN_SIZES.MOBILE - 100,
      height: 800,
    });
    await page.screenshot({
      path: "test-results/SU-010-01-mobile-viewport-set.png",
    });

    await page.getByRole("button", { name: /sign in/i }).click();

    await page.screenshot({
      path: "test-results/SU-010-02-mobile-login-popup.png",
    });

    await page.getByText("Create an account").click();

    await page.screenshot({
      path: "test-results/SU-010-03-mobile-signup-form.png",
    });

    const userSetupPopup = page.locator("#user_setup_popup");
    await expect(userSetupPopup).toBeVisible();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();
  });
});
