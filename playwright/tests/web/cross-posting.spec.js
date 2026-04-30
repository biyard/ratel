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
    // The seeded test user (user1) may carry over an actual Bluesky
    // SocialConnection from prior local exploration of the cross-posting
    // flow, which would push the card into the connected-switch branch
    // instead of the connect-CTA branch. Reset Bluesky to disconnected
    // before asserting. The disconnect handler is idempotent — a second
    // DELETE on an already-Revoked row is a no-op success
    // (see disconnect.rs comment).
    await page.request.delete("/api/cross-posting/connections/bluesky");

    await goto(page, `/posts/${postId}/edit`);

    // Restore Public visibility so the sidebar mounts again.
    await click(page, { text: "Public" });
    await expect(page.locator(".crosspost").first()).toBeVisible();

    // With no Connected SocialConnection row, every card falls through
    // to the `connect-cta` branch in `compose_sidebar/component.rs`
    // (`if c.status != ConnectionStatus::Connected`).
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

  test("Already-seen onboarding interstitial bounces to home (AC-4b)", async ({
    page,
  }) => {
    // The seeded test user (set up in `user.auth.setup.js`) has logged in
    // before, so by the time this test runs they have already gone
    // through onboarding (or skipped it) at least once. AC-4b: hitting
    // `/onboarding/connections` directly must NOT pin them on the page —
    // the OnboardingPage's `use_effect` reads `User.interstitial_seen` and
    // navigator.replace's home.
    //
    // We check the post-redirect URL rather than asserting on home-page
    // content because the rest of `Index {}` rendering varies by feature
    // flags / data; the navigator.replace itself is the contract.
    await goto(page, "/onboarding/connections");
    await expect(page).toHaveURL(/\/$|\/\?/, { timeout: 10000 });
  });
});

test.describe.serial("Cross-posting Phase 1D — visibility notice (AC-20)", () => {
  let postId;

  test("Setup: create + publish a public post via REST", async ({ page }) => {
    const create = await page.request.post("/api/posts", { data: {} });
    expect(create.ok(), `create draft: ${await create.text()}`).toBeTruthy();
    const pk = (await create.json()).post_pk;
    postId = pk.includes("#") ? pk.split("#")[1] : pk;

    // Publish the draft so its server status flips to Published. The
    // editor's "already-published-public" guard reads `post.status` and
    // `post.visibility` from the loaded post — both must be set for the
    // notice to render when we toggle to Private below.
    //
    // Two server-function quirks hit this PUT:
    //   1. Handler signature `update_post_handler(post_id, req: UpdatePostRequest)`
    //      makes the Dioxus macro wrap the JSON body as
    //      `{ "req": <UpdatePostRequest> }`.
    //   2. `UpdatePostRequest` is `#[serde(untagged)]`, so we send the
    //      Publish variant's fields flat (NOT under a "Publish" key) — the
    //      first variant whose required fields match wins. With
    //      `title + content + publish` present, serde picks `Publish`.
    const publish = await page.request.put(`/api/posts/${postId}`, {
      data: {
        req: {
          publish: true,
          title: "Cross-posting AC-20 fixture",
          content: "<p>This is the AC-20 verification post body.</p>",
          visibility: "public",
          categories: [],
          image_urls: [],
        },
      },
    });
    expect(publish.ok(), `publish: ${await publish.text()}`).toBeTruthy();
  });

  test("Flipping public→private surfaces the syndicated-copies notice", async ({
    page,
  }) => {
    await goto(page, `/posts/${postId}/edit`);

    // The publish step above set visibility=Public, so the toggle starts
    // selected on Public. Click Private to trigger the notice.
    await click(page, { text: "Private" });

    // `.syndication-remain-notice` is rendered only when:
    //   * post.status == Published AND
    //   * initial visibility was Public AND
    //   * user has just toggled to Private
    await expect(page.locator(".syndication-remain-notice")).toBeVisible();
    await expect(
      page.locator(".syndication-remain-notice__title")
    ).toContainText(/Syndicated copies stay visible|이미 발행된 사본은 그대로 남습니다/);

    // Toggling back to Public should hide the notice.
    await click(page, { text: "Public" });
    await expect(page.locator(".syndication-remain-notice")).toBeHidden();
  });
});

test.describe.serial("Cross-posting Phase 1D — public backlink landing (AC-17, 18)", () => {
  let postId;

  test("Setup: publish a public post we can land on", async ({ page }) => {
    const create = await page.request.post("/api/posts", { data: {} });
    expect(create.ok(), `create draft: ${await create.text()}`).toBeTruthy();
    const pk = (await create.json()).post_pk;
    postId = pk.includes("#") ? pk.split("#")[1] : pk;

    // Same untagged-enum + req-wrapping shape as the AC-20 setup above.
    const publish = await page.request.put(`/api/posts/${postId}`, {
      data: {
        req: {
          publish: true,
          title: "AC-17 public landing fixture",
          content: "<p>Anonymous viewers should see this without sign-up.</p>",
          visibility: "public",
          categories: [],
          image_urls: [],
        },
      },
    });
    expect(publish.ok(), `publish: ${await publish.text()}`).toBeTruthy();
  });

  test("Anonymous viewer sees brand bar + subscribe CTA, no Edit topbar (AC-17)", async ({
    browser,
  }) => {
    // Fresh context with empty storage so the viewer is signed-out.
    const ctx = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await ctx.newPage();
    try {
      await goto(page, `/posts/${postId}`);

      // Brand bar with sign-in / get-started replaces the author topbar.
      await expect(page.getByTestId("post-brand-signin")).toBeVisible();
      await expect(page.getByTestId("post-brand-get-started")).toBeVisible();

      // Subscribe CTA at the article footer.
      await expect(page.getByTestId("post-subscribe-primary")).toBeVisible();
      await expect(page.getByTestId("post-subscribe-secondary")).toBeVisible();

      // Edit / Share author topbar must be hidden.
      await expect(page.locator(".arena-topbar")).toBeHidden();
    } finally {
      await ctx.close();
    }
  });

  test("Bluesky utm_source shows the tier-1 banner (AC-18)", async ({
    browser,
  }) => {
    const ctx = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await ctx.newPage();
    try {
      await goto(page, `/posts/${postId}?utm_source=bluesky`);

      const bar = page.getByTestId("post-refer-bar");
      await expect(bar).toHaveAttribute("data-show", "true", { timeout: 5000 });
      await expect(bar).toHaveAttribute("data-platform", "bluesky");
    } finally {
      await ctx.close();
    }
  });

  test("No utm_source / no referrer keeps the banner hidden", async ({
    browser,
  }) => {
    const ctx = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await ctx.newPage();
    try {
      await goto(page, `/posts/${postId}`);
      const bar = page.getByTestId("post-refer-bar");
      // The bar element exists in the DOM (rendered for signed-out viewers)
      // but JS classification leaves data-show="false" when there's no
      // referrer and no utm_source.
      await expect(bar).toHaveAttribute("data-show", "false");
    } finally {
      await ctx.close();
    }
  });
});
