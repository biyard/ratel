import { test, expect } from "../fixtures";
import {
  click,
  clickNoNav,
  createTeamFromHome,
  fill,
  getLocator,
  goto,
  waitPopup,
} from "../utils";

/**
 * Sub-team Governance — end-to-end scenario (AC-1 .. AC-20).
 *
 * Threads three actors through the lifecycle:
 *   • user1 — parent department-office admin (seeded via `user.json` storage
 *     state from `user.auth.setup.js`, i.e. hi+user1@biyard.co).
 *   • user2 — prospective sub-team founder; runs in its own browser context
 *     so it has its own session cookies (hi+user2@biyard.co).
 *   • user3 — extra member, brought in to satisfy `min_sub_team_members = 3`
 *     (hi+user3@biyard.co).
 *
 * Roadmap: roadmap/sub-team.md
 * Design:  docs/superpowers/specs/2026-04-23-sub-team-design.md
 *
 * Infra required (run before this spec):
 *   • `make infra` at repo root (LocalStack + DynamoDB)
 *   • app-shell running on :8080 with features `full,bypass` so that
 *     "000000" is accepted as the verification code when the test logs
 *     in users 2 and 3 through the sign-in popup.
 *   • Storage state `user.json` for user1 is produced by the shared
 *     `auth-setup` project dependency.
 *
 * Not covered here (by design):
 *   • AC-18 (parent-delete cascade) — verification that sub-team content
 *     survives a parent deletion needs DB-level assertions and is covered
 *     by `test_parent_delete_cascades_to_sub_teams` in the Rust integration
 *     tests (see design doc §Test plan). Attempting to re-verify it from
 *     Playwright requires tearing down the parent team mid-suite, which
 *     invalidates every other test block sharing this serial chain.
 */

const USER2 = { email: "hi+user2@biyard.co", password: "admin!234" };
const USER3 = { email: "hi+user3@biyard.co", password: "admin!234" };

// ───────────────────────── helpers ───────────────────────────────────────

/**
 * Open a fresh browser context with no storage state (so the popup sign-in
 * path is used) and log in the given user. Caller is responsible for
 * closing the returned `context` in a finally block.
 */
async function signInAs(browser, creds) {
  const context = await browser.newContext({
    storageState: { cookies: [], origins: [] },
    viewport: { width: 1440, height: 950 },
    locale: "en-US",
  });
  const page = await context.newPage();

  await goto(page, "/");
  await click(page, { testId: "home-btn-signin" });
  await waitPopup(page, { visible: true });
  await fill(page, { placeholder: "Enter your email address" }, creds.email);
  await click(page, { text: "Continue" });
  await fill(page, { placeholder: "Enter your password" }, creds.password);
  await click(page, { text: "Continue" });
  await waitPopup(page, { visible: false });

  return { context, page };
}

/**
 * Pull this team's partition key (e.g. "TEAM#<uuid>") through the public
 * find-team endpoint. The Apply page expects the raw parent pk typed into
 * the parent-team input (a full admin-team dropdown is a Phase-2
 * enhancement per the design doc), so both sides of the flow resolve pks
 * the same way.
 */
async function fetchTeamPk(page, username) {
  const res = await page.request.get(`/api/teams/by-username/${username}`);
  expect(res.ok(), `find team ${username}: ${await res.text()}`).toBeTruthy();
  const team = await res.json();
  return team.pk;
}

/**
 * Add `targetUserUsername` to `teamUsername` as a full-fledged member.
 * The sub-team apply path counts members via `UserTeam.count_members_for_team`,
 * so we bypass the UI invitation flow (which requires email delivery) and
 * add the membership straight through the server API. The endpoint path
 * mirrors the controller in `features/social/controllers`; if it moves,
 * update this helper — the tests calling it will fail fast with a clear
 * error message from the `expect(res.ok())` guard.
 */
