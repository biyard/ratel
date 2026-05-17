// Tauri Android smoke test — drives the WebView inside the installed APK via
// Chrome DevTools Protocol. The APK exposes its devtools socket at
// `@webview_devtools_remote_<pid>`; CI forwards that to tcp:9223 via
// `adb forward` before this spec runs, and we connect with `connectOverCDP`.
//
// Scope: signup against the live dev backend, create a team, then create a
// post + space via REST (issued from inside the WebView via `fetch` so the
// session cookie rides along). Verifies the response shape — does NOT try
// to `page.goto` the space URL because the WebView is locked to
// `tauri.localhost` and cross-origin navigation would fail. Confirming the
// space-creation API returns a valid `space_id` is enough to prove every
// Tauri-specific plumbing layer (custom `use_loader`, reqwest server_fn
// shim, cross-origin session cookie) is working end-to-end.

import { test, expect, chromium } from "@playwright/test";
import { click, fill, waitPopup } from "../utils";

const CDP_URL = process.env.TAURI_CDP_URL || "http://localhost:9223";
const API_BASE =
  process.env.TAURI_API_BASE || "https://dev.ratel.foundation";

const RUN_ID = Date.now();
const user = {
  email: `tauri-smoke-${RUN_ID}@biyard.co`,
  username: `tauri_smoke_${RUN_ID}`,
  nickname: `Tauri Smoke ${RUN_ID}`,
  password: "admin!234",
};
const team = {
  username: `tauri_team_${RUN_ID}`,
  nickname: `Tauri Team ${RUN_ID}`,
};

let browser;
let context;
let page;

test.beforeAll(async () => {
  // Connect to the already-running WebView inside the Tauri APK. CI
  // launches the APK and sets up
  //   adb forward tcp:9223 localabstract:webview_devtools_remote_<pid>
  // before invoking playwright, so this URL points at the live WebView.
  browser = await chromium.connectOverCDP(CDP_URL);
  context = browser.contexts()[0];
  page = context.pages()[0];

  // The APK boots into the home screen and renders the unauthenticated
  // HUD. Waiting for `home-btn-signin` proves both that the wasm bundle
  // initialized and that `Context::init` resolved past the suspense.
  // 30s covers cold wasm init on a CI emulator.
  await expect(page.getByTestId("home-btn-signin")).toBeVisible({
    timeout: 30_000,
  });
});

test.afterAll(async () => {
  await browser?.close();
});

test("tauri smoke: signup → team → post → space", async () => {
  // ── 1. Sign up new user ───────────────────────────────────────────────
  // Fresh email per run so consecutive PRs don't fight over the same dev
  // DB row. Verification code "000000" requires `bypass` feature enabled
  // on the dev backend.
  await click(page, { testId: "home-btn-signin" });
  await waitPopup(page, { visible: true });
  await click(page, { text: "Create an account" });

  await fill(page, { placeholder: "Enter your email address" }, user.email);
  await click(page, { text: "Send" });
  await fill(page, { placeholder: "Enter the verification code" }, "000000");
  await click(page, { text: "Verify" });
  await expect(page.getByText("Send", { exact: true })).toBeHidden({
    timeout: 10_000,
  });

  await fill(page, { placeholder: "Enter your password" }, user.password);
  await fill(page, { placeholder: "Re-enter your password" }, user.password);
  await fill(page, { placeholder: "Enter your display name" }, user.nickname);
  await fill(page, { placeholder: "Enter your user name" }, user.username);
  await click(page, {
    label: "[Required] I have read and accept the Terms of Service.",
  });
  await click(page, { text: "Finished Sign-up" });
  await waitPopup(page, { visible: false });

  // Signed-in HUD shows Teams button, signin button gone.
  await expect(page.getByTestId("home-btn-signin")).toBeHidden({
    timeout: 15_000,
  });
  await expect(page.getByTestId("home-btn-teams")).toBeVisible({
    timeout: 15_000,
  });

  // ── 2. Create team via UI ─────────────────────────────────────────────
  await click(page, { testId: "home-btn-teams" });
  await click(page, { testId: "home-btn-create-team" });
  await expect(page.getByTestId("arena-create-team-popup")).toBeVisible({
    timeout: 10_000,
  });
  await fill(page, { testId: "team-nickname-input" }, team.nickname);
  await fill(page, { testId: "team-username-input" }, team.username);
  await click(page, { testId: "team-create-submit" });
  // Successful team creation navigates the SPA router to `/{username}` —
  // dioxus updates `window.location` via pushState, so waitForURL sees the
  // change without an actual navigation request.
  await page.waitForURL(new RegExp(`/${team.username}/?$`), {
    waitUntil: "load",
  });

  // ── 3. Create post + space via REST (from inside the WebView) ────────
  // Use `page.evaluate(fetch)` instead of `page.request.post`:
  //   - `page.request` runs from playwright's APIRequestContext, which
  //     does NOT automatically inherit cookies set on the page's domain
  //     when those cookies were assigned via cross-origin `Set-Cookie`.
  //   - `page.evaluate(() => fetch(...))` runs inside the WebView, so the
  //     session cookie that was set on dev.ratel.foundation during signup
  //     rides along automatically (the same way the Rust server_fn shim
  //     emits requests via `fetch_credentials_include`).
  //
  // This single REST call exercises every Tauri-specific layer:
  //   - cross-origin session cookie from signup
  //   - dev backend CORS accepting tauri.localhost origin
  //   - server-side post-create + space-create handler chain
  const apiBase = API_BASE;

  const postId = await page.evaluate(async (base) => {
    const r = await fetch(`${base}/api/posts`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify({}),
    });
    if (!r.ok) throw new Error(`POST /api/posts -> ${r.status}`);
    const data = await r.json();
    const pk = data.post_pk;
    return pk.includes("#") ? pk.split("#")[1] : pk;
  }, apiBase);
  expect(postId).toBeTruthy();

  const spaceId = await page.evaluate(
    async ({ base, postId }) => {
      const r = await fetch(`${base}/api/spaces/create`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ req: { post_id: postId } }),
      });
      if (!r.ok) throw new Error(`POST /api/spaces/create -> ${r.status}`);
      const data = await r.json();
      return data.space_id;
    },
    { base: apiBase, postId },
  );
  expect(spaceId, "space creation must return a space_id").toBeTruthy();

  // ── 4. Verify space is queryable from the WebView ─────────────────────
  // GET the space we just created to prove the session can read it back
  // through the same Tauri plumbing. Final assertion proves the round-trip.
  const spaceTitle = await page.evaluate(
    async ({ base, spaceId }) => {
      const r = await fetch(`${base}/api/spaces/${spaceId}`, {
        method: "GET",
        credentials: "include",
      });
      if (!r.ok) throw new Error(`GET /api/spaces/${spaceId} -> ${r.status}`);
      const data = await r.json();
      return data.title ?? null;
    },
    { base: apiBase, spaceId },
  );
  expect(spaceTitle).not.toBeNull();
});
