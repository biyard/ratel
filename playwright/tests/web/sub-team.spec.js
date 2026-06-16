import { test, expect } from "../fixtures";
import {
  click,
  clickNoNav,
  createTeamFromHome,
  fill,
  getEditor,
  goto,
} from "../utils";

/**
 * Sub-team governance — 12-step end-to-end flow.
 *
 *  1. 상위팀: application form + 운영 수칙 작성, 신청 받기 ON
 *  2. 하위팀: 상위팀에 application 작성
 *  3. 상위팀: 수정 요청 (Return)
 *  4. 하위팀: 수정 후 재제출
 *  5. 상위팀: Approve
 *  6. 하위팀: Approve 상태 확인
 *  7. 상위팀: 공지 broadcast
 *  8. 상위팀: 운영 수칙 재작성 (doc edit)
 *  9. 하위팀: 탈퇴 (leave parent)
 * 10. 하위팀: 재신청
 * 11. 상위팀: 재승인
 * 12. 상위팀: 하위팀 강제 탈퇴 (deregister)
 *
 * Two actors:
 *   • user1 — parent department-team admin (default storage state).
 *   • user2 — sub-team founder; runs in its own browser context.
 *
 * Prereqs to run locally:
 *   • `make infra` at repo root.
 *   • app-shell built with `full,bypass` (verification code "000000").
 */

const USER2 = { email: "hi+user2@biyard.co" };

// ───────────────────────── helpers ───────────────────────────────────────

async function signInAs(browser, creds) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, "/");
  await click(page, { testId: "home-btn-signin" });
  await expect(page.getByPlaceholder("Enter your email address")).toBeVisible({
    timeout: 15000,
  });
  // Passwordless email-code login for an existing user: email → Continue
  // (sends code) → code "000000" → Continue (verifies). The bypass code
  // "000000" is accepted under --features bypass.
  await fill(page, { placeholder: "Enter your email address" }, creds.email);
  await click(page, { testId: "continue-button" });
  await fill(page, { testId: "code-input" }, "000000");
  await click(page, { testId: "continue-button" });

  return { context, page };
}

/**
 * Open the apply page's team picker and select the team identified by
 * `teamUsername`. The page's auto-pick effect seeds the first admin
 * team, but when the viewer has leftover teams from prior runs the
 * first item is often *not* the one we just created — submit then
 * fails with "ApplicationInFlight". Explicitly picking the freshly
 * created team avoids that drift.
 */
async function pickApplicantTeam(page, teamUsername) {
  const trigger = page.getByTestId("sub-team-apply-picker-trigger");
  // Picker collapses on close, so items only exist while the dropdown is
  // open. Fast-path: if the trigger already shows the right `@handle`,
  // skip opening.
  const triggerText = (await trigger.innerText()).trim();
  if (triggerText.includes(`@${teamUsername}`)) {
    return;
  }
  await trigger.click();
  const item = page.getByTestId(`sub-team-apply-picker-item-${teamUsername}`);
  await expect(item).toBeVisible({ timeout: 15000 });
  await item.click();
  await expect(trigger).toContainText(`@${teamUsername}`, { timeout: 5000 });
}

/**
 * Agree to every required-doc row that isn't already agreed.
 * The Apply page lists `sub-team-apply-req-doc` rows; clicking one
 * opens the agreement modal, where `doc-agreement-agree-btn` confirms.
 */
async function agreeAllRequiredDocs(page) {
  const docRows = page.getByTestId("sub-team-apply-req-doc");
  const count = await docRows.count();
  for (let i = 0; i < count; i += 1) {
    const row = docRows.nth(i);
    if ((await row.getAttribute("data-agreed")) === "true") continue;
    await row.click();
    // After the modal opens, the agree button is `disabled` when
    // `agreed_doc_ids` was hydrated from a prior Returned application
    // — but the outer row's `data-agreed` can still read "false" mid-
    // hydration, so we'd land here even for already-agreed docs.
    // Branch on the button's live state instead of trying to click a
    // disabled button (which `click` would wait on forever).
    const agreeBtn = page.getByTestId("doc-agreement-agree-btn");
    await expect(agreeBtn).toBeVisible({ timeout: 5000 });
    if (await agreeBtn.isDisabled()) {
      // Already agreed — close the modal via its X button instead of
      // re-confirming. (Escape isn't bound; the cancel button has no
      // testid and uses translated text.)
      await page.locator(".doc-modal__close-x").first().click();
    } else {
      await agreeBtn.click();
    }
    // Wait for backdrop to close before iterating to the next doc.
    await expect(
      page.locator(".sub-team-apply-doc-modal"),
    ).toHaveAttribute("data-open", "false", { timeout: 5000 });
  }
}

