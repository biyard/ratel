// E2E coverage for the report publish + viewer download flow.
//
// Single serial suite because each step depends on the previous: the
// admin creates a team + space, installs the Report app, drafts a
// report, publishes it; then a brand-new viewer account signs up,
// opens the same space, finds the report in the settings sidebar,
// and clicks `View Report` → the detail page in viewer mode fires
// the print dialog when the PDF Download button is clicked.
//
// Notes on what's NOT tested here:
//   * The actual PDF rendering / file download — `window.print()` is
//     stubbed; we only assert it was invoked. The browser's native
//     "Save as PDF" picker is outside Playwright's reach.
//   * Slash-command chart insertion — the requirement explicitly says
//     "plain text only, no `/`", so the editor body is just typed.

import { expect, test } from "@playwright/test";
import {
  click,
  clickNoNav,
  createTeamFromHome,
  createTeamPostFromHome,
  dismissDevToast,
  fill,
  getEditor,
  goto,
  waitPopup,
} from "../utils";

/**
 * Sign up a fresh account from inside a space's "Sign in" modal. Mirrors
 * the `signUpFromSpace` helper in `team-space-with-signup-users.spec.js`
 * — kept inline here so this spec stays self-contained.
 *
 * Returns { context, page } — caller MUST close `context` in a
 * `try/finally` to avoid leaking browser contexts across tests.
 */
async function signUpFromSpace(browser, spaceUrl) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, spaceUrl);
  await page.addStyleTag({
    content:
      "*, *::before, *::after { animation-play-state: paused !important; }",
  });
  await clickNoNav(page, { testId: "btn-signin" });
  await waitPopup(page, { visible: true });

  // Passwordless email-code flow: email → Continue (sends code) →
  // code "000000" → Continue (verifies). A new email surfaces UserNotFound,
  // opening the signup modal with email + code already verified.
  const signupEmail = `e2e_report_${Date.now()}@biyard.co`;
  await fill(page, { placeholder: "Enter your email address" }, signupEmail);
  await click(page, { testId: "continue-button" });
  await fill(page, { testId: "code-input" }, "000000");
  await click(page, { testId: "continue-button" });

  const uniqueId = Date.now().toString();
  await fill(
    page,
    { placeholder: "Enter your display name" },
    `Report Viewer ${uniqueId}`,
  );
  await fill(page, { placeholder: "Enter your user name" }, `rv${uniqueId}`);
  await click(page, {
    label: "[Required] I have read and accept the Terms of Service.",
  });
  await click(page, { text: "Finished Sign-up" });
  await waitPopup(page, { visible: false });

  // Fresh signups land on /onboarding/connections — skip back to the
  // space so the rest of the test can hit the space surface.
  await expect(page).toHaveURL(/\/onboarding\/connections/, { timeout: 10000 });
  await click(page, { testId: "onboarding-skip" });
  await goto(page, spaceUrl);

  return { context, page };
}

