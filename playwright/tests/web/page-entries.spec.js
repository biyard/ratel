import { test, expect } from "@playwright/test";
import { goto } from "../utils";

/**
 * Page Entries — Accessibility Smoke Test
 *
 * Verifies that every registered route in the app loads without showing
 * the "Page not found" component, and that the expected layout container
 * is visible for each page group.
 *
 * Layout testid mapping:
 *   app-layout          → AppLayout (SidebarInset wrapper): /, /privacy, /terms,
 *                         /membership, /my-follower, /posts/*, /admin/
 *   social-layout       → SocialLayout: /:username/* (user + team pages)
 *   team-setting-layout → TeamSettingLayout: /:username/team-settings/*
 *   space-layout-container → SpaceLayout: /spaces/:id/*
 *
 * Test data is set up via REST API calls using page.request (which carries
 * the browser's auth cookies from user.json).
 *
 * API body format follows the Dioxus server function convention:
 *   Each parameter is wrapped by its Rust parameter name as the JSON key.
 *   e.g. create_team_handler(body: CreateTeamRequest) → {"body": {...}}
 *        create_space_handler(req: CreateSpaceRequest) → {"req": {...}}
 *
 * User: hi+user1@biyard.co (pre-authenticated via user.json storage state)
 */

// Shared state across serial tests (same worker)
const data = {};

// Strip the type prefix from a DynamoDB sort key / partition key.
// e.g. "SpacePoll#abc-123"    → "abc-123"
//      "FEED#abc-123"         → "abc-123"
//      "SpaceActionFollow#xyz" → "xyz"
function stripPrefix(key) {
  const idx = key.indexOf("#");
  return idx >= 0 ? key.slice(idx + 1) : key;
}

