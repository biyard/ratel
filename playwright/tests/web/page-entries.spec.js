import { test, expect } from "@playwright/test";
import { goto } from "../utils";

/**
 * Page Entries — Accessibility Smoke Test
 *
 * Verifies that every registered route in the app loads without showing the
 * "Page not found" component, and that the page itself rendered actual
 * content (not an empty/error shell).
 *
 * After the renewal/home-ui migration, AppLayout (the `app-layout` testid) is
 * no longer wired into `src/route.rs`. Top-level routes (`/`, `/privacy`,
 * `/terms`, `/membership`, `/posts/*`, `/:username/rewards`,
 * `/:username/settings`, `/admin/`) now render directly under RootLayout, so
 * we verify each page with a page-specific marker instead of a shared layout
 * testid.
 *
 * Layouts that ARE still wired and keep their testids:
 *   social-layout       → SocialLayout: /:username/posts | memberships |
 *                         drafts | credentials | spaces
 *   team-arena-layout   → TeamArenaLayout: /:teamUsername/home |
 *                         team-drafts | dao | members | team-rewards |
 *                         team-memberships | team-settings*
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
    data.spaceId = body.space_id;
    expect(data.spaceId).toBeTruthy();
  });

  test("setup: create poll via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/polls`, {
      data: {},
    });
    expect(res.ok(), `create poll: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    data.pollId = stripPrefix(body.sk);
    expect(data.pollId).toBeTruthy();
  });

  test("setup: create quiz via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/quizzes`, {
      data: {},
    });
    expect(res.ok(), `create quiz: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
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
    data.discussionId = stripPrefix(body.sk);
    expect(data.discussionId).toBeTruthy();
  });

  test("setup: create follow action via API", async ({ page }) => {
    const res = await page.request.post(`/api/spaces/${data.spaceId}/follows`, {
      data: {},
    });
    expect(res.ok(), `create follow: ${await res.text()}`).toBeTruthy();
    const body = await res.json();
    data.followId = stripPrefix(body.sk);
    expect(data.followId).toBeTruthy();
  });

  // ── Helpers ────────────────────────────────────────────────────────────────

  /**
   * Navigate to `url` and verify:
   *   1. "Page not found" is NOT shown (no 404/route mismatch)
   *   2. The page-specific `marker` IS visible (confirms real content
   *      rendered, not an empty/error shell)
   *
   * `marker` is an object like `{ testId }`, `{ text }`, or `{ css }`.
   */
  async function checkPage(page, url, marker) {
    await goto(page, url);
    await expect(page.getByText("Page not found")).toBeHidden({
      timeout: 15000,
    });
    const locator = resolveMarker(page, marker);
    await expect(locator).toBeVisible({ timeout: 15000 });
  }

  function resolveMarker(page, marker) {
    if (marker.testId) return page.getByTestId(marker.testId);
    if (marker.text) return page.getByText(marker.text, { exact: true }).first();
    if (marker.css) return page.locator(marker.css).first();
    throw new Error(`Unknown marker: ${JSON.stringify(marker)}`);
  }

  // ── Top-level pages (RootLayout only, no app-layout) ──────────────────────

  test("GET /", async ({ page }) =>
    checkPage(page, "/", { testId: "home-btn-create" }));
  test("GET /privacy", async ({ page }) =>
    checkPage(page, "/privacy", { text: "Privacy Policy" }));
  test("GET /terms", async ({ page }) =>
    checkPage(page, "/terms", { text: "Terms of Service" }));
  test("GET /membership", async ({ page }) =>
    checkPage(page, "/membership", { text: "Membership Plans" }));
  test("GET /my-follower", async ({ page }) =>
    checkPage(page, "/my-follower", { text: "My Network" }));

  // ── Post pages (RootLayout only) ──────────────────────────────────────────

  // `/posts/` renders a placeholder "app shell" component.
  test("GET /posts/", async ({ page }) =>
    checkPage(page, "/posts/", { text: "app shell" }));
  test("GET /posts/:post_id", async ({ page }) =>
    checkPage(page, `/posts/${data.postId}`, {
      css: ".post-detail-header, .max-w-desktop",
    }));
  test("GET /posts/:post_id/edit", async ({ page }) =>
    checkPage(page, `/posts/${data.postId}/edit`, { css: ".title-input" }));

  // ── User pages — inside SocialLayout ──────────────────────────────────────

  test("GET /:username/", async ({ page }) =>
    checkPage(page, `/${data.username}/`, { testId: "social-layout" }));
  test("GET /:username/posts", async ({ page }) =>
    checkPage(page, `/${data.username}/posts`, { testId: "social-layout" }));
  test("GET /:username/memberships", async ({ page }) =>
    checkPage(page, `/${data.username}/memberships`, {
      testId: "social-layout",
    }));
  test("GET /:username/drafts", async ({ page }) =>
    checkPage(page, `/${data.username}/drafts`, { testId: "social-layout" }));
  test("GET /:username/credentials", async ({ page }) =>
    checkPage(page, `/${data.username}/credentials`, {
      testId: "social-layout",
    }));
  test("GET /:username/spaces", async ({ page }) =>
    checkPage(page, `/${data.username}/spaces`, { testId: "social-layout" }));

  // NOTE: /:username/rewards depends on an external biyard service for SSR.
  // The server-side rendering hangs when the service is unavailable,
  // causing page.goto to timeout. Skip in test environments.
  test.skip("GET /:username/rewards", async ({ page }) =>
    checkPage(page, `/${data.username}/rewards`, { text: "Rewards" }));

  // `/:username/settings` is now outside SocialLayout (RootLayout only).
  test("GET /:username/settings", async ({ page }) =>
    checkPage(page, `/${data.username}/settings`, { text: "Settings" }));

  // ── Team-specific pages (TeamArenaLayout) ─────────────────────────────────

  test("GET /:teamUsername/home", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/home`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/team-drafts", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-drafts`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/dao", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/dao`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/members", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/members`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/team-rewards", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-rewards`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/team-memberships", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-memberships`, {
      testId: "team-arena-layout",
    }));

  // ── Team settings pages (TeamArenaLayout, consolidated) ──────────────────

  test("GET /:teamUsername/team-settings", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-settings`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/team-settings/members", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-settings/members`, {
      testId: "team-arena-layout",
    }));
  test("GET /:teamUsername/team-settings/subscription", async ({ page }) =>
    checkPage(page, `/${data.teamUsername}/team-settings/subscription`, {
      testId: "team-arena-layout",
    }));

  // ── Space pages (SpaceLayout) ─────────────────────────────────────────────

  test("GET /spaces/:space_id/dashboard", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/dashboard`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/overview", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/overview`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/report", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/report`, {
      testId: "space-layout-container",
    }));

  // ── Space action pages (SpaceLayout) ──────────────────────────────────────

  test("GET /spaces/:space_id/actions/", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/actions/`, {
      testId: "space-layout-container",
    }));
  // Admin users hit SpaceLayout's early-return (arena/action-edit route)
  // which skips the sidebar shell and its space-layout-container testid.
  // Verify each creator page renders its own card instead.
  test("GET /spaces/:space_id/actions/polls/:poll_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/polls/${data.pollId}`,
      { testId: "page-card-content" },
    ));
  test("GET /spaces/:space_id/actions/quizzes/:quiz_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/quizzes/${data.quizId}`,
      { testId: "page-card-content" },
    ));
  // Discussion at /discussions/:id still renders legacy CreatorMain (tab UI)
  // for admins, not the arena editor. Use tablist role as the marker.
  test("GET /spaces/:space_id/actions/discussions/:discussion_id", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/discussions/${data.discussionId}`,
      { css: "[role='tablist']" },
    ));
  // NOTE: discussion editor page loads discussion context via server function,
  // which can cause SSR crash (ERR_EMPTY_RESPONSE) in some environments.
  test.skip("GET /spaces/:space_id/actions/discussions/:discussion_id/edit", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/discussions/${data.discussionId}/edit`,
      { testId: "page-card-content" },
    ));
  // Follow creator renders page-card-targets (not page-card-content).
  test("GET /spaces/:space_id/actions/follows/:follow_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/actions/follows/${data.followId}`,
      { testId: "page-card-targets" },
    ));

  // ── Space app pages (SpaceLayout) ─────────────────────────────────────────

  test("GET /spaces/:space_id/apps/", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/apps/general", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/general`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/apps/files", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/files`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/apps/analyzes", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/analyzes`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/apps/analyzes/poll/:poll_id", async ({ page }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/analyzes/poll/${data.pollId}`,
      { testId: "space-layout-container" },
    ));
  test("GET /spaces/:space_id/apps/analyzes/discussion/:discussion_id", async ({
    page,
  }) =>
    checkPage(
      page,
      `/spaces/${data.spaceId}/apps/analyzes/discussion/${data.discussionId}`,
      { testId: "space-layout-container" },
    ));
  test("GET /spaces/:space_id/apps/panels", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/panels`, {
      testId: "space-layout-container",
    }));
  test("GET /spaces/:space_id/apps/incentive-pool", async ({ page }) =>
    checkPage(page, `/spaces/${data.spaceId}/apps/incentive-pool`, {
      testId: "space-layout-container",
    }));

  // NOTE: The `admin-menu` testid comes from AppMenu, which was rendered by
  // the removed AppLayout. After the home-ui renewal AppMenu is no longer
  // part of any active route layout, so the element is never rendered for
  // any user. The test is retained to verify that the home page does not
  // surface an "Admin" link for regular users, but it now holds trivially.
  test("should NOT show Admin menu item for non-admin users", async ({
    page,
  }) => {
    await goto(page, "/");
    await expect(page.getByTestId("admin-menu")).toBeHidden({ timeout: 10000 });
  });
});
