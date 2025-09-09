import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";

test.describe("Create Ordinary Post", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("[COP-001] should display login option in login popup", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await expect(signInButton).toBeVisible();

    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-001/01-login-popup-opened.png",
    });

    const loginPopup = page.locator("#login_popup");
    await expect(loginPopup).toBeVisible();

    const createAccountButton = page.getByText("Create an account");
    await expect(createAccountButton).toBeVisible();
  });

  test("[SU-002] should open user login popup when clicking create account", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-002/01-login-popup-before-signup.png",
    });

    const createAccountButton = page.getByText("Create an account");
    await createAccountButton.click();

    await page.screenshot({
      path: "test-results/SU-002/02-user-setup-popup-opened.png",
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
      path: "test-results/SU-003/01-signup-form-loaded.png",
    });

    // const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    const emailInput = page.getByRole('textbox', { name: 'Enter your email address' })
    await emailInput.fill("invalid-email");
    await page.screenshot({
      path: "test-results/SU-003/02-invalid-email-entered.png",
    });

    const sendButton = page.getByRole("button", { name: /send/i });
    await sendButton.click();
    await page.screenshot({
      path: "test-results/SU-003/03-after-send-invalid-email.png",
    });

    await expect(
      page.locator("text=Email verification code sent"),
    ).not.toBeVisible();
  });

  test("[SU-011] should validate password requirements", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const passwordInput = page.getByPlaceholder(/password/i);

    await passwordInput.fill("weakpassword");
    await page.screenshot({
      path: "test-results/SU-011/01-weak-password-validation.png",
    });
    await expect(page.getByText("Password must contain letters, numbers, and special characters (min 8 chars).")).toBeVisible();

    await passwordInput.fill("Password123!");
    await page.screenshot({
      path: "test-results/SU-011/02-strong-password-validation.png",
    });
    await expect(
      page.getByText("Password must contain letters, numbers, and special characters (min 8 chars)."),
    ).not.toBeVisible();
  });

  test("[SU-012] should validate username format", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const usernameInput = page.getByPlaceholder(/user.*name/i);

    await usernameInput.fill("Invalid Username!");
    await page.screenshot({
      path: "test-results/SU-012/01-invalid-username-validation.png",
    });
    await expect(page.getByText("Only numbers, lowercase letters, -, _ and more than one character can be entered.")).toBeVisible();

    await usernameInput.fill("valid_username123");

    await page.screenshot({
      path: "test-results/SU-012/02-valid-username-validation.png",
    });
    await expect(
      page.getByText("Only numbers, lowercase letters, -, _ and more than one character can be entered."),
    ).not.toBeVisible();
  });

  test("[SU-013] should require terms of service agreement", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    await emailInput.fill("test@example.com");

    const passwordInput = page.getByPlaceholder(/password/i);
    await passwordInput.fill("Password123!");

    const displayNameInput = page.getByPlaceholder(/display.*name/i);
    await displayNameInput.fill("Valid User");

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("validuser123");
    
    // Wait for username validation to complete
    await page.waitForTimeout(2000);
    
    await page.screenshot({ path: "test-results/SU-013/01-form-filled-before-tos.png" });

    const finishButton = page.getByRole("button", { name: "Finished Sign-up" });
    await expect(finishButton).toBeDisabled();

    const tosCheckbox = page.locator('label[for="agree_checkbox"]');
    await tosCheckbox.click();
    await page.screenshot({
      path: "test-results/SU-013/02-tos-checked-button-enabled.png",
    });

    await expect(finishButton).not.toBeDisabled();
  });

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
      path: "test-results/SU-004/03-after-signup-submit.png",
    });
  });

  test("[SU-005] should handle Google signup button visibility", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await signInButton.click();

    await page.screenshot({
      path: "test-results/SU-005/01-google-button-visibility-check.png",
    });

    const googleButton = page.getByText("Continue With Google");
    if (await googleButton.isVisible()) {
      await expect(googleButton).toBeVisible();
      await expect(googleButton).toBeEnabled();
      await page.screenshot({
        path: "test-results/SU-005/02-google-button-visible.png",
      });
    } else {
      await page.screenshot({
        path: "test-results/SU-005/02-google-button-not-visible.png",
      });
    }
  });

  test("[SU-006] should display profile image uploader", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();
    await page.screenshot({
      path: "test-results/SU-006/01-profile-image-uploader.png",
    });

    await profileImage.hover({ force: true });
    await page.screenshot({
      path: "test-results/SU-006/02-profile-image-hover-state.png",
    });
    await expect(page.getByText(/click.*change.*profile.*image/i)).toBeVisible();
  });

  test("[SU-007] should show newsletter subscription option", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const newsletterCheckbox = page.locator('label[for="announcement_checkbox"]');
    await expect(newsletterCheckbox).toBeVisible();
    await page.screenshot({
      path: "test-results/SU-007/01-newsletter-checkbox-visible.png",
    });

    await newsletterCheckbox.click();
    await page.screenshot({
      path: "test-results/SU-007/02-newsletter-checkbox-checked.png",
    });
    const actualCheckbox = page.locator("#announcement_checkbox");
    await expect(actualCheckbox).toBeChecked();
  });

  test("[SU-008] should prevent signup with blocked keywords", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const displayNameInput = page.getByPlaceholder(/display.*name/i);
    await displayNameInput.fill("test");
    await page.screenshot({
      path: "test-results/SU-008/01-blocked-display-name.png",
    });
    await expect(page.getByText("Please remove the test keyword from your display name.")).toBeVisible();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("test");
    await page.screenshot({
      path: "test-results/SU-008/02-blocked-username.png",
    });
    await expect(page.getByText("Please remove the test keyword from your username.")).toBeVisible();
  });

  test("[SU-009] should check username availability", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("validusername123");
    await page.screenshot({
      path: "test-results/SU-009/01-username-availability-check.png",
    });

    await page.screenshot({
      path: "test-results/SU-009/02-username-availability-result.png",
    });
  });

  test("[SU-010] should handle mobile responsive layout", async ({ page }) => {
    // First access the page normally to get to the signup form
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByText("Create an account").click();
    
    // Then test mobile layout on the signup form
    await page.setViewportSize({
      width: CONFIGS.DEVICE_SCREEN_SIZES.MOBILE - 100,
      height: 800,
    });
    await page.screenshot({
      path: "test-results/SU-010/01-mobile-viewport-set.png",
    });

    const userSetupPopup = page.locator("#user_setup_popup");
    await expect(userSetupPopup).toBeVisible();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();
    
    await page.screenshot({
      path: "test-results/SU-010/02-mobile-signup-form.png",
    });

    // Verify form fields are still accessible in mobile layout
    const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    await expect(emailInput).toBeVisible();
    
    const passwordInput = page.getByPlaceholder(/password/i);
    await expect(passwordInput).toBeVisible();
  });
});