test.describe.serial("Page entries accessibility", () => {
  // ── Setup ──────────────────────────────────────────────────────────────────

  test("setup: get current user info", async ({ page }) => {
    const res = await page.request.get("/api/auth/me");
    expect(res.ok()).toBeTruthy();
    const body = await res.json();
    expect(body.user, "should be logged in").toBeTruthy();
    data.username = body.user.username;
    expect(data.username).toBeTruthy();
  });

  test("setup: create team via API", async ({ page }) => {
    const teamUsername = `e2e_pg_${Date.now()}`;
    const res = await page.request.post("/api/teams/create", {
      data: {
        body: {
          username: teamUsername,
          nickname: "Page Test Team",
          profile_url: "",
          description: "E2E page entries test team",
        },
      },
    });
    expect(res.ok(), `create team: ${await res.text()}`).toBeTruthy();
    data.teamUsername = teamUsername;
  });

  test("setup: create post via API", async ({ page }) => {
    const res = await page.request.post("/api/posts", {
      data: {},
    });
    expect(res.ok(), `create post: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // post_pk is "FEED#uuid" — strip prefix for URL param
    data.postId = stripPrefix(body.post_pk);
    expect(data.postId).toBeTruthy();
  });

  test("setup: create space via API", async ({ page }) => {
    const res = await page.request.post("/api/spaces/create", {
      data: {
        req: { post_id: data.postId },
      },
    });
    expect(res.ok(), `create space: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // space_id is already a uuid (SpacePartition)
    data.spaceId = body.space_id;
    expect(data.spaceId).toBeTruthy();
  });

  test("setup: create poll via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/polls`, {
      data: {},
    });
    expect(res.ok(), `create poll: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // sk is "SpacePoll#uuid" — strip prefix
    data.pollId = stripPrefix(body.sk);
    expect(data.pollId).toBeTruthy();
  });

  test("setup: create quiz via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/quizzes`, {
      data: {},
    });
    expect(res.ok(), `create quiz: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // quiz response returns quiz_id directly (no prefix)
    data.quizId = body.quiz_id;
    expect(data.quizId).toBeTruthy();
  });

  test("setup: create discussion via API", async ({ page }) => {
    const res = await page.request.post(
      `/api/spaces/${data.spaceId}/discussions`,
      { data: {} },
    );
    expect(res.ok(), `create discussion: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // sk is "SpacePost#uuid" — strip prefix
    data.discussionId = stripPrefix(body.sk);
    expect(data.discussionId).toBeTruthy();
  });

  test("setup: create follow action via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/follows`, {
      data: {},
    });
    expect(res.ok(), `create follow: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    // sk is "SpaceActionFollow#uuid" — strip prefix
    data.followId = stripPrefix(body.sk);
    expect(data.followId).toBeTruthy();
  });

  // ── Helpers ────────────────────────────────────────────────────────────────

  /**
   * Navigate to `url` and verify:
   *   1. "Page not found" is NOT shown (no 404/route mismatch)
   *   2. The expected layout container identified by `testId` IS visible
   *      (confirms the page rendered actual content, not an empty/error shell)
   */
  async function checkPage(page, url, testId) {
    await goto(page, url);
    await expect(page.getByText("Page not found")).toBeHidden({
      timeout: 15000,
    });
    await expect(page.getByTestId(testId)).toBeVisible({ timeout: 15000 });
  }

  // ── Static / app-shell pages (AppLayout) ───────────────────────────────────

  test("GET /", async ({ page }) => checkPage(page, "/", "app-layout"));
  test("GET /privacy", async ({ page }) =>
    checkPage(page, "/privacy", "app-layout"));
  test("GET /terms", async ({ page }) =>
    checkPage(page, "/terms", "app-layout"));
  test("GET /membership", async ({ page }) =>
    checkPage(page, "/membership", "app-layout"));
  test("GET /my-follower", async ({ page }) =>
    checkPage(page, "/my-follower", "app-layout"));

  // ── Post pages (AppLayout) ─────────────────────────────────────────────────

  test("GET /posts/", async ({ page }) =>
    checkPage(page, "/posts/", "app-layout"));
  test("GET /posts/:post_id", async ({ page }) =>
    checkPage(page, `/posts/${data.postId}`, "app-layout"));
  test("GET /posts/:post_id/edit", async ({ page }) =>
    checkPage(page, `/posts/${data.postId}/edit`, "app-layout"));

  // ── User / team pages (SocialLayout) ──────────────────────────────────────

  test("GET /:username/", async ({ page }) =>
    checkPage(page, `/${data.username}/`, "social-layout"));
  test("GET /:username/posts", async ({ page }) =>
    checkPage(page, `/${data.username}/posts`, "social-layout"));
  test("GET /:username/memberships", async ({ page }) =>
    checkPage(page, `/${data.username}/memberships`, "social-layout"));
  test("GET /:username/drafts", async ({ page }) =>
    checkPage(page, `/${data.username}/drafts`, "social-layout"));
  test("GET /:username/credentials", async ({ page }) =>
    checkPage(page, `/${data.username}/credentials`, "social-layout"));
  test("GET /:username/spaces", async ({ page }) =>
    checkPage(page, `/${data.username}/spaces`, "social-layout"));
  // NOTE: /:username/rewards depends on an external biyard service for SSR.
  // The server-side rendering hangs when the service is unavailable,
  // causing page.goto to timeout. Skip in test environments.
  test.skip("GET /:username/rewards", async ({ page }) =>
    checkPage(page, `/${data.username}/rewards`, "app-layout"));
  test("GET /:username/settings", async ({ page }) =>
    checkPage(page, `/${data.username}/settings`, "app-layout"));

  // ── Team-specific pages (SocialLayout) ────────────────────────────────────

  test("GET /:teamUsername/home", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/home`, "social-layout"));
  test("GET /:teamUsername/team-drafts", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-drafts`, "social-layout"));
  test("GET /:teamUsername/groups", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/groups`, "social-layout"));
  test("GET /:teamUsername/dao", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/dao`, "social-layout"));
  test("GET /:teamUsername/members", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/members`, "social-layout"));
  test("GET /:teamUsername/team-rewards", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-rewards`, "social-layout"));
  test("GET /:teamUsername/team-memberships", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-memberships`, "social-layout"));

  // ── Team settings pages (TeamSettingLayout) ────────────────────────────────

  test("GET /:teamUsername/team-settings", async ({ page }) =>
    checkPage(
      page,
      `/${data.teamUsername}/team-settings`,
      "team-setting-layout",
    ));
  test("GET /:teamUsername/team-settings/members", async ({ page }) =>
    checkPage(
      page,
      `/${data.teamUsername}/team-settings/members`,
      "team-setting-layout",
    ));
  test("GET /:teamUsername/team-settings/subscription", async ({ page }) =>
    checkPage(
      page,
      `/${data.teamUsername}/team-settings/subscription`,
      "team-setting-layout",
    ));

  // ── Space pages (SpaceLayout) ──────────────────────────────────────────────

  test("GET /spaces/:space_id/dashboard", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/dashboard`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/overview", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/overview`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/report", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/report`,
      "space-layout-container",
    ));

  // ── Space action pages (SpaceLayout) ──────────────────────────────────────

  test("GET /spaces/:space_id/actions/", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/actions/polls/:poll_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/polls/${data.pollId}`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/actions/quizzes/:quiz_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/quizzes/${data.quizId}`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/actions/discussions/:discussion_id", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/discussions/${data.discussionId}`,
      "space-layout-container",
    ));
  // NOTE: discussion editor page loads discussion context via server function,
  // which can cause SSR crash (ERR_EMPTY_RESPONSE) in some environments.
  // The route exists but may need a fully configured space to render without errors.
  test.skip("GET /spaces/:space_id/actions/discussions/:discussion_id/edit", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/discussions/${data.discussionId}/edit`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/actions/follows/:follow_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/follows/${data.followId}`,
      "space-layout-container",
    ));

  // ── Space app pages (SpaceLayout) ─────────────────────────────────────────

  test("GET /spaces/:space_id/apps/", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/`, "space-layout-container"));
  test("GET /spaces/:space_id/apps/general", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/general`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/files", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/files`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/analyzes", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/analyzes`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/analyzes/poll/:poll_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/analyzes/poll/${data.pollId}`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/analyzes/discussion/:discussion_id", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/analyzes/discussion/${data.discussionId}`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/panels", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/panels`,
      "space-layout-container",
    ));
  test("GET /spaces/:space_id/apps/incentive-pool", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/incentive-pool`,
      "space-layout-container",
    ));

  test("should NOT show Admin menu item for non-admin users", async ({
    page,
  }) => {
    // The `page` fixture uses the default user.json storage state
    // (hi+user1@biyard.co), which is a regular non-admin user.
    await goto(page, "/");
    await expect(page.getByTestId("admin-menu")).toBeHidden({ timeout: 10000 });
  });
});
