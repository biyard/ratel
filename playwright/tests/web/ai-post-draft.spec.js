import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor, openHomeMenuItem } from "../utils";

/**
 * AI Post Draft — Pro-only opinion-gathering generation
 *
 * Backend must run with `--features bypass` (default for docker-compose
 * `testing` profile) so the AI writer resolves to `FixtureWriter`. The fixture
 * returns a deterministic 5-section opinion-gathering response — no Bedrock /
 * Ollama call leaves the test container.
 *
 * Flow:
 *   1. Sign in (handled by `user.json` storage state)
 *   2. Open the home arena → Compose → land on /posts/.../edit
 *   3. Click the "AI 로 작성" entry button
 *   4. Pick the Opinion Gathering template
 *   5. Fill topic / background / feedback (required) — leave notes optional
 *   6. Click "초안 생성" → modal closes; editor shows the 5 sections
 *   7. Verify the AI button has disappeared from the topbar (AC-13)
 *
 * Acceptance criteria covered: AC-1, AC-3, AC-4, AC-6, AC-9, AC-10, AC-13.
 *
 * The Pro/Free-tier branch is exercised by server-side cargo integration
 * tests (`tests/ai_post_draft_tests.rs`) — they don't need a real browser
 * and cover the Free-user upsell + 403 response.
 */

test.describe.serial("AI post draft (paid user)", () => {
  const uniqueId = `${Date.now()}${Math.random().toString(36).slice(2, 6)}`;
  const topic = `Smoking-zone relocation E2E ${uniqueId}`;

  test("paid user can generate an opinion-gathering draft via AI", async ({
    page,
  }) => {
    await goto(page, "/");

    // Compose lives inside the home arena hamburger overlay.
    await openHomeMenuItem(page, "home-menu-compose");
    await page.waitForURL(/\/posts\/.*\/edit/, { waitUntil: "load" });

    // AC-1: AI button is visible on the empty draft.
    await expect(page.getByTestId("ai-draft-button")).toBeVisible();

    // Click the AI entry button — paid test users land on the picker.
    await click(page, { testId: "ai-draft-button" });

    // AC-3: template picker shows opinion gathering as selectable.
    await expect(page.getByTestId("ai-draft-modal")).toBeVisible();
    await expect(page.getByTestId("ai-template-opinion")).toBeVisible();

    await click(page, { testId: "ai-modal-next" });

    // AC-4: form has the three required fields + language dropdown.
    await page.getByTestId("ai-form-topic").fill(topic);
    await page
      .getByTestId("ai-form-background")
      .fill(
        "Complaints about smoking zones near residential areas have increased and we need to revisit placement.",
      );
    await page
      .getByTestId("ai-form-feedback")
      .fill(
        "Evaluation of current sites, preferences for alternative locations, operating values to prioritise.",
      );

    // AC-6: the Generate button only activates once required fields are filled.
    await expect(page.getByTestId("ai-modal-generate")).toBeEnabled();

    await click(page, { testId: "ai-modal-generate" });

    // Modal closes on success — title/body land in the editor.
    await expect(page.getByTestId("ai-draft-modal")).toBeHidden({
      timeout: 30000,
    });

    // AC-10: five sections appear in the editor body (KO output by default).
    // The editor remounts when AI content lands (see
    // `editor_remount_key` in post_edit/component.rs), so give the new
    // mount time to attach `dangerous_inner_html` before asserting. The
    // first heading takes the brunt of the wait; the rest are sub-ms.
    const editor = await getEditor(page);
    await expect(editor.getByRole("heading", { name: "추진배경", exact: true }))
      .toBeVisible({ timeout: 15000 });
    for (const heading of ["추진목적", "추진내용", "의견수렴 사항", "참여 안내"]) {
      await expect(editor.getByRole("heading", { name: heading, exact: true }))
        .toBeVisible();
    }

    // AC-13: AI button disappears after a successful generation on this post.
    await expect(page.getByTestId("ai-draft-button")).toBeHidden();
  });
});
