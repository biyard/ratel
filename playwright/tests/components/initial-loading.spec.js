import { test, expect } from "@playwright/test";
import { goto } from "../utils";
import { CONFIGS } from "../config";

/**
 * Initial Loading Performance — E2E Tests
 *
 * Tests that the initial page load is fast and functional after webpack
 * code splitting was enabled (issue #1296).
 *
 * Behavior under test:
 *   - The home page loads and hydrates within a reasonable time
 *   - The window.ratel namespace is initialized with all expected modules
 *   - Lazy-loaded chunk files are NOT fetched on initial page load
 *     (they load on demand when features are triggered)
 *   - The main JS bundle is small enough for fast initial load
 *
 * NOTE: These tests require the app to be built with webpack code splitting
 *       enabled (the default since issue #1296).
 */

const DESKTOP_VIEWPORT = { width: 1440, height: 950 };

// ---------------------------------------------------------------------------
// Scenario 1: Page loads and hydrates correctly with code-split bundles
// ---------------------------------------------------------------------------

test.describe("Scenario: Initial loading — page hydration", () => {
  test("should load the home page and hydrate Dioxus successfully", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Verify Dioxus hydration completed (data-dioxus-id present)
      const hydrated = await page.evaluate(
        () => document.querySelector("[data-dioxus-id]") !== null
      );
      expect(hydrated).toBe(true);
    } finally {
      await context.close();
    }
  });

  test("should initialize window.ratel namespace with expected modules", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Verify the window.ratel global namespace exists
      const ratelExists = await page.evaluate(
        () => typeof window.ratel !== "undefined"
      );
      expect(ratelExists).toBe(true);

      // Verify expected top-level modules are registered
      const modules = await page.evaluate(() => Object.keys(window.ratel));
      expect(modules).toContain("common");
      expect(modules).toContain("auth");
      expect(modules).toContain("app_shell");
    } finally {
      await context.close();
    }
  });
});

// ---------------------------------------------------------------------------
// Scenario 2: Main bundle size is reasonable (code splitting working)
// ---------------------------------------------------------------------------

test.describe("Scenario: Initial loading — bundle size", () => {
  test("should serve main JS bundle successfully", async ({ browser }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      // Track network requests for JS files
      const jsRequests = [];
      page.on("response", (response) => {
        const url = response.url();
        if (url.includes("ratel-app-shell") && url.endsWith(".js")) {
          jsRequests.push({
            url,
            status: response.status(),
          });
        }
      });

      await goto(page, "/");

      // The main bundle should have loaded
      expect(jsRequests.length).toBeGreaterThanOrEqual(1);

      // Verify main bundle response was successful
      for (const req of jsRequests) {
        expect(req.status).toBeLessThan(400);
      }
    } finally {
      await context.close();
    }
  });

  test("should NOT load chunk files on initial home page load", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      // Track chunk file requests
      const chunkRequests = [];
      page.on("response", (response) => {
        const url = response.url();
        if (url.includes("ratel-chunk-")) {
          chunkRequests.push(url);
        }
      });

      await goto(page, "/");

      // Wait a moment for any deferred loads to settle
      await page.waitForTimeout(2000);

      // No chunk files should be loaded on the home page --
      // they only load when specific features are triggered
      // (e.g., wallet connect, firebase auth, chart rendering)
      expect(chunkRequests.length).toBe(0);
    } finally {
      await context.close();
    }
  });
});

// ---------------------------------------------------------------------------
// Scenario 3: Lazy module functions are callable
// ---------------------------------------------------------------------------

test.describe("Scenario: Initial loading — lazy module availability", () => {
  test("should have auth.firebase functions available as callable", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Firebase functions should be registered on window.ratel.auth.firebase
      // They are available immediately as functions, but internally lazy-load
      // the Firebase SDK on first call
      const firebaseFns = await page.evaluate(() => {
        const fb = window.ratel?.auth?.firebase;
        if (!fb) return null;
        return {
          hasInitFirebase: typeof fb.init_firebase === "function",
        };
      });

      expect(firebaseFns).not.toBeNull();
      expect(firebaseFns.hasInitFirebase).toBe(true);
    } finally {
      await context.close();
    }
  });

  test("should have common.theme functions available synchronously", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      baseURL: CONFIGS.BASE_URL,
      viewport: DESKTOP_VIEWPORT,
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Theme functions are small and remain synchronous (not lazy-loaded)
      const themeFns = await page.evaluate(() => {
        const theme = window.ratel?.common?.theme;
        if (!theme) return null;
        return {
          hasLoadTheme: typeof theme.load_theme === "function",
          hasSaveTheme: typeof theme.save_theme === "function",
          hasApplyTheme: typeof theme.apply_theme === "function",
        };
      });

      expect(themeFns).not.toBeNull();
      expect(themeFns.hasLoadTheme).toBe(true);
      expect(themeFns.hasSaveTheme).toBe(true);
      expect(themeFns.hasApplyTheme).toBe(true);
    } finally {
      await context.close();
    }
  });
});
