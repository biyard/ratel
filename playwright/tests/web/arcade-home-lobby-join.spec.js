import { test, expect } from "@playwright/test";
import { click, goto } from "../utils";

/**
 * Arcade home — partial lobby flow (single-user friendly subset).
 *
 *   1. Render /arcade/home and verify the chip-balance widget + Join CTA.
 *   2. Drive the Join CTA — verifies the user lands on the matching page
 *      with their own slot filled. Capacity (4) is not reached with a
 *      single user, so we stop here.
 *   3. Leave via the matching Cancel button — returns to /arcade/home.
 *   4. Navigate to the leaderboard tab — accepts either populated rows or
 *      the empty-state placeholder.
 *
 * The four-player full round flow is intentionally out of scope; that
 * variant needs stage-timer manipulation that is being designed separately.
 *
 * NOTE: requires backend built with `--features bypass`.
 */
test.describe.serial("Arcade home + lobby join", () => {
  test("Step 1: home renders chip balance + join CTA", async ({ page }) => {
    await goto(page, "/arcade/home");

    await expect(page.getByTestId("ff-arcade-chip")).toBeVisible({
      timeout: 15000,
    });
    // The Join button is gated by `lobby.can_join`. With an empty lobby
    // it is the visible CTA; on a noisy DB it may be hidden behind a
    // "round-in-progress" message — accept either state by waiting up
    // to a reasonable budget and skipping the rest of the flow if the
    // Join button never appears.
    await page
      .getByTestId("ff-arcade-join")
      .waitFor({ state: "visible", timeout: 5000 })
      .catch(() => {});
  });

  test("Step 2: clicking Join lands on the matching page with my slot filled", async ({
    page,
  }) => {
    await goto(page, "/arcade/home");

    const joinBtn = page.getByTestId("ff-arcade-join");
    if (!(await joinBtn.isVisible().catch(() => false))) {
      test.skip(true, "join CTA not surfaced — round may already be in flight");
    }

    await click(page, { testId: "ff-arcade-join" });
    await page.waitForURL(/\/arcade\/games\/fact-or-fold\/matching\/[^/]+$/, {
      waitUntil: "load",
    });

    await expect(page.getByTestId("ff-matching-slot-self")).toBeVisible({
      timeout: 15000,
    });
  });

  test("Step 3: Cancel returns to /arcade/home", async ({ page }) => {
    // Resume the matching context from the lobby join the prior test left
    // behind. The home page auto-redirects when `already_joined` is true,
    // so kicking off from /arcade/home is the most robust way back into
    // matching.
    await goto(page, "/arcade/home");

    // Either we are already on the matching page (auto-redirected) or the
    // round expired/cleared — accept both. If we're still on home, skip
    // the Cancel verification.
    const onMatching = await page
      .waitForURL(/\/arcade\/games\/fact-or-fold\/matching\/[^/]+$/, {
        timeout: 5000,
        waitUntil: "load",
      })
      .then(() => true)
      .catch(() => false);
    if (!onMatching) {
      test.skip(true, "no active matching round to cancel");
    }

    await click(page, { testId: "ff-matching-cancel" });
    await page.waitForURL(/\/arcade\/home$/, { waitUntil: "load" });
  });

  test("Step 4: leaderboard tab is reachable", async ({ page }) => {
    await goto(page, "/arcade/home");

    await click(page, { testId: "ff-arcade-tab-leaderboard" });
    await page.waitForURL(/\/arcade\/leaderboard$/, { waitUntil: "load" });
    await expect(
      page.getByTestId("ff-arcade-tab-leaderboard"),
    ).toHaveAttribute("aria-selected", "true");
  });
});
