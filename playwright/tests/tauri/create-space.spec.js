// Tauri Android smoke test — drives the WebView inside the installed APK
// directly via Chrome DevTools Protocol (CDP).
//
// Why a hand-rolled CDP client (`cdp-client.js`) and not Playwright's
// `chromium.connectOverCDP` or `chrome-remote-interface`:
//
//   * Playwright's connectOverCDP calls `Browser.setDownloadBehavior`
//     during connection setup. Android WebView's CDP doesn't implement
//     browser-level methods, so the connection fails with
//     `Browser context management is not supported.`
//   * `chrome-remote-interface` works against desktop Chrome but on the
//     `api-level: 34, target: default` emulator WebView (Chromium 113)
//     the WebSocket handshake silently aborts ("socket hang up") with
//     no further diagnostics — no way to see why the server dropped us.
//
// The raw `ws`-based client in `cdp-client.js` connects directly to the
// `webSocketDebuggerUrl` from `/json/list`, sets explicit `Origin` /
// `Host` headers, and logs the upgrade response on failure so we can
// debug rather than guess.
//
// Scope: signup against the live dev backend, create a team, then create
// a post + space via REST issued from inside the WebView. Verifies the
// space round-trips back through GET. Exercises every Tauri-specific
// plumbing layer: custom `use_loader`, reqwest server_fn shim,
// cross-origin session cookie, dev backend CORS acceptance of
// tauri.localhost.

import { test, expect } from "@playwright/test";
import { connectCdp } from "./cdp-client.js";

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

/**
 * Click an element by its visible text. Filters out hidden + disabled
 * candidates because the Tauri WebView (Chromium 113) swallows click
 * events on `disabled` buttons and on `display: none` elements — Dioxus
 * never sees the onclick. Polls so an in-flight async handler (which
 * temporarily flips `disabled`) doesn't break the click.
 */
async function clickByText(text, opts = {}) {
  const exact = opts.exact ?? false;
  const timeout = opts.timeout ?? 15_000;
  const target = JSON.stringify(text);
  const finder = `
    (() => {
      const t0 = ${target};
      const els = [...document.querySelectorAll('button, a, [role="button"]')];
      const match = els.find((el) => {
        const t = (el.textContent || '').trim();
        const textOk = ${exact} ? t === t0 : t.includes(t0);
        if (!textOk) return false;
        if (el.disabled) return false;
        if (el.offsetParent === null && getComputedStyle(el).position !== 'fixed') return false;
        return true;
      });
      if (!match) return false;
      match.click();
      return true;
    })()
  `;
  const deadline = Date.now() + timeout;
  let lastErr;
  while (Date.now() < deadline) {
    try {
      if (await evalJs(finder)) return;
    } catch (e) {
      lastErr = e;
    }
    await new Promise((r) => setTimeout(r, 250));
  }
  throw new Error(
    `clickByText timed out for "${text}"${lastErr ? `: ${lastErr.message}` : ""}`,
  );
}

/** Wait for an element matching selector to exist (not necessarily visible). */
function waitForSelector(selector, opts = {}) {
  const escaped = selector.replace(/\\/g, "\\\\").replace(/`/g, "\\`");
  return waitFor(
    `!!document.querySelector(\`${escaped}\`)`,
    { ...opts, label: `selector "${selector}"` },
  );
}

/**
 * Wait for an element matching selector to be visible (rendered, not
 * `display: none` / `visibility: hidden`, not a zero-size box). The
 * Dioxus signup modal keeps the verification-code row in the DOM at all
 * times but hides it via `aria-hidden:hidden` (a Tailwind variant that
 * applies `display: none`). `waitForSelector` returns instantly because
 * the input is present; this helper waits until it's actually shown.
 */
function waitForVisible(selector, opts = {}) {
  const escaped = selector.replace(/\\/g, "\\\\").replace(/`/g, "\\`");
  return waitFor(
    `(() => {
      const el = document.querySelector(\`${escaped}\`);
      if (!el) return false;
      if (el.offsetParent === null) return false;
      const rect = el.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0;
    })()`,
    { ...opts, label: `visible "${selector}"` },
  );
}

/**
 * Toggle a checkbox by the visible text of its containing `<label>`.
 * Why a dedicated helper: programmatic `.click()` on a `<label>` does
 * not activate the associated control (browsers gate that on
 * user-initiated clicks for security). Calling `.click()` directly on
 * the inner `<input type="checkbox">` does flip `.checked` and fires
 * the synthetic `change` event that Dioxus's `onchange` listens for.
 */
async function clickCheckboxByLabel(text, opts = {}) {
  const timeout = opts.timeout ?? 15_000;
  const target = JSON.stringify(text);
  const finder = `
    (() => {
      const t0 = ${target};
      const labels = [...document.querySelectorAll('label')];
      const lbl = labels.find((l) => (l.textContent || '').includes(t0));
      if (!lbl) return false;
      const cb = lbl.querySelector('input[type="checkbox"]');
      if (!cb || cb.disabled) return false;
      if (cb.offsetParent === null && lbl.offsetParent === null) return false;
      cb.click();
      return true;
    })()
  `;
  const deadline = Date.now() + timeout;
  let lastErr;
  while (Date.now() < deadline) {
    try {
      if (await evalJs(finder)) return;
    } catch (e) {
      lastErr = e;
    }
    await new Promise((r) => setTimeout(r, 250));
  }
  throw new Error(
    `clickCheckboxByLabel timed out for "${text}"${lastErr ? `: ${lastErr.message}` : ""}`,
  );
}

// ── Test ─────────────────────────────────────────────────────────────────

test.beforeAll(async () => {
  // CI sets up `adb forward tcp:9223 localabstract:webview_devtools_remote_<pid>`
  // before this spec runs, so the WebView's CDP is reachable via
  // `localhost:9223`. `connectCdp` will fetch `/json/list`, pick the
  // first page target, and connect to its `webSocketDebuggerUrl`
  // directly via raw `ws`. Logs the upgrade response on failure.
  client = await connectCdp({ host: CDP_HOST, port: CDP_PORT });
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
  // The verification-code input/Verify-button row is always in the DOM
  // but hidden via aria-hidden until `sent_code = true` (after the
  // send-code-handler request resolves). Waiting for visibility avoids
  // the race where the spec fills + clicks Verify while the row is
  // still hidden (Verify is also `disabled: loading()` during that
  // window — disabled buttons swallow click events in Chromium).
  await waitForVisible(
    'input[placeholder="Enter the verification code"]',
    { timeout: 30_000 },
  );
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
  // ToS checkbox — its label text matches the web tests. The label
  // wraps the actual `<input type="checkbox">` so programmatic
  // `.click()` on the label is a no-op (browsers gate label-activation
  // on user-initiated clicks). Use the dedicated helper that drills
  // into the inner input.
  await clickCheckboxByLabel("I have read and accept the Terms of Service");
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
