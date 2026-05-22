import { test, expect } from "@playwright/test";
import { click, fill, goto } from "../utils";

/**
 * Fact-or-Fold admin — settings + auxiliary tab navigation.
 *
 *   1. Enter the admin area through the arcade home CTA.
 *   2. Navigate via the admin tab bar to /settings, /stats, /reports —
 *      every tab must reach its URL and mark itself selected.
 *   3. Edit the "queue low alert days" number, save it, reload the page,
 *      and verify the value persisted server-side.
 *
 * NOTE: requires backend built with `--features bypass`. Storage state
 * `admin.json` is provided by `admin.auth.setup.js`.
 */
test.describe.serial("Fact-or-Fold admin settings + stats", () => {
  test("Step 1: navigate from arcade home CTA into the admin tab bar", async ({
    page,
  }) => {
    await goto(page, "/arcade/home");

    await click(page, { testId: "ff-arcade-admin-cta" });
    await page.waitForURL(/\/admin\/fact-or-fold\/subjects\/new$/, {
      waitUntil: "load",
    });

    // All five admin tabs must be present (rendered by FactFoldAdminLayout).
    for (const id of [
      "ff-admin-tab-subjects",
      "ff-admin-tab-schedule",
      "ff-admin-tab-stats",
      "ff-admin-tab-reports",
      "ff-admin-tab-settings",
    ]) {
      await expect(page.getByTestId(id)).toBeVisible();
    }
  });

  test("Step 2: tab bar moves between Stats and Reports", async ({ page }) => {
    await goto(page, "/admin/fact-or-fold/subjects/new");

    await click(page, { testId: "ff-admin-tab-stats" });
    await page.waitForURL(/\/admin\/fact-or-fold\/stats$/, {
      waitUntil: "load",
    });
    await expect(page.getByTestId("ff-admin-tab-stats")).toHaveAttribute(
      "aria-selected",
      "true",
    );

    await click(page, { testId: "ff-admin-tab-reports" });
    await page.waitForURL(/\/admin\/fact-or-fold\/reports$/, {
      waitUntil: "load",
    });
    await expect(page.getByTestId("ff-admin-tab-reports")).toHaveAttribute(
      "aria-selected",
      "true",
    );
  });

  test("Step 3: edit queue-low alert, save, reload, verify persistence", async ({
    page,
  }) => {
    await goto(page, "/admin/fact-or-fold/subjects/new");

    await click(page, { testId: "ff-admin-tab-settings" });
    await page.waitForURL(/\/admin\/fact-or-fold\/settings$/, {
      waitUntil: "load",
    });

    const queueInput = page.getByTestId("ff-admin-settings-queue-low");
    await expect(queueInput).toBeVisible({ timeout: 15000 });
    const original = (await queueInput.inputValue()) || "5";
    // Pick a value distinct from whatever the singleton currently holds.
    const next = original === "7" ? "9" : "7";

    await fill(page, { testId: "ff-admin-settings-queue-low" }, next);
    await click(page, { testId: "ff-admin-settings-save" });

    // Saved indicator surfaces on success — wait for it explicitly instead
    // of `waitForLoadState` so we're tied to the actual save response.
    await expect(page.locator(".ff-settings__saved")).toBeVisible({
      timeout: 15000,
    });

    // Round-trip reload to confirm persistence.
    await goto(page, "/admin/fact-or-fold/settings");
    await expect(page.getByTestId("ff-admin-settings-queue-low")).toHaveValue(
      next,
    );

    // Restore so reruns don't drift the singleton.
    await fill(page, { testId: "ff-admin-settings-queue-low" }, original);
    await click(page, { testId: "ff-admin-settings-save" });
    await expect(page.locator(".ff-settings__saved")).toBeVisible({
      timeout: 15000,
    });
  });
});
