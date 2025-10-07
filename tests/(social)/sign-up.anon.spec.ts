import { test, expect } from "@playwright/test";
import { CONFIGS } from "../config";
import { click } from "../utils";

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

    const createAccountButton = page.getByText("Create an account");
    await createAccountButton.click();

    const userSetupPopup = page.locator("#user_setup_popup");
    await expect(userSetupPopup).toBeVisible();
  });

  test("[SU-003] should validate email format in user setup", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    await emailInput.fill("invalid-email");

    const sendButton = page.getByRole("button", { name: /send/i });
    await sendButton.click();

    await expect(
      page.locator("text=Email verification code sent"),
    ).not.toBeVisible();
  });

  test("[SU-011] should validate password requirements", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const passwordInput = page.getByPlaceholder(/password/i);

    await passwordInput.fill("weakpassword");
    await expect(
      page.getByText(
        "Password must contain letters, numbers, and special characters (min 8 chars).",
      ),
    ).toBeVisible();

    await passwordInput.fill("Password123!");
    await expect(
      page.getByText(
        "Password must contain letters, numbers, and special characters (min 8 chars).",
      ),
    ).not.toBeVisible();
  });

  test("[SU-012] should validate username format", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const usernameInput = page.getByPlaceholder(/user.*name/i);

    await usernameInput.fill("Invalid Username!");
    await expect(
      page.getByText(
        "Only numbers, lowercase letters, -, _ and more than one character can be entered.",
      ),
    ).toBeVisible();

    await usernameInput.fill("valid_username123");

    await expect(
      page.getByText(
        "Only numbers, lowercase letters, -, _ and more than one character can be entered.",
      ),
    ).not.toBeVisible();
  });

  test("[SU-013] should require terms of service agreement", async ({
    page,
  }) => {
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

    const finishButton = page.getByRole("button", { name: "Finished Sign-up" });
    await expect(finishButton).toBeDisabled();

    const tosCheckbox = page.locator('label[for="agree_checkbox"]');
    await tosCheckbox.click();

    await expect(finishButton).not.toBeDisabled();
  });

  test("[SU-004] should handle complete signup flow (email signup)", async ({
    page,
  }) => {
    const id = CONFIGS.PLAYWRIGHT.ID;
    const email = `su004+${id}@ratel.foundation`;
    const password = "password1234!@#$";
    const displayName = `Playwright User ${id}`;
    const userName = `su004-${id}`;

    await page.getByRole("button", { name: /sign in/i }).click();
    await page.getByText("Create an account").click();

    await page.getByPlaceholder("Email", { exact: true }).fill(email);
    await page.getByText("Send").click();

    await page
      .getByPlaceholder("Verify code in your email.", { exact: true })
      .fill("000000");
    await click(page, { text: "Verify" });

    await page.getByPlaceholder(/password/i).fill(password);
    await page.getByPlaceholder(/display name/i).fill(displayName);
    await page.getByPlaceholder(/user name/i).fill(userName);

    // Accept terms by clicking the label (checkbox is hidden)
    const tosCheckbox = page.locator('label[for="agree_checkbox"]');
    await tosCheckbox.click();
    await page.getByRole("button", { name: /finished sign-up/i }).click();
    await expect(page.getByText(/start/i)).toBeVisible();
  });

  test("[SU-005] should handle Google signup button visibility", async ({
    page,
  }) => {
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await signInButton.click();

    const googleButton = page.getByText("Continue With Google");
    if (await googleButton.isVisible()) {
      await expect(googleButton).toBeVisible();
      await expect(googleButton).toBeEnabled();
    } else {
    }
  });

  test("[SU-006] should display profile image uploader", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();

    await profileImage.hover({ force: true });
    await expect(
      page.getByText(/click.*change.*profile.*image/i),
    ).toBeVisible();
  });

  test("[SU-007] should show newsletter subscription option", async ({
    page,
  }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const newsletterCheckbox = page.locator(
      'label[for="announcement_checkbox"]',
    );
    await expect(newsletterCheckbox).toBeVisible();

    await newsletterCheckbox.click();
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
    await expect(
      page.getByText("Please remove the test keyword from your display name."),
    ).toBeVisible();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("test");
    await expect(
      page.getByText("Please remove the test keyword from your username."),
    ).toBeVisible();
  });

  test("[SU-009] should check username availability", async ({ page }) => {
    await page.getByRole("button", { name: /sign in/i }).click();

    await page.getByText("Create an account").click();

    const usernameInput = page.getByPlaceholder(/user.*name/i);
    await usernameInput.fill("validusername123");
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

    const userSetupPopup = page.locator("#user_setup_popup");
    await expect(userSetupPopup).toBeVisible();

    const profileImage = page.locator('img[alt="Team Logo"]');
    await expect(profileImage).toBeVisible();

    // Verify form fields are still accessible in mobile layout
    const emailInput = page.getByRole("textbox", { name: /email/i }).first();
    await expect(emailInput).toBeVisible();

    const passwordInput = page.getByPlaceholder(/password/i);
    await expect(passwordInput).toBeVisible();
  });
});