/**
 * Fill every required (* labeled) form field with a placeholder value.
 * Fields are rendered as `.field` rows inside `.sub-team-apply-field`.
 */
async function fillRequiredFormFields(page) {
  const fields = page.getByTestId("sub-team-apply-field");
  const count = await fields.count();
  for (let i = 0; i < count; i += 1) {
    const field = fields.nth(i);
    const labelText = (await field.locator(".field__label").innerText()).trim();
    if (!labelText.includes("*")) continue;
    const input = field.locator("input, textarea").first();
    if ((await input.count()) === 0) continue;
    const current = await input.inputValue().catch(() => "");
    if (current && current.trim().length > 0) continue;
    await input.fill(`E2E ${labelText.replace(/\s*\*\s*$/, "").trim()}`);
    await input.press("Tab");
  }
}

// ───────────────────────── test suite ────────────────────────────────────

test.describe.serial("Sub-team governance — 12-step flow", () => {
  test.setTimeout(180000);

  const stamp = Date.now();
  const parentUsername = `e2e_parent_${stamp}`;
  const parentNickname = `Department ${stamp}`;
  const childUsername = `e2e_child_${stamp}`;
  const childNickname = `Robotics Club ${stamp}`;

  // user2 keeps a long-lived browser context so sign-in cost is paid once.
  let user2Ctx;
  let user2Page;

  test.beforeAll(async ({ browser }) => {
    const setup = await signInAs(browser, USER2);
    user2Ctx = setup.context;
    user2Page = setup.page;
  });

  test.afterAll(async () => {
    if (user2Ctx) {
      await user2Ctx.close();
    }
  });

  // ─── Step 1: parent creates form + bylaws + opens applications ───────

  test("Step 1: parent creates team, sets requirements, drafts bylaws", async ({
    page,
  }) => {
    // 1a. user1 creates the parent team.
    await createTeamFromHome(page, {
      username: parentUsername,
      nickname: parentNickname,
      description: "E2E parent department for sub-team governance",
    });

    // 1b. Open sub-team management; flip "open applications" ON.
    // (Leaving `min_sub_team_members` at its default 0 keeps user2 — a
    // freshly-created solo team — eligible to apply without dragging in
    // a separate seeded-account dependency.)
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-settings-eligibility-switch" });

    // 1c. Add a custom required form field "Faculty advisor".
    await click(page, { testId: "sub-team-tab-requirements" });
    const rowsBefore = await page
      .getByTestId("sub-team-form-field-row")
      .count();
    await clickNoNav(page, { testId: "sub-team-form-field-create-btn" });
    // The create action does an `await create_handler` then `fields.restart()`,
    // so we wait for the row count to grow before targeting the new row.
    await expect
      .poll(
        async () => await page.getByTestId("sub-team-form-field-row").count(),
        { timeout: 15000 }
      )
      .toBe(rowsBefore + 1);
    // The new row is the only one with `data-locked="false"`; default
    // fields ("팀 이름", "설립 목적") are seeded with `locked: true`.
    const newRow = page
      .getByTestId("sub-team-form-field-row")
      .filter({ has: page.locator('.field-row[data-locked="false"]') })
      .last();
    const newLabel = newRow.getByTestId("sub-team-form-field-label-input");
    await newLabel.fill("Faculty advisor");
    await newLabel.press("Tab");
    const newRequired = newRow.getByTestId(
      "sub-team-form-field-required-check"
    );
    if (!(await newRequired.isChecked())) {
      await newRequired.click({ force: true });
    }

    // 1d. Author a required bylaws doc.
    await click(page, { testId: "sub-team-tab-documents" });
    await click(page, { testId: "sub-team-doc-add-btn" });
    await page.waitForURL(/\/bylaws\/compose\//, { waitUntil: "load" });

    await fill(
      page,
      { testId: "sub-team-doc-title-input" },
      "Department Bylaws"
    );
    const editor = await getEditor(page);
    await editor.fill(
      "All sub-teams must follow these bylaws. Attendance is mandatory."
    );
    // Input is wrapped in a label with an overlaying `.switch` span
    // (pointer-events on the span intercept clicks on the hidden input).
    // Use `force: true` so Playwright dispatches the click straight at
    // the input regardless of the overlay.
    await page
      .getByTestId("sub-team-doc-required-toggle")
      .click({ force: true });
    await click(page, { testId: "sub-team-doc-save-btn" });
    await page.waitForURL(/\/sub-teams\/manage(\?|$|#)/, {
      waitUntil: "load",
    });

    // Sanity: bylaws now show up on the public bylaws page.
    await goto(page, `/${parentUsername}/bylaws`);
    await expect(page.getByText("Department Bylaws")).toBeVisible({
      timeout: 15000,
    });
  });

  // ─── Step 2: child team applies ──────────────────────────────────────

  test("Step 2: user2 creates the child team and applies", async () => {
    await createTeamFromHome(user2Page, {
      username: childUsername,
      nickname: childNickname,
      description: "E2E prospective sub-team",
    });

    // Apply flow lives at the PARENT's apply route. The page's team-picker
    // auto-pick can latch onto a leftover team from a previous run, so we
    // explicitly select the team we just created.
    await goto(user2Page, `/${parentUsername}/sub-teams/apply`);
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc").first()
    ).toBeVisible({ timeout: 15000 });
    await pickApplicantTeam(user2Page, childUsername);

    await agreeAllRequiredDocs(user2Page);
    await fillRequiredFormFields(user2Page);

    const submitBtn = user2Page.getByTestId("sub-team-apply-submit-btn");
    await expect(submitBtn).toBeEnabled({ timeout: 15000 });
    await submitBtn.click();
    await user2Page.waitForURL(/\/sub-teams\/application(\?|$|#)/, {
      waitUntil: "load",
    });
  });

  // ─── Step 3: parent returns the application with a comment ───────────

  test("Step 3: parent returns the application with a comment", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await expect(page.getByTestId("sub-team-queue-row").first()).toBeVisible({
      timeout: 15000,
    });

    await click(page, { testId: "sub-team-queue-return-btn" });
    await fill(
      page,
      { testId: "sub-team-queue-decision-text" },
      "Please clarify advisor's office."
    );
    await click(page, { testId: "sub-team-queue-decision-confirm" });
  });

  // ─── Step 4: child edits and resubmits ───────────────────────────────

  test("Step 4: user2 edits the application and resubmits", async () => {
    // Application status URL is keyed on the **parent** username — the
    // route reads `:username` as "the parent I applied to" and the loader
    // looks up the viewer's application targeting that parent.
    await goto(user2Page, `/${parentUsername}/sub-teams/application`);
    await expect(
      user2Page.getByText("Please clarify advisor's office.")
    ).toBeVisible({ timeout: 15000 });

    await click(user2Page, { text: "Edit and resubmit" });
    await user2Page.waitForURL(/\/sub-teams\/apply(\?|$|#)/, {
      waitUntil: "load",
    });
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc").first()
    ).toBeVisible({ timeout: 15000 });
    await pickApplicantTeam(user2Page, childUsername);

    await agreeAllRequiredDocs(user2Page);
    await fillRequiredFormFields(user2Page);

    const submitBtn = user2Page.getByTestId("sub-team-apply-submit-btn");
    await expect(submitBtn).toBeEnabled({ timeout: 15000 });
    await submitBtn.click();
    await user2Page.waitForURL(/\/sub-teams\/application(\?|$|#)/, {
      waitUntil: "load",
    });
  });

  // ─── Step 5: parent approves ─────────────────────────────────────────

  test("Step 5: parent approves the application", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await expect(page.getByTestId("sub-team-queue-row").first()).toBeVisible({
      timeout: 15000,
    });

    // Approve button opens an inline welcome-message textarea; the
    // confirm button fires the actual mutation.
    await click(page, { testId: "sub-team-queue-approve-btn" });
    await click(page, { testId: "sub-team-queue-decision-confirm" });

    // Roster grows after approval.
    await click(page, { testId: "sub-team-tab-roster" });
    await expect(page.getByTestId("sub-team-roster-row").first()).toBeVisible({
      timeout: 15000,
    });
  });

  // ─── Step 6: child sees recognized state ─────────────────────────────

  test("Step 6: user2 confirms recognized status", async () => {
    await goto(user2Page, `/${parentUsername}/sub-teams/application`);
    await expect(
      user2Page.getByText("Recognized", { exact: false }).first()
    ).toBeVisible({ timeout: 15000 });
  });

  // ─── Step 7: parent publishes a broadcast announcement ───────────────

  test("Step 7: parent publishes a broadcast", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-broadcast" });
    await click(page, { testId: "sub-team-broadcast-compose-cta" });
    await page.waitForURL(/\/sub-teams\/announcements\/compose(\?|$|#)/, {
      waitUntil: "load",
    });

    await fill(
      page,
      { testId: "sub-team-broadcast-title-input" },
      "Welcome, clubs"
    );
    const editor = await getEditor(page);
    await editor.fill(
      "Our first department-wide announcement for the semester."
    );

    // Autosave persists the draft after a 500ms debounce; wait for the
    // autosave chip to flip from "Saving..." back to "Saved" so the
    // publish-btn unblocks (it's `aria-disabled` until an id exists).
    await expect(
      page.locator('.autosave-chip[data-state="saved"]')
    ).toBeVisible({ timeout: 15000 });

    // Publish — anchor with onclick + href to /sub-teams/manage. The
    // anchor's href full-navigates to the management page; assert the
    // URL after click.
    await page.getByTestId("sub-team-broadcast-publish-btn").click();
    await page.waitForURL(/\/sub-teams\/manage(\?|$|#)/, {
      waitUntil: "load",
    });
  });

  // ─── Step 8: parent re-edits the bylaws ──────────────────────────────

  test("Step 8: parent edits the existing bylaws", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-documents" });
    await expect(page.getByTestId("sub-team-doc-item").first()).toBeVisible({
      timeout: 15000,
    });

    await page.getByTestId("sub-team-doc-edit-btn").first().click();
    await page.waitForURL(/\/sub-teams\/docs\/[^/]+\/edit(\?|$|#)/, {
      waitUntil: "load",
    });

    // Replace title.
    const titleInput = page.getByTestId("sub-team-doc-title-input");
    await titleInput.fill("Department Bylaws — v2");
    const editor = await getEditor(page);
    await editor.fill("Bylaws revised. Effective immediately.");
    await click(page, { testId: "sub-team-doc-save-btn" });
    await page.waitForURL(/\/sub-teams\/manage(\?|$|#)/, {
      waitUntil: "load",
    });

    await click(page, { testId: "sub-team-tab-documents" });
    // Doc title renders inside a readonly `<input value="...">`, so
    // `getByText` won't match. Assert the input value directly.
    await expect
      .poll(
        async () =>
          await page.getByTestId("sub-team-doc-title").first().inputValue(),
        { timeout: 15000 }
      )
      .toBe("Department Bylaws — v2");
  });

  // ─── Step 9: child leaves parent ─────────────────────────────────────

  test("Step 9: user2 leaves the parent", async () => {
    await goto(user2Page, `/${childUsername}/parent/leave`);
    await fill(
      user2Page,
      { testId: "sub-team-leave-reason-input" },
      "Shifting focus to operate independently."
    );
    await user2Page
      .getByTestId("sub-team-leave-confirm-check")
      .click({ force: true });
    await click(user2Page, { testId: "sub-team-leave-confirm-btn" });

    // Parent's roster should drop the child after the leave settles.
    // Sub-team handlers are awaited server-side, so a fresh goto is enough.
  });

  // ─── Step 10: child re-applies ───────────────────────────────────────

  test("Step 10: user2 re-applies", async () => {
    await goto(user2Page, `/${parentUsername}/sub-teams/apply`);
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc").first()
    ).toBeVisible({ timeout: 15000 });
    await pickApplicantTeam(user2Page, childUsername);

    await agreeAllRequiredDocs(user2Page);
    await fillRequiredFormFields(user2Page);

    const submitBtn = user2Page.getByTestId("sub-team-apply-submit-btn");
    await expect(submitBtn).toBeEnabled({ timeout: 15000 });
    await submitBtn.click();
    await user2Page.waitForURL(/\/sub-teams\/application(\?|$|#)/, {
      waitUntil: "load",
    });
  });

  // ─── Step 11: parent approves again ──────────────────────────────────

  test("Step 11: parent approves the re-application", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await expect(page.getByTestId("sub-team-queue-row").first()).toBeVisible({
      timeout: 15000,
    });
    await click(page, { testId: "sub-team-queue-approve-btn" });
    await click(page, { testId: "sub-team-queue-decision-confirm" });

    await click(page, { testId: "sub-team-tab-roster" });
    await expect(page.getByTestId("sub-team-roster-row").first()).toBeVisible({
      timeout: 15000,
    });
  });

  // ─── Step 12: parent deregisters the child ───────────────────────────

  test("Step 12: parent deregisters the sub-team", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    const rosterRow = page.getByTestId("sub-team-roster-row").first();
    await expect(rosterRow).toBeVisible({ timeout: 15000 });
    await rosterRow.click();
    await page.waitForURL(/\/sub-teams\/[^/]+(\?|$|#)/, { waitUntil: "load" });

    await click(page, { testId: "sub-team-detail-deregister-btn" });
    await page.waitForURL(/\/sub-teams\/[^/]+\/deregister(\?|$|#)/, {
      waitUntil: "load",
    });

    await fill(
      page,
      { testId: "sub-team-deregister-reason-input" },
      "Club has been inactive for a full semester."
    );
    await page
      .getByTestId("sub-team-deregister-confirm-check")
      .click({ force: true });
    await click(page, { testId: "sub-team-deregister-confirm-btn" });

    // Roster empties after deregister.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await expect(page.getByTestId("sub-team-roster-row")).toHaveCount(0, {
      timeout: 15000,
    });
  });
});
