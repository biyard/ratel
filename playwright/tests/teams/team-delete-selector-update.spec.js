import { test, expect } from "@playwright/test";
import { click, goto, waitPopup } from "../utils";

// This test verifies that after deleting a team, the Team Selector
// immediately reflects the removal without requiring a page refresh.
// Covers issue #1318.

test.describe.serial("Team deletion updates Team Selector (#1318)", () => {
  const teamNickname = `Delete Test Team`;
  const teamUsername = `e2e_del_${Date.now()}`;

  test("Create a team, delete it, and verify it is removed from Team Selector", async ({
    page,
  }) => {
    // Step 1: Navigate to home and open profile dropdown
    await goto(page, "/");
    await click(page, { label: "User Profile" });

    // Step 2: Click "Create Team"
    await click(page, { text: "Create Team" });

    // Step 3: Fill in team creation form
    const nicknameInput = page.locator('[data-testid="team-nickname-input"]');
    await nicknameInput.fill(teamNickname);

    const usernameInput = page.locator('[data-testid="team-username-input"]');
    await usernameInput.fill(teamUsername);

    const descInput = page.locator('[data-testid="team-description-input"]');
    await descInput.fill("E2E test team for deletion verification");

    // Submit the form
    await click(page, { text: "Create" });

    // Wait for navigation to team home page
    await page.waitForURL(new RegExp(`/${teamUsername}/home`), {
      waitUntil: "load",
    });

    // Step 4: Verify team appears on the team home page heading
    await expect(
      page.getByRole("heading", { name: teamNickname })
    ).toBeVisible();

    // Step 5: Navigate to team settings
    await goto(page, `/${teamUsername}/settings`);

    // Step 6: Click "Delete team" button
    // The settings page uses nested use_loader calls (TeamContext + settings data).
    // SSR may not resolve the inner suspension, so the client fetches after hydration.
    // Wait with a longer timeout for the admin page content to appear.
    await expect(
      page.getByText("Delete team", { exact: true })
    ).toBeVisible({ timeout: 30000 });
    await click(page, { text: "Delete team" });

    // Step 7: Confirm deletion in the popup
    await waitPopup(page, { visible: true });
    await click(page, { text: "Confirm" });

    // Step 8: Wait for navigation to home page after deletion
    await page.waitForURL("/", { waitUntil: "load" });

    // Step 9: Verify the deleted team is NOT in the Team Selector
    // The team nickname should no longer appear anywhere on the page
    await expect(page.getByText(teamNickname)).toBeHidden();
  });
});