test.describe.serial("Report publish + viewer download", () => {
  // Shared state between sequential tests in this describe.
  let spaceUrl;

  const teamUsername = `e2e_rp_${Date.now()}`;
  const teamNickname = "Report Publish Team";
  const postTitle = "Report Publish E2E Space";
  const postContents =
    "Backing post for the report publish + viewer download e2e suite.";
  const reportBody =
    "This is the plain text body of the published report. No slash commands, no charts — just narrative text we expect the viewer to see after publish.";

  test("Setup: admin creates team, post, and space", async ({ page }) => {
    await createTeamFromHome(page, {
      username: teamUsername,
      nickname: teamNickname,
      description: "E2E team for report publish flow",
    });

    const postId = await createTeamPostFromHome(
      page,
      teamUsername,
      postTitle,
      postContents,
    );

    const spaceRes = await page.request.post("/api/spaces/create", {
      data: { req: { post_id: postId } },
    });
    expect(
      spaceRes.ok(),
      `create space: ${await spaceRes.text()}`,
    ).toBeTruthy();
    const spaceId = (await spaceRes.json()).space_id;
    spaceUrl = `/spaces/${spaceId}`;

    // Sanity-check the dashboard is reachable so subsequent tests can
    // assume the admin context lands on a live space.
    await goto(page, `${spaceUrl}/dashboard/`);
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/dashboard\/?$/, {
      waitUntil: "load",
      timeout: 10000,
    });

    // Publish the space as public so the viewer account created later
    // can actually open it — a Designing-state space rejects everyone
    // except the creator with a "Something went wrong" guard. Mirrors
    // the topbar publish flow from `space-publish-invitation.spec.js`.
    await goto(page, `${spaceUrl}/`);
    await clickNoNav(page, { testId: "btn-publish" });
    await waitPopup(page, { visible: true });
    await clickNoNav(page, { testId: "public-option" });
    await click(page, { label: "Confirm visibility selection" });
    await waitPopup(page, { visible: false });
  });

  test("Step 1: admin installs the Report app", async ({ page }) => {
    await goto(page, spaceUrl);
    await dismissDevToast(page);

    // Open the settings drawer
    await click(page, { testId: "btn-settings" });
    const settings = page.getByTestId("settings-panel");
    await expect(settings).toHaveAttribute("data-open", "true");

    const apps = page.getByTestId("apps-section");
    await expect(apps).toBeVisible();

    // Report appears in Available Apps with `+ INSTALL`
    const installBtn = page.getByTestId("install-app-report");
    await expect(installBtn).toBeVisible({ timeout: 15000 });
    await installBtn.click();

    // After install the row moves to Installed Apps — the `settings`
    // button appears and the install button disappears.
    await expect(page.getByTestId("settings-app-report")).toBeVisible({
      timeout: 15000,
    });
    await expect(page.getByTestId("install-app-report")).toBeHidden();
  });

  test("Step 2: admin opens reports list via settings sidebar", async ({
    page,
  }) => {
    await goto(page, spaceUrl);
    await dismissDevToast(page);

    await click(page, { testId: "btn-settings" });
    await expect(page.getByTestId("settings-panel")).toHaveAttribute(
      "data-open",
      "true",
    );

    // Click SETTINGS on the Report row → navigates to the reports
    // list page. `click()` already waits for `load`, but we also
    // verify the URL so a route mis-wiring is caught loudly.
    await click(page, { testId: "settings-app-report" });
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/report\/?$/, {
      waitUntil: "load",
      timeout: 15000,
    });
  });

  test("Step 3: admin creates a new draft report", async ({ page }) => {
    await goto(page, `${spaceUrl}/report/`);
    await dismissDevToast(page);

    // The "NEW REPORT" topbar button is identified by its aria-label
    // (i18n: "Create a new report" / "새 보고서 생성").
    const newReportBtn = page
      .locator('button[aria-label*="new report" i], button[aria-label*="보고서"]')
      .first();
    await expect(newReportBtn).toBeVisible({ timeout: 10000 });
    await newReportBtn.click();

    // Lands on the detail page (`/spaces/:id/reports/:report_id`).
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/report\/[^/]+\/?$/, {
      waitUntil: "load",
      timeout: 15000,
    });

    // Editor body should be ready before we start typing into it.
    const editor = await getEditor(page);
    await expect(editor).toBeVisible();
  });

  test("Step 4: admin types a plain-text body", async ({ page }) => {
    // Navigate back into the most recently created draft — the
    // reports list carousel shows it first after `ReportCreateCard`.
    await goto(page, `${spaceUrl}/report/`);
    await dismissDevToast(page);

    // First non-create card → most recent draft we just made.
    const firstCard = page.locator(".report-card").nth(1);
    await expect(firstCard).toBeVisible({ timeout: 10000 });
    await firstCard.click();
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/report\/[^/]+\/?$/, {
      waitUntil: "load",
      timeout: 15000,
    });

    const editor = await getEditor(page);
    await editor.click();
    // Use `editor.fill()` for the textual body — RichEditor's
    // contenteditable accepts plain text via fill(). The autosave
    // effect persists the body 3s after the last keystroke.
    await editor.fill(reportBody);

    // Wait long enough for the debounced autosave to fire so the
    // server-side `html_contents` reflects what we just typed before
    // the next test publishes.
    await page.waitForTimeout(4000);
  });

  test("Step 5: admin publishes the report", async ({ page }) => {
    await goto(page, `${spaceUrl}/report/`);
    await dismissDevToast(page);
    const firstCard = page.locator(".report-card").nth(1);
    await firstCard.click();
    await page.waitForURL(/\/spaces\/[a-z0-9-]+\/report\/[^/]+\/?$/, {
      waitUntil: "load",
      timeout: 15000,
    });

    // Trigger the publish modal.
    await click(page, { testId: "publish-report" });

    // Modal confirm — submits the PATCH that flips status=Published.
    await click(page, { testId: "publish-confirm" });

    // After the publish action settles, the publish button must be
    // gone (admins + Published hides it; from this point onward
    // edits stream through the autosave loop instead).
    await expect(page.getByTestId("publish-report")).toBeHidden({
      timeout: 15000,
    });

    // PDF Download stays visible — admins can still download on the
    // detail page.
    await expect(page.getByTestId("pdf-download")).toBeVisible();
  });

  test("Step 6: viewer signs up + sees the report in sidebar", async ({
    browser,
  }) => {
    const { context, page } = await signUpFromSpace(browser, spaceUrl);
    try {
      await dismissDevToast(page);

      // Open the settings panel. Admin chrome won't render for the
      // viewer (no Apps section because they aren't a Creator), but
      // the Reports section should — the published report is the
      // single row inside.
      await click(page, { testId: "btn-settings" });
      const settings = page.getByTestId("settings-panel");
      await expect(settings).toHaveAttribute("data-open", "true");

      const reports = page.getByTestId("reports-section");
      await expect(reports).toBeVisible({ timeout: 15000 });
      // Title from the draft is "Untitled report …" since the admin
      // never renamed it — assert by selecting the row's "View Report"
      // button (i18n label `보고서 확인하기` / "View Report") rather
      // than by exact title.
      const viewBtn = reports
        .locator('[data-testid^="download-report-"]')
        .first();
      await expect(viewBtn).toBeVisible({ timeout: 5000 });
    } finally {
      await context.close();
    }
  });

  test("Step 7: viewer opens the report + PDF download triggers print", async ({
    browser,
  }) => {
    const { context, page } = await signUpFromSpace(browser, spaceUrl);
    try {
      // Stub `window.print` BEFORE any navigation hits the detail
      // page — addInitScript runs on every navigation, so the stub
      // survives the click into the report detail page.
      await page.addInitScript(() => {
        window.__printCalled = false;
        // eslint-disable-next-line no-undef
        window.print = () => {
          window.__printCalled = true;
        };
      });
      // Re-navigate so the init script applies to the freshly loaded
      // page (signUpFromSpace already called goto, but addInitScript
      // only affects future navigations).
      await goto(page, spaceUrl);
      await dismissDevToast(page);

      await click(page, { testId: "btn-settings" });
      await expect(page.getByTestId("settings-panel")).toHaveAttribute(
        "data-open",
        "true",
      );

      const reports = page.getByTestId("reports-section");
      await expect(reports).toBeVisible({ timeout: 15000 });

      // Click the per-row View Report button.
      const viewBtn = reports
        .locator('[data-testid^="download-report-"]')
        .first();
      await viewBtn.click();

      // Lands on the detail page in viewer mode — verify by URL and
      // by the absence of the publish button.
      await page.waitForURL(/\/spaces\/[a-z0-9-]+\/report\/[^/]+\/?$/, {
        waitUntil: "load",
        timeout: 15000,
      });
      await expect(page.getByTestId("publish-report")).toHaveCount(0);
      // The root container picks up the viewer modifier class — also
      // a useful signal that the role gate did its job.
      await expect(page.locator(".report-detail--viewer")).toBeVisible();

      // PDF Download is visible to viewers.
      const pdfBtn = page.getByTestId("pdf-download");
      await expect(pdfBtn).toBeVisible();

      // Click → the in-component `dioxus::document::eval` schedules a
      // `window.print()` via `setTimeout(50ms)`. Poll for the stub
      // flag to flip rather than relying on exact timing.
      await pdfBtn.click();
      await expect
        .poll(() => page.evaluate(() => window.__printCalled === true), {
          timeout: 5000,
          intervals: [100, 200, 300, 500, 1000],
        })
        .toBe(true);
    } finally {
      await context.close();
    }
  });
});
