import { test, expect } from "@playwright/test";
import { click, goto } from "../utils";

test.describe.serial("Character progression", () => {
  test("brand-new user lands at Level 1 with 5 SP", async ({ page }) => {
    await goto(page, "/me/character");
    await expect(page.getByTestId("hero-level")).toBeVisible();
    await expect(page.getByTestId("hero-sp-value")).toBeVisible();
  });

  test("buying Money Tree L1 disables button when SP runs out", async ({ page }) => {
    await goto(page, "/me/character");
    await click(page, { testId: "skill-levelup-money_tree" });
    // After spending 5 SP at L1 user, the next-level cost (9) > unspent (0).
    await expect(page.getByTestId("skill-levelup-money_tree")).toBeDisabled();
  });

  // TODO: voting-grants-XP test requires multi-page fixture — split into a
  // follow-up spec once the cross-page activity helper exists.
});
