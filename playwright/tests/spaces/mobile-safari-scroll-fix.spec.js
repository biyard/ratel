import { test, expect } from "@playwright/test";
import { goto, getLocator } from "../utils.js";

/**
 * Mobile Safari Address Bar Scroll Fix (Issue #1274)
 *
 * This test verifies the CSS/layout changes that fix the mobile Safari
 * address bar behavior. On mobile viewports (< 900px tablet breakpoint),
 * the body and space layout must allow native scrolling so that the
 * browser address bar can hide/show naturally.
 *
 * Changes under test:
 *   1. tailwind.css: body { overflow-hidden } -> body { overflow-hidden max-tablet:overflow-auto }
 *   2. spaces/layout.rs: layout container gets max-tablet:overflow-visible, max-tablet:h-auto, max-tablet:min-h-screen
 *   3. space_nav/mod.rs: bottom nav gets max-tablet:sticky, max-tablet:bottom-0, max-tablet:bg-space-bg
 *
 * The "max-tablet" breakpoint is max-width: 899px (tablet breakpoint is 900px).
 *
 * This test does NOT require authentication for body-level checks.
 * Space-specific layout tests use the saved storageState from auth-setup.
 *
 * NOTE: Requires backend built with --features bypass for auth flows.
 *
 * Environment variables:
 *   PLAYWRIGHT_TEST_SPACE_URL - A known space dashboard URL (e.g., "/spaces/SPACE%23abc123/dashboard").
 *     When set, space-specific tests navigate directly to this URL instead of
 *     scanning the home feed. This makes tests deterministic in CI.
 */

// Mobile viewport matching iPhone SE / small mobile
const MOBILE_VIEWPORT = { width: 375, height: 667 };
// Desktop viewport matching the default test config
const DESKTOP_VIEWPORT = { width: 1440, height: 950 };

/**
 * Resolve a space dashboard URL for testing.
 *
 * Priority:
 *   1. PLAYWRIGHT_TEST_SPACE_URL env var (deterministic, preferred for CI)
 *   2. Scan the home feed for a link containing /spaces/ (fallback for local dev)
 *
 * @returns {Promise<string|null>} A space dashboard URL or null if none found.
 */
async function resolveSpaceDashboardUrl(page) {
  // 1. Prefer deterministic env var
  const envUrl = process.env.PLAYWRIGHT_TEST_SPACE_URL;
  if (envUrl) {
    return envUrl;
  }

  // 2. Fallback: scan the home feed for a /spaces/ link using getLocator
  //    with role-based selector. We use page.getByRole directly here
  //    because getLocator awaits visibility of a single element, and we
  //    need to iterate over all matching links.
  const links = page.getByRole("link");
  const count = await links.count();

  for (let i = 0; i < count; i++) {
    const link = links.nth(i);
    const href = await link.getAttribute("href");
    if (href && href.includes("/spaces/")) {
      const match = href.match(/\/spaces\/[^/]+/);
      return match ? match[0] + "/dashboard" : null;
    }
  }

  return null;
}

