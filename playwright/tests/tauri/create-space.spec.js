// Tauri Android smoke test — drives the WebView inside the installed APK
// directly via Chrome DevTools Protocol (CDP).
//
// Why CDP via `chrome-remote-interface` and not Playwright's
// `chromium.connectOverCDP`: Android System WebView's CDP implementation
// doesn't expose browser-level methods like `Browser.setDownloadBehavior`,
// which Playwright calls during connection setup. The connection fails
// with `Protocol error (Browser.setDownloadBehavior): Browser context
// management is not supported.` Using a raw CDP client (which only talks
// the page-level CDP we actually need) sidesteps that.
//
// Scope: signup against the live dev backend, create a team, then create
// a post + space via REST issued from inside the WebView. Verifies the
// space round-trips back through GET. Exercises every Tauri-specific
// plumbing layer: custom `use_loader`, reqwest server_fn shim,
// cross-origin session cookie, dev backend CORS acceptance of
// tauri.localhost.

import { test, expect } from "@playwright/test";
import CDP from "chrome-remote-interface";

const CDP_HOST = "localhost";
const CDP_PORT = Number(process.env.TAURI_CDP_PORT || 9223);
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

let client;

// ── CDP helpers ──────────────────────────────────────────────────────────

/** Evaluate JS in the page context and return the value. */
async function evalJs(expression) {
  const { result, exceptionDetails } = await client.Runtime.evaluate({
    expression,
    returnByValue: true,
    awaitPromise: true,
  });
  if (exceptionDetails) {
    throw new Error(
      `evaluate failed: ${exceptionDetails.text}${
        exceptionDetails.exception?.description
          ? "\n" + exceptionDetails.exception.description
          : ""
      }`,
    );
  }
  return result.value;
}

/** Wait until a predicate evaluates to truthy in the page (poll every 250ms). */
async function waitFor(jsExpression, { timeout = 30_000, label } = {}) {
  const deadline = Date.now() + timeout;
  let lastErr;
  while (Date.now() < deadline) {
    try {
      if (await evalJs(jsExpression)) return;
    } catch (e) {
      lastErr = e;
    }
    await new Promise((r) => setTimeout(r, 250));
  }
  throw new Error(
    `timed out after ${timeout}ms waiting for: ${label || jsExpression}${
      lastErr ? "\nlast eval error: " + lastErr.message : ""
    }`,
  );
}

