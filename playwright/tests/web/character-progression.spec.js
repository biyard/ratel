import { test, expect } from "@playwright/test";
import { click, fill, goto, waitPopup } from "../utils";

/**
 * Character progression
 *
 * `/me/character` requires authentication (handler uses `user: User`
 * extractor). The spec signs up a fresh user via the bypass code "000000",
 * then drives the character page interactions.
 *
 * NOTE: requires backend built with `--features bypass` so that email
 *       verification accepts "000000".
 */

test.describe.serial("Character progression", () => {
  let context;
  let page;

  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const testUser = {
    email: `e2e_character_${uniqueId}@biyard.co`,
    username: `ch${uniqueId}`,
    displayName: `Character ${uniqueId}`,
    password: "Test!234",
  };

  test.beforeAll(async ({ browser }) => {
    context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    page = await context.newPage();
  });

  test.afterAll(async () => {
    await context.close();
  });

  test("sign up a brand-new user", async () => {
    await goto(page, "/");
    await click(page, { label: "Sign In" });
    await waitPopup(page, { visible: true });

    await click(page, { text: "Create an account" });
    await fill(page, { placeholder: "Enter your email address" }, testUser.email);
    await click(page, { text: "Send" });
    await fill(page, { placeholder: "Enter the verification code" }, "000000");
    await click(page, { text: "Verify" });
    await expect(page.getByText("Send", { exact: true })).toBeHidden({
      timeout: 10000,
    });

    await fill(page, { placeholder: "Enter your password" }, testUser.password);
    await fill(page, { placeholder: "Re-enter your password" }, testUser.password);
    await fill(page, { placeholder: "Enter your display name" }, testUser.displayName);
    await fill(page, { placeholder: "Enter your user name" }, testUser.username);
    await click(page, {
      label: "[Required] I have read and accept the Terms of Service.",
    });
    await click(page, { text: "Finished Sign-up" });
    await waitPopup(page, { visible: false });
  });

  test("brand-new user lands at Level 1 with 5 SP", async () => {
    await goto(page, "/me/character");
    await expect(page.getByTestId("hero-level")).toBeVisible();
    await expect(page.getByTestId("hero-sp-value")).toBeVisible();
    await expect(page.getByTestId("hero-level")).toContainText("1");
    await expect(page.getByTestId("hero-sp-value")).toContainText("5");
  });

  test("buying Money Tree L1 disables button when SP runs out", async () => {
    await goto(page, "/me/character");
    await click(page, { testId: "skill-levelup-money_tree" });
    // After spending 5 SP at L1 user, the next-level cost (9) > unspent (0),
    // so the level-up button should be disabled. Allow extra time for the
    // POST + Loader.refresh round trip before assertion.
    await expect(page.getByTestId("skill-levelup-money_tree")).toBeDisabled({
      timeout: 10000,
    });
  });

  // TODO: voting-grants-XP test requires multi-page fixture — split into a
  // follow-up spec once the cross-page activity helper exists.
});