test.describe("Mobile Safari address bar scroll fix (#1274)", () => {
  test("body has overflow:auto on mobile viewport", async ({ browser }) => {
    const context = await browser.newContext({
      viewport: MOBILE_VIEWPORT,
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Verify the body element has overflow: auto (not hidden) on mobile
      const bodyOverflow = await page.evaluate(() => {
        return window.getComputedStyle(document.body).overflow;
      });

      // On mobile (< 900px), the Tailwind class "max-tablet:overflow-auto"
      // should override "overflow-hidden", resulting in overflow: auto
      expect(bodyOverflow).toBe("auto");
    } finally {
      await context.close();
    }
  });

  test("body has overflow:hidden on desktop viewport", async ({ browser }) => {
    const context = await browser.newContext({
      viewport: DESKTOP_VIEWPORT,
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // Verify the body element retains overflow: hidden on desktop
      const bodyOverflow = await page.evaluate(() => {
        return window.getComputedStyle(document.body).overflow;
      });

      expect(bodyOverflow).toBe("hidden");
    } finally {
      await context.close();
    }
  });

  test("space layout container allows scrolling on mobile viewport", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      viewport: MOBILE_VIEWPORT,
      // Reuse auth storage state so the space page renders fully
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      const spaceUrl = await resolveSpaceDashboardUrl(page);

      if (!spaceUrl) {
        test.skip(
          true,
          "No space available -- set PLAYWRIGHT_TEST_SPACE_URL env var for deterministic CI runs"
        );
        return;
      }

      await goto(page, spaceUrl);

      // Identify the space layout container via a stable data-testid attribute.
      const layoutContainer = await getLocator(page, {
        testId: "space-layout-container",
      });

      const styles = await layoutContainer.evaluate((el) => {
        const computed = window.getComputedStyle(el);
        return {
          overflow: computed.overflow,
          overflowY: computed.overflowY,
          height: computed.height,
          minHeight: computed.minHeight,
        };
      });

      // On mobile, the layout container should NOT have overflow: hidden.
      // The max-tablet:overflow-visible class makes it "visible".
      expect(styles.overflow).not.toBe("hidden");
      expect(styles.overflowY).not.toBe("hidden");

      // The container should NOT have a fixed height of exactly the viewport
      // height (h-screen becomes h-auto on mobile via max-tablet:h-auto).
      // Since min-h-screen is set, minHeight should be a non-zero value.
      const minHeightPx = parseFloat(styles.minHeight);
      expect(minHeightPx).toBeGreaterThan(0);
    } finally {
      await context.close();
    }
  });

  test("space bottom nav has position:sticky on mobile viewport", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      viewport: MOBILE_VIEWPORT,
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      const spaceUrl = await resolveSpaceDashboardUrl(page);

      if (!spaceUrl) {
        test.skip(
          true,
          "No space available -- set PLAYWRIGHT_TEST_SPACE_URL env var for deterministic CI runs"
        );
        return;
      }

      await goto(page, spaceUrl);

      // Identify SpaceNav via stable data-testid for reliable targeting.
      const navBar = await getLocator(page, { testId: "space-nav-root" });

      const position = await navBar.evaluate((el) => {
        return window.getComputedStyle(el).position;
      });

      expect(position).toBe("sticky");

      // Verify bottom: 0px for the sticky positioning
      const bottom = await navBar.evaluate((el) => {
        return window.getComputedStyle(el).bottom;
      });

      expect(bottom).toBe("0px");
    } finally {
      await context.close();
    }
  });

  test("page is scrollable on mobile viewport when content overflows", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      viewport: MOBILE_VIEWPORT,
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      // On mobile, the body has overflow: auto, so the document should be
      // scrollable when content exceeds the viewport height.
      const scrollInfo = await page.evaluate(() => {
        return {
          scrollHeight: document.documentElement.scrollHeight,
          innerHeight: window.innerHeight,
          bodyOverflow: window.getComputedStyle(document.body).overflow,
        };
      });

      // The body must have overflow: auto to allow scrolling
      expect(scrollInfo.bodyOverflow).toBe("auto");

      // scrollHeight should be at least as large as the viewport
      // (content may or may not overflow depending on data, but the
      // overflow property must allow it)
      expect(scrollInfo.scrollHeight).toBeGreaterThanOrEqual(
        scrollInfo.innerHeight
      );
    } finally {
      await context.close();
    }
  });

  test("space bottom nav is NOT sticky on desktop viewport", async ({
    browser,
  }) => {
    const context = await browser.newContext({
      viewport: DESKTOP_VIEWPORT,
      storageState: "user.json",
    });
    const page = await context.newPage();

    try {
      await goto(page, "/");

      const spaceUrl = await resolveSpaceDashboardUrl(page);

      if (!spaceUrl) {
        test.skip(
          true,
          "No space available -- set PLAYWRIGHT_TEST_SPACE_URL env var for deterministic CI runs"
        );
        return;
      }

      await goto(page, spaceUrl);

      // Identify SpaceNav via stable data-testid for reliable targeting.
      const navBar = await getLocator(page, { testId: "space-nav-root" });

      const position = await navBar.evaluate((el) => {
        return window.getComputedStyle(el).position;
      });

      // On desktop (>= 900px), "max-tablet:sticky" does NOT apply,
      // so position should be the default (static or relative).
      expect(position).not.toBe("sticky");
    } finally {
      await context.close();
    }
  });
});