/** Click an element matching `selector` (XPath or CSS). */
async function clickSelector(selector) {
  const escaped = selector.replace(/\\/g, "\\\\").replace(/`/g, "\\`");
  const success = await evalJs(`
    (() => {
      const el = document.querySelector(\`${escaped}\`);
      if (!el) return false;
      el.click();
      return true;
    })()
  `);
  if (!success) throw new Error(`selector not found for click: ${selector}`);
}

/** Set the value of an input element matching selector and dispatch input/change. */
async function fillSelector(selector, value) {
  const escaped = selector.replace(/\\/g, "\\\\").replace(/`/g, "\\`");
  const escapedValue = JSON.stringify(value);
  const success = await evalJs(`
    (() => {
      const el = document.querySelector(\`${escaped}\`);
      if (!el) return false;
      const nativeSetter = Object.getOwnPropertyDescriptor(
        el.tagName === 'TEXTAREA' ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype,
        'value'
      ).set;
      nativeSetter.call(el, ${escapedValue});
      el.dispatchEvent(new Event('input', { bubbles: true }));
      el.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    })()
  `);
  if (!success) throw new Error(`selector not found for fill: ${selector}`);
}

/** Click an element by its visible text (button/link/etc.). */
async function clickByText(text, opts = {}) {
  const exact = opts.exact ?? false;
  const escaped = JSON.stringify(text);
  const success = await evalJs(`
    (() => {
      const target = ${escaped};
      const els = [...document.querySelectorAll('button, a, [role="button"]')];
      const match = els.find((el) => {
        const t = (el.textContent || '').trim();
        return ${exact} ? t === target : t.includes(target);
      });
      if (!match) return false;
      match.click();
      return true;
    })()
  `);
  if (!success) throw new Error(`element with text "${text}" not found`);
}

/** Wait for an element matching selector to exist. */
function waitForSelector(selector, opts = {}) {
  const escaped = selector.replace(/\\/g, "\\\\").replace(/`/g, "\\`");
  return waitFor(
    `!!document.querySelector(\`${escaped}\`)`,
    { ...opts, label: `selector "${selector}"` },
  );
}

// ── Test ─────────────────────────────────────────────────────────────────

test.beforeAll(async () => {
  // Connect to the first page target the WebView exposes. CI sets up
  // `adb forward tcp:9223 localabstract:webview_devtools_remote_<pid>`
  // before this spec runs, so the WebView's CDP is reachable via
  // `localhost:9223`. `target` filter is needed because the action lists
  // both the page and any service workers — we want the page.
  client = await CDP({
    host: CDP_HOST,
    port: CDP_PORT,
    target: (targets) => {
      const page = targets.find((t) => t.type === "page");
      if (!page) throw new Error("no page target in WebView devtools");
      return page;
    },
  });
  await client.Runtime.enable();
  await client.Page.enable();

  // The APK boots into the home screen and renders the unauthenticated
  // HUD. Waiting for `home-btn-signin` proves both that the wasm bundle
  // initialized and that `Context::init` resolved past the suspense.
  await waitForSelector('[data-testid="home-btn-signin"]', {
    timeout: 30_000,
  });
});

test.afterAll(async () => {
  await client?.close();
});

test("tauri smoke: signup → team → post → space", async () => {
  // ── 1. Sign up new user ───────────────────────────────────────────────
  await clickSelector('[data-testid="home-btn-signin"]');
  await waitFor(
    `document.body.textContent.includes('Create an account')`,
    { label: "Create an account link visible" },
  );
  await clickByText("Create an account");

  await waitForSelector('input[placeholder="Enter your email address"]');
  await fillSelector(
    'input[placeholder="Enter your email address"]',
    user.email,
  );
  await clickByText("Send", { exact: true });
  await waitForSelector('input[placeholder="Enter the verification code"]');
  await fillSelector(
    'input[placeholder="Enter the verification code"]',
    "000000",
  );
  await clickByText("Verify", { exact: true });
  // Send button disappears once server confirms code.
  await waitFor(
    `![...document.querySelectorAll('button')].some(b => b.textContent.trim() === 'Send')`,
    { timeout: 10_000, label: "Send button hidden after verify" },
  );

  await fillSelector('input[placeholder="Enter your password"]', user.password);
  await fillSelector(
    'input[placeholder="Re-enter your password"]',
    user.password,
  );
  await fillSelector(
    'input[placeholder="Enter your display name"]',
    user.nickname,
  );
  await fillSelector(
    'input[placeholder="Enter your user name"]',
    user.username,
  );
  // ToS checkbox — its label text matches the web tests.
  await clickByText("I have read and accept the Terms of Service");
  await clickByText("Finished Sign-up");

  // Wait for the signed-in HUD (sign-in button gone, teams button visible).
  await waitFor(
    `!document.querySelector('[data-testid="home-btn-signin"]')`,
    { timeout: 30_000, label: "signin button removed after signup" },
  );
  await waitForSelector('[data-testid="home-btn-teams"]', { timeout: 30_000 });

  // ── 2. Create team via UI ─────────────────────────────────────────────
  await clickSelector('[data-testid="home-btn-teams"]');
  await waitForSelector('[data-testid="home-btn-create-team"]');
  await clickSelector('[data-testid="home-btn-create-team"]');
  await waitForSelector('[data-testid="arena-create-team-popup"]');
  await fillSelector('[data-testid="team-nickname-input"]', team.nickname);
  await fillSelector('[data-testid="team-username-input"]', team.username);
  await clickSelector('[data-testid="team-create-submit"]');

  // Successful team creation triggers SPA navigation to /{username}.
  await waitFor(
    `window.location.pathname.replace(/\\/$/, '') === '/${team.username}'`,
    { timeout: 15_000, label: `navigated to /${team.username}` },
  );

  // ── 3. Create post + space via REST (from inside the WebView) ────────
  // `fetch` runs in the WebView's origin context (tauri.localhost), so
  // the session cookie that was set on dev.ratel.foundation during
  // signup rides along automatically via credentials: 'include'.
  const apiBase = JSON.stringify(API_BASE);

  const postId = await evalJs(`
    (async () => {
      const r = await fetch(${apiBase} + '/api/posts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: '{}',
      });
      if (!r.ok) throw new Error('POST /api/posts -> ' + r.status);
      const data = await r.json();
      const pk = data.post_pk;
      return pk.includes('#') ? pk.split('#')[1] : pk;
    })()
  `);
  expect(postId, "post creation must return post_pk").toBeTruthy();

  const spaceId = await evalJs(`
    (async () => {
      const r = await fetch(${apiBase} + '/api/spaces/create', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ req: { post_id: ${JSON.stringify(postId)} } }),
      });
      if (!r.ok) throw new Error('POST /api/spaces/create -> ' + r.status);
      const data = await r.json();
      return data.space_id;
    })()
  `);
  expect(spaceId, "space creation must return space_id").toBeTruthy();

  // ── 4. Verify space is queryable from the WebView ─────────────────────
  const spaceTitle = await evalJs(`
    (async () => {
      const r = await fetch(${apiBase} + '/api/spaces/' + ${JSON.stringify(spaceId)}, {
        method: 'GET',
        credentials: 'include',
      });
      if (!r.ok) throw new Error('GET /api/spaces/' + ${JSON.stringify(spaceId)} + ' -> ' + r.status);
      const data = await r.json();
      return data.title ?? null;
    })()
  `);
  expect(spaceTitle).not.toBeNull();
});
