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
// Backend validator restricts usernames to `[a-z0-9_-]{3,20}`. A full
// 13-digit Date.now() blows the 20-char cap, so we keep only the last
// 9 digits (still unique-enough within a test run) and prefix short.
const SHORT_ID = String(RUN_ID).slice(-9);
const user = {
  email: `tauri-smoke-${RUN_ID}@biyard.co`,
  username: `t_${SHORT_ID}`,
  nickname: `Tauri Smoke ${RUN_ID}`,
  password: "admin!234",
};
const team = {
  username: `tt_${SHORT_ID}`,
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
  await client.Network.enable();

  // Capture Set-Cookie headers + blocked-cookie reasons. The previous
  // run showed `document.cookie` and `Network.getCookies` empty after
  // signup, but didn't tell us *why* the cookie was rejected.
  // responseReceivedExtraInfo carries the response headers plus
  // `blockedCookies` (each with a `blockedReasons` enum from
  // SetCookieBlockedReason — SecureOnly, SameSiteNoneInsecure,
  // SamePartyConflictsWithOtherAttributes, etc.).
  client.on("Network.responseReceivedExtraInfo", (params) => {
    const url = params.headers?.[":path"] || params.headers?.["location"] || "";
    const sc = params.headers?.["set-cookie"];
    if (sc) console.log(`[smoke] Set-Cookie ${url}: ${sc}`);
    const blocked = params.blockedCookies;
    if (blocked && blocked.length) {
      console.log(`[smoke] blockedCookies: ${JSON.stringify(blocked)}`);
    }
  });

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
  // The Android emulator + slow software-rendered WebView add latency
  // at every step. Override the default 30s test timeout so the
  // multi-page signup → team → post → space flow can complete without
  // chasing transient slowness as if it were a real failure.
  test.setTimeout(120_000);
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

  // Diagnostic: dump cookie state after the signup POST has fired. If
  // the session cookie isn't getting stored in the WebView (the
  // suspected cause of the post-signup 401 chain on /api/inbox + team
  // listing), this will tell us before we time out waiting for the
  // signed-in HUD.
  await new Promise((r) => setTimeout(r, 3_000));
  const jsCookies = await evalJs(`document.cookie`);
  console.log(`[smoke] document.cookie after signup: ${JSON.stringify(jsCookies)}`);
  try {
    const cdpCookies = await client.Network.getCookies({ urls: [API_BASE] });
    console.log(
      `[smoke] CDP cookies for ${API_BASE}: ${JSON.stringify(cdpCookies)}`,
    );
    const allCookies = await client.Network.getCookies({});
    console.log(`[smoke] all cookies: ${JSON.stringify(allCookies)}`);
  } catch (e) {
    console.log(`[smoke] Network.getCookies failed: ${e.message}`);
  }

  // After successful signup the modal closes and the navigator replaces
  // the route with `OnboardingConnectionsPage` (the cross-posting
  // onboarding interstitial shown once per new account). Dismiss it to
  // land on the home page; the smoke test doesn't exercise the
  // onboarding flow itself.
  await waitForVisible('[data-testid="onboarding-skip-link"]', {
    timeout: 30_000,
  });
  await clickSelector('[data-testid="onboarding-skip-link"]');

  // Now wait for the signed-in HUD on the home page.
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

  // ── 3. Create post + space via REST (through the native transport) ───
  // The session cookie lives in the NATIVE reqwest cookie jar (the app
  // routes every API call through the `api_request` Tauri command), NOT
  // in the WebView — iOS WKWebView (ITP) and the cross-site origin pair
  // (tauri.localhost -> ratel.foundation) mean a plain in-WebView `fetch`
  // never carries the session and gets 401. So these calls must invoke
  // the same `api_request` command the app uses; it returns { status, body }.
  const apiBase = JSON.stringify(API_BASE);
  // Helper (defined in the page) that proxies a request through api_request
  // and throws on non-2xx, mirroring server_fn::send.
  const apiRequest = `async (method, path, body) => {
    const res = await window.__TAURI_INTERNALS__.invoke('api_request', {
      method, url: ${apiBase} + path, body,
    });
    if (res.status < 200 || res.status >= 300) {
      throw new Error(method + ' ' + path + ' -> ' + res.status + ' ' + res.body);
    }
    return res.body ? JSON.parse(res.body) : null;
  }`;

  const postId = await evalJs(`
    (async () => {
      const apiRequest = ${apiRequest};
      const data = await apiRequest('POST', '/api/posts', '{}');
      const pk = data.post_pk;
      return pk.includes('#') ? pk.split('#')[1] : pk;
    })()
  `);
  expect(postId, "post creation must return post_pk").toBeTruthy();

  const spaceId = await evalJs(`
    (async () => {
      const apiRequest = ${apiRequest};
      const data = await apiRequest(
        'POST', '/api/spaces/create',
        JSON.stringify({ req: { post_id: ${JSON.stringify(postId)} } }),
      );
      return data.space_id;
    })()
  `);
  expect(spaceId, "space creation must return space_id").toBeTruthy();

  // ── 4. Verify space is queryable through the native transport ─────────
  const spaceTitle = await evalJs(`
    (async () => {
      const apiRequest = ${apiRequest};
      const data = await apiRequest('GET', '/api/spaces/' + ${JSON.stringify(spaceId)}, undefined);
      return data.title ?? null;
    })()
  `);
  expect(spaceTitle).not.toBeNull();
});
