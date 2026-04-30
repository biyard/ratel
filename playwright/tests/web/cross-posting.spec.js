import { test, expect } from "@playwright/test";
import { goto, click } from "../utils";

/**
 * Cross-posting Phase 1A — frontend coverage.
 *
 * Phase 1A scope check (`roadmap/cross-posting.md`):
 *   AC-8  Compose page renders cross-post sidebar for connected platforms.
 *   AC-9  Disabling a platform drops "Reaching N networks" by 1.
 *   AC-10 Body > 300 chars triggers Bluesky truncation warning.
 *   AC-11 Private visibility hides the sidebar entirely.
 *
 * Out of scope here (deferred until the platform-adapter mock harness lands):
 *   AC-1  Onboarding interstitial (Phase 1D).
 *   AC-3  LinkedIn OAuth (Phase 1B).
 *   AC-12 Publish → fan-out → Published — needs Bluesky / LinkedIn mocks
 *         + EventBridge pipes wired in test infra.
 *   AC-13–AC-29 — same dependency on platform mocks + Stage 3/4 sweepers.
 *
 * The author-only post-detail panel (AC-29 visibility, E3) is checked at
 * the server-function layer in `app/ratel/src/tests/cross_posting_tests.rs`
 * (`test_get_syndication_panel_other_users_post_rejected`); rendering is
 * a `Loader<Option<…>>` short-circuit on `None`, so duplicating it here
 * would just retread the unit test.
 */

test.describe.serial("Cross-posting Phase 1A — compose sidebar", () => {
  let postId;

  test("Setup: create a draft post via REST", async ({ page }) => {
    const res = await page.request.post("/api/posts", { data: {} });
    expect(
      res.ok(),
      `create draft post: ${await res.text()}`
    ).toBeTruthy();
    const pk = (await res.json()).post_pk;
    // `Partition::Feed` serializes as `FEED#<id>`; strip the prefix to get
    // the inner post id the route accepts.
    postId = pk.includes("#") ? pk.split("#")[1] : pk;
    expect(postId).toBeTruthy();
  });

  test("Public visibility renders the cross-post sidebar (AC-8)", async ({
    page,
  }) => {
    await goto(page, `/posts/${postId}/edit`);

    // Sidebar root from `compose_sidebar/component.rs` (mockup class
    // preserved). Visibility defaults to Public on a fresh draft, so the
    // sidebar should be present without any toggle action.
    await expect(page.locator(".crosspost").first()).toBeVisible({
      timeout: 10000,
    });
    await expect(
      page.locator(".crosspost-head__title-accent").first()
    ).toBeVisible();

    // Three Phase-1A platform cards (Bluesky / LinkedIn / Threads). They
    // render even when the user has no SocialConnection rows — disconnected
    // ones show the `connect-cta` instead of a switch.
    await expect(
      page.locator('.pp-card[data-platform="bluesky"]')
    ).toBeVisible();
    await expect(
      page.locator('.pp-card[data-platform="linkedin"]')
    ).toBeVisible();
    await expect(
      page.locator('.pp-card[data-platform="threads"]')
    ).toBeVisible();
  });

  test("Switching to Private hides the sidebar (AC-11)", async ({ page }) => {
    await goto(page, `/posts/${postId}/edit`);

    // The post-edit shell on desktop renders the side-panel inline; on
    // mobile it's a drawer. Playwright's default viewport is desktop, so
    // the visibility option is reachable without opening a drawer.
    await click(page, { text: "Private" });

    // Sidebar gone. `if matches!(visibility(), Visibility::Public)` guards
    // the mount in post_edit/component.rs.
    await expect(page.locator(".crosspost")).toBeHidden();
  });

  test("Disconnected platforms render the Connect CTA", async ({ page }) => {
    await goto(page, `/posts/${postId}/edit`);

    // Restore Public visibility so the sidebar mounts again.
    await click(page, { text: "Public" });
    await expect(page.locator(".crosspost").first()).toBeVisible();

    // The pre-auth user has no `SocialConnection` rows seeded (Playwright
    // can't seed one until the BYPASS_PLATFORM_API=mock harness lands —
    // see notes at the top of this file). With no connection, every card
    // falls through to the `connect-cta` branch in
    // `compose_sidebar/component.rs`.
    await expect(
      page.locator('.pp-card[data-platform="bluesky"] .connect-cta__btn')
    ).toBeVisible();
    await expect(
      page.locator('.pp-card[data-platform="bluesky"] .pp-name__status')
    ).toContainText(/Not connected|연결 안 됨/);

    // "Reaching N networks" summary should read 0 — derived from
    // `UseCrossPosting::reach_count` Memo (counts platforms toggled true
    // in `per_post_enabled`, which an unconnected user can't flip).
    await expect(
      page.locator(".reach-summary__value strong")
    ).toHaveText("0");
  });

  // AC-10 (Bluesky truncation warning when body > 300 chars) requires the
  // user to have a Connected Bluesky `SocialConnection` so the card renders
  // the `pp-foot` footer. Playwright can't seed that today — wiring it up
  // is tracked alongside `BYPASS_PLATFORM_API=mock`. Char-count math itself
  // is exercised in the unit-level `compose_sidebar` render path; this
  // skipped entry is the placeholder so future PRs can drop the `.skip()`.
  test.skip("Body > 300 chars triggers Bluesky truncation warning (AC-10)", () => {});
});