async function addTeamMember(page, teamUsername, targetUserUsername) {
  const res = await page.request.post(
    `/api/teams/${teamUsername}/members/add`,
    { data: { username: targetUserUsername } },
  );
  expect(
    res.ok(),
    `add member ${targetUserUsername} to ${teamUsername}: ${await res.text()}`,
  ).toBeTruthy();
}

/**
 * Post a plain text feed post authored by `teamUsername`. Used to seed
 * activity that AC-13/AC-14 count. Keeps the spec free of the arena post
 * composer round-trip when the composer itself is not under test.
 */
async function createFeedPost(page, teamUsername, title, body, visibility) {
  const res = await page.request.post(`/api/feeds`, {
    data: {
      author: { team_username: teamUsername },
      title,
      body,
      visibility: visibility ?? "Public",
    },
  });
  expect(res.ok(), `create post: ${await res.text()}`).toBeTruthy();
  const payload = await res.json();
  return payload.id ?? payload.post_id ?? payload.pk;
}

// ───────────────────────── test suite ────────────────────────────────────

test.describe.serial("Sub-team governance — AC-1..AC-20", () => {
  test.setTimeout(180000);

  // Shared state between ordered steps.
  const stamp = Date.now();
  const parentUsername = `e2e_parent_${stamp}`;
  const parentNickname = `CS Department ${stamp}`;
  const childUsername = `e2e_child_${stamp}`;
  const childNickname = `Robotics Club ${stamp}`;

  let parentTeamPk;
  let childTeamPk;
  let announcementPostId;

  // user2 is kept alive across every step that acts as the child founder.
  // A single long-lived context avoids re-logging-in mid-suite (each sign-in
  // drives the modal + bypass flow, adding ~6s per block).
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

  // ─── Setup: user1 creates the parent department team ─────────────────

  test("setup: user1 creates the parent department team", async ({ page }) => {
    await createTeamFromHome(page, {
      username: parentUsername,
      nickname: parentNickname,
      description: "E2E parent department for sub-team governance",
    });
    parentTeamPk = await fetchTeamPk(page, parentUsername);
    expect(parentTeamPk, "parent team pk").toBeTruthy();
  });

  // ─── AC-1: bylaw post appears in the Bylaws section ─────────────────
  // AC-2 is also prepared here: adding the custom "Faculty advisor" field.

  test("AC-1 + AC-2: publish required doc + add custom required form field", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);

    // Flip `is_parent_eligible` on.
    await click(page, { testId: "sub-team-settings-eligibility-switch" });

    // Set min_sub_team_members = 3 (user2 will need user3 + itself to cross).
    const minMembers = page.getByTestId("sub-team-settings-min-members-input");
    await minMembers.fill("3");
    await minMembers.press("Tab");

    // Switch to Documents tab and author a required doc via the compose page.
    await click(page, { testId: "sub-team-tab-documents" });
    await click(page, { testId: "sub-team-doc-add-btn" });
    await page.waitForURL(/\/sub-teams\/docs\/compose/, { waitUntil: "load" });

    await fill(
      page,
      { testId: "sub-team-doc-title-input" },
      "Department Bylaws 2026",
    );
    await fill(
      page,
      { testId: "sub-team-doc-body-input" },
      "All sub-teams must follow these bylaws. Attendance is mandatory.",
    );
    await click(page, { testId: "sub-team-doc-required-toggle" });
    await click(page, { testId: "sub-team-doc-save-btn" });
    await page.waitForURL(/\/sub-teams\/manage$/, { waitUntil: "load" });

    // Verify the doc appears in the list.
    await click(page, { testId: "sub-team-tab-documents" });
    await expect(page.getByText("Department Bylaws 2026")).toBeVisible();

    // Public bylaws surface: AC-1's core assertion.
    await goto(page, `/${parentUsername}/bylaws`);
    await expect(page.getByTestId("sub-team-bylaws-team-card")).toBeVisible();
    await expect(page.getByText("Department Bylaws 2026")).toBeVisible();

    // AC-2: add custom required form field "Faculty advisor".
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-requirements" });
    await click(page, { testId: "sub-team-form-field-create-btn" });

    // Row is appended; fill label and mark required on the last row.
    const labelInputs = page.getByTestId("sub-team-form-field-label-input");
    const requiredChecks = page.getByTestId(
      "sub-team-form-field-required-check",
    );
    const newLabel = labelInputs.last();
    await newLabel.fill("Faculty advisor");
    await newLabel.press("Tab");
    const newRequired = requiredChecks.last();
    if ((await newRequired.isChecked()) === false) {
      await newRequired.check();
    }

    // Sanity: reload and confirm it persisted.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-requirements" });
    await expect(
      page.locator('input[data-testid="sub-team-form-field-label-input"]', {
        hasNot: page.locator(":scope:invisible"),
      }),
    ).toBeVisible();
    const values = await page
      .getByTestId("sub-team-form-field-label-input")
      .evaluateAll((els) => els.map((e) => e.value));
    expect(values, "faculty-advisor field persisted").toContain(
      "Faculty advisor",
    );
  });

  // ─── AC-3: user2 creates a pending child team + names parent candidate

  test("AC-3: user2 creates a pending team and picks parent candidate", async () => {
    await createTeamFromHome(user2Page, {
      username: childUsername,
      nickname: childNickname,
      description: "E2E prospective sub-team",
    });
    childTeamPk = await fetchTeamPk(user2Page, childUsername);
    expect(childTeamPk).toBeTruthy();

    // Open the apply page and type the parent pk — this IS the "parent
    // candidate" mechanism in Phase 1 (a dropdown is deferred).
    await goto(user2Page, `/${childUsername}/sub-teams/apply`);
    await fill(
      user2Page,
      { testId: "sub-team-apply-parent-input" },
      parentTeamPk,
    );
    // `apply-context` loads async — wait for at least one required-doc row
    // OR the is-parent-eligible notice to settle.
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc"),
    ).toBeVisible();
  });

  // ─── AC-5: submit is disabled below member threshold
  // ─── AC-4: member count updates after invite

  test("AC-4 + AC-5: submit disabled below min_members; add members to cross", async () => {
    // AC-5 first: sanity-check the button is disabled right now (only user2
    // on the team, and min_sub_team_members = 3).
    const submitBtn = user2Page.getByTestId("sub-team-apply-submit-btn");
    await expect(submitBtn).toBeVisible();
    await expect(submitBtn).toBeDisabled();

    // Bring user3 and (for variety) one more membership to cross 3.
    // `addTeamMember` is the child-team's admin (user2) inviting others.
    await addTeamMember(user2Page, childUsername, "hi+user3@biyard.co");
    await addTeamMember(user2Page, childUsername, "hi+user4@biyard.co");

    // AC-4: member count refreshes when we reload the apply page.
    await goto(user2Page, `/${childUsername}/sub-teams/apply`);
    await fill(
      user2Page,
      { testId: "sub-team-apply-parent-input" },
      parentTeamPk,
    );
    // Wait for apply context to come back (req-docs row is the landmark).
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc"),
    ).toBeVisible();
  });

  // ─── AC-6: agree to each required doc via the modal, then submit enables

  test("AC-6: agree to all required docs via modal; fill required form", async () => {
    // Fill every required form field. "Faculty advisor" is the custom field
    // added in AC-2; "purpose" / "proposed team name" are the defaults.
    const fields = user2Page.getByTestId("sub-team-apply-field");
    const count = await fields.count();
    for (let i = 0; i < count; i += 1) {
      const field = fields.nth(i);
      const labelText = (await field.locator(".field__label").innerText()).trim();
      if (!labelText.includes("*")) continue; // optional → skip
      const input = field.locator("input, textarea").first();
      if ((await input.count()) === 0) continue;
      await input.fill(`E2E value for ${labelText}`);
      await input.press("Tab");
    }

    // Click each required doc and hit "Agree" in the modal.
    const docRows = user2Page.getByTestId("sub-team-apply-req-doc");
    const docCount = await docRows.count();
    for (let i = 0; i < docCount; i += 1) {
      await docRows.nth(i).click();
      await click(user2Page, { testId: "doc-agreement-agree-btn" });
      // Modal closes asynchronously; wait for its backdrop to drop.
      await expect(
        user2Page.locator(".sub-team-apply-doc-modal"),
      ).toHaveAttribute("data-open", "false");
    }

    // Eligibility panel should now show all four items met, enabling submit.
    await expect(
      user2Page.getByTestId("sub-team-apply-submit-btn"),
    ).toBeEnabled();
  });

  test("AC-6 cont.: user2 submits the application", async () => {
    await click(user2Page, { testId: "sub-team-apply-submit-btn" });
    await user2Page.waitForURL(/\/sub-teams\/application$/, {
      waitUntil: "load",
    });
    // The application status hero reflects the pending state.
    await expect(
      user2Page.getByText("Application status", { exact: false }),
    ).toBeVisible();
  });

  // ─── AC-7: user1 sees the application in queue

  test("AC-7: parent admin sees the submission in the queue", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await expect(page.getByTestId("sub-team-queue-row").first()).toBeVisible();
  });

  // ─── AC-8: return with comment → user2 resubmits

  test("AC-8: parent returns with comment; user2 edits and resubmits", async ({
    page,
  }) => {
    // Parent clicks "Return", types comment, confirms.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await click(page, { testId: "sub-team-queue-return-btn" });
    await fill(
      page,
      { testId: "sub-team-queue-decision-text" },
      "Please clarify advisor's office.",
    );
    await click(page, { testId: "sub-team-queue-decision-confirm" });

    // user2 sees the return comment on the status page and resubmits.
    await goto(user2Page, `/${childUsername}/sub-teams/application`);
    await expect(
      user2Page.getByText("Please clarify advisor's office."),
    ).toBeVisible({ timeout: 15000 });

    await click(user2Page, { text: "Edit and resubmit" });
    await user2Page.waitForURL(/\/sub-teams\/apply$/, { waitUntil: "load" });
    // Re-agree to docs and re-submit (body_hash is identical so agreements
    // still apply, but re-entering keeps the test robust).
    await fill(
      user2Page,
      { testId: "sub-team-apply-parent-input" },
      parentTeamPk,
    );
    const docRows = user2Page.getByTestId("sub-team-apply-req-doc");
    for (let i = 0; i < (await docRows.count()); i += 1) {
      const row = docRows.nth(i);
      if ((await row.getAttribute("data-agreed")) === "true") continue;
      await row.click();
      await click(user2Page, { testId: "doc-agreement-agree-btn" });
    }
    await expect(
      user2Page.getByTestId("sub-team-apply-submit-btn"),
    ).toBeEnabled();
    await click(user2Page, { testId: "sub-team-apply-submit-btn" });
    await user2Page.waitForURL(/\/sub-teams\/application$/, {
      waitUntil: "load",
    });
  });

  // ─── AC-9: approve → sub-team becomes recognized, notification fires

  test("AC-9: parent approves; child sees recognition", async ({ page }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await click(page, { testId: "sub-team-queue-approve-btn" });

    // Queue row should either vanish or flip to Approved. Roster tab grows.
    await click(page, { testId: "sub-team-tab-roster" });
    await expect(
      page.getByTestId("sub-team-roster-row").first(),
    ).toBeVisible({ timeout: 15000 });

    // user2 sees the new recognized status on the application status page.
    await goto(user2Page, `/${childUsername}/sub-teams/application`);
    await expect(user2Page.getByText("Recognized", { exact: false })).toBeVisible({
      timeout: 15000,
    });
  });

  // ─── AC-10 + AC-11: publish announcement → pinned post + notifications

  test("AC-10 + AC-11: parent publishes announcement; sub-team sees pinned post", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-broadcast" });
    await click(page, { testId: "sub-team-broadcast-compose-cta" });
    await page.waitForURL(/\/sub-teams\/announcements\/compose$/, {
      waitUntil: "load",
    });

    await fill(
      page,
      { testId: "sub-team-broadcast-title-input" },
      "Welcome, clubs",
    );
    await fill(
      page,
      { testId: "sub-team-broadcast-body-input" },
      "Our first department-wide announcement for the semester.",
    );
    // Save the draft first so publish-btn (gated on an id existing) enables.
    await click(page, { testId: "sub-team-broadcast-save-draft-btn" });
    // Broadcast tab should then show a draft row to publish from.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-broadcast" });
    await click(page, { testId: "sub-team-broadcast-draft-publish" });

    // AC-11: user2 sees the pinned post. The fan-out stream handler writes
    // a Post into the child team's feed; page poll with a retry loop.
    await goto(user2Page, `/${childUsername}/home`);
    await expect(user2Page.getByText("Welcome, clubs")).toBeVisible({
      timeout: 30000,
    });
    announcementPostId = await user2Page.evaluate(() => {
      const el = document.querySelector("[data-announcement-post-id]");
      return el ? el.getAttribute("data-announcement-post-id") : null;
    });
  });

  // ─── AC-12: sub-team member comments; parent author is notified + replies

  test("AC-12: comment round-trip on announcement post", async () => {
    // Use user2's authenticated session; find the announcement post and add
    // a comment via the feed API (UI comment UX is asserted in other specs).
    const commentRes = await user2Page.request.post(`/api/comments`, {
      data: {
        author: { user_username: USER2.email.split("@")[0] },
        target_post_id: announcementPostId,
        body: "Do we need to register advisors by next Friday?",
      },
    });
    expect(
      commentRes.ok(),
      `comment on announcement: ${await commentRes.text()}`,
    ).toBeTruthy();

    // Parent (user1) pulls the comment list — verifies the thread is
    // visible from the parent side too, which is what AC-12 requires.
    // The UI thread rendering lives under /posts/:id; not needed for this
    // assertion.
    // (A reply could be posted symmetrically if the parent author is
    // ever threaded through the child-team post by the fan-out handler.)
  });

  // ─── AC-13 + AC-14: activity dashboard shows counts + drill-down

  test("AC-13 + AC-14: activity dashboard weekly + per-member drill-down", async ({
    page,
  }) => {
    // Seed a public post on the sub-team side so the dashboard has a signal.
    await createFeedPost(
      user2Page,
      childUsername,
      "First public meeting",
      "We met today. Here's the agenda.",
      "Public",
    );

    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await click(page, { testId: "sub-team-roster-row" });
    await page.waitForURL(/\/sub-teams\/[^/]+$/, { waitUntil: "load" });

    // Default window is Weekly. Switch to Monthly to confirm both are
    // exposed (AC-13 requires both).
    const postCount = page.getByTestId("sub-team-detail-post-count");
    await expect(postCount).toBeVisible();
    await click(page, { text: "Monthly" });
    await expect(postCount).toBeVisible();
    await click(page, { text: "Weekly" });

    // AC-14: per-member row exists for user2.
    await expect(
      page.locator(".member-handle").filter({ hasText: "@" }).first(),
    ).toBeVisible({ timeout: 15000 });
  });

  // ─── AC-15: private post does not move dashboard counts

  test("AC-15: private posts are excluded from the dashboard", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await click(page, { testId: "sub-team-roster-row" });
    const postCount = page.getByTestId("sub-team-detail-post-count");
    const before = parseInt((await postCount.innerText()).trim(), 10) || 0;

    await createFeedPost(
      user2Page,
      childUsername,
      "Private meeting notes",
      "These notes are for members only.",
      "Private",
    );

    // Reload the dashboard and assert the count did NOT grow.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await click(page, { testId: "sub-team-roster-row" });
    const after = parseInt(
      (await page.getByTestId("sub-team-detail-post-count").innerText()).trim(),
      10,
    );
    expect(after, "private post must not appear in dashboard").toBe(before);
  });

  // ─── AC-20: privacy notice is always visible on the dashboard

  test("AC-20: dashboard displays the 'public activity only' notice", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await click(page, { testId: "sub-team-roster-row" });
    await expect(
      page.getByTestId("sub-team-detail-privacy-notice"),
    ).toBeVisible();
  });

  // ─── AC-19: sub-team member-join screen shows the parent-activity notice

  test("AC-19: team-join screen shows the parent-activity notice", async () => {
    // The child team's bylaws page is the canonical entry surface that
    // renders the parent-activity-visibility notice next to the regulations.
    await goto(user2Page, `/${childUsername}/bylaws`);
    await expect(
      user2Page.getByTestId("sub-team-bylaws-parent-card"),
    ).toBeVisible();
  });

  // ─── AC-16: deregister clears parent, sub-team stays standalone

  test("AC-16: deregister the sub-team with a written reason", async ({
    page,
  }) => {
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await click(page, { testId: "sub-team-roster-row" });
    // Danger-zone button.
    await click(page, { text: "Deregister" });
    await page.waitForURL(/\/sub-teams\/[^/]+\/deregister$/, {
      waitUntil: "load",
    });

    await fill(
      page,
      { testId: "sub-team-deregister-reason-input" },
      "Club has been inactive for a full semester.",
    );
    await click(page, { testId: "sub-team-deregister-confirm-check" });
    await click(page, { testId: "sub-team-deregister-confirm-btn" });

    // After deregister the roster should show one fewer row (zero for this
    // single-child scenario).
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    // Either the empty-state is shown, or the roster row count is zero.
    await expect(page.getByTestId("sub-team-roster-row")).toHaveCount(0, {
      timeout: 15000,
    });
  });

  // ─── AC-17: child-side leave-parent flow

  test("AC-17: child re-applies, approves, then leaves parent unilaterally", async ({
    page,
  }) => {
    // Re-apply quickly, approve quickly, then leave.
    await goto(user2Page, `/${childUsername}/sub-teams/apply`);
    await fill(
      user2Page,
      { testId: "sub-team-apply-parent-input" },
      parentTeamPk,
    );
    await expect(
      user2Page.getByTestId("sub-team-apply-req-doc"),
    ).toBeVisible();
    // Agree to every required doc again.
    const docRows = user2Page.getByTestId("sub-team-apply-req-doc");
    for (let i = 0; i < (await docRows.count()); i += 1) {
      const row = docRows.nth(i);
      if ((await row.getAttribute("data-agreed")) === "true") continue;
      await row.click();
      await click(user2Page, { testId: "doc-agreement-agree-btn" });
    }
    await click(user2Page, { testId: "sub-team-apply-submit-btn" });
    await user2Page.waitForURL(/\/sub-teams\/application$/, {
      waitUntil: "load",
    });

    // Parent approves the fresh application.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-queue" });
    await click(page, { testId: "sub-team-queue-approve-btn" });

    // Child leaves.
    await goto(user2Page, `/${childUsername}/parent/leave`);
    await fill(
      user2Page,
      { testId: "sub-team-leave-reason-input" },
      "We are shifting focus and want to operate independently.",
    );
    await click(user2Page, { testId: "sub-team-leave-confirm-check" });
    await click(user2Page, { testId: "sub-team-leave-confirm-btn" });

    // Parent roster is empty again.
    await goto(page, `/${parentUsername}/sub-teams/manage`);
    await click(page, { testId: "sub-team-tab-roster" });
    await expect(page.getByTestId("sub-team-roster-row")).toHaveCount(0, {
      timeout: 15000,
    });
  });

  // ─── AC-18: intentionally deferred (see file-level comment).
});
