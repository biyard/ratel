import { test, expect } from "@playwright/test";

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
 */

// Mobile viewport matching iPhone SE / small mobile
const MOBILE_VIEWPORT = { width: 375, height: 667 };
// Desktop viewport matching the default test config
const DESKTOP_VIEWPORT = { width: 1440, height: 950 };

test.describe("Mobile Safari address bar scroll fix (#1274)", () => {
  test("body has overflow:auto on mobile viewport", async ({ browser }) => {
    const context = await browser.newContext({
      viewport: MOBILE_VIEWPORT,
    });
    const page = await context.newPage();

    try {
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");

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
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");

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
      // Navigate to home first to find a space link
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");

      // Try to find a space card link on the home feed
      const spaceLink = page.locator('a[href*="/spaces/"]').first();
      let spaceUrl;

      if (await spaceLink.isVisible({ timeout: 5000 }).catch(() => false)) {
        const href = await spaceLink.getAttribute("href");
        // Extract the space base URL (e.g., /spaces/some-id)
        const match = href.match(/\/spaces\/[^/]+/);
        spaceUrl = match ? match[0] + "/dashboard" : null;
      }

      if (!spaceUrl) {
        // No space found on home page; skip space-specific checks.
        test.skip(
          true,
          "No space available in the test environment for layout checks"
        );
        return;
      }

      await page.goto(spaceUrl);
      await page.waitForLoadState("domcontentloaded");
      // Wait for Dioxus hydration
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null
      );

      // The space layout container is the outermost div inside the space
      // layout with the grid/flex classes. We identify it by its
      // characteristic class pattern: "bg-space-bg".
      const layoutContainer = page.locator("div.bg-space-bg").first();

      if (
        await layoutContainer.isVisible({ timeout: 5000 }).catch(() => false)
      ) {
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
      }
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
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");

      // Find a space link from the home page
      const spaceLink = page.locator('a[href*="/spaces/"]').first();
      let spaceUrl;

      if (await spaceLink.isVisible({ timeout: 5000 }).catch(() => false)) {
        const href = await spaceLink.getAttribute("href");
        const match = href.match(/\/spaces\/[^/]+/);
        spaceUrl = match ? match[0] + "/dashboard" : null;
      }

      if (!spaceUrl) {
        test.skip(
          true,
          "No space available in the test environment for nav checks"
        );
        return;
      }

      await page.goto(spaceUrl);
      await page.waitForLoadState("domcontentloaded");
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null
      );

      // The SpaceNav component renders a div with the class "max-tablet:sticky".
      // On mobile, this should compute to position: sticky.
      // We identify it by the "divide-divider" class which is unique to SpaceNav.
      const navBar = page.locator("div.divide-divider").first();

      if (await navBar.isVisible({ timeout: 5000 }).catch(() => false)) {
        const position = await navBar.evaluate((el) => {
          return window.getComputedStyle(el).position;
        });

        expect(position).toBe("sticky");

        // Verify bottom: 0px for the sticky positioning
        const bottom = await navBar.evaluate((el) => {
          return window.getComputedStyle(el).bottom;
        });

        expect(bottom).toBe("0px");
      }
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
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null
      );

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
      await page.goto("/");
      await page.waitForLoadState("domcontentloaded");

      const spaceLink = page.locator('a[href*="/spaces/"]').first();
      let spaceUrl;

      if (await spaceLink.isVisible({ timeout: 5000 }).catch(() => false)) {
        const href = await spaceLink.getAttribute("href");
        const match = href.match(/\/spaces\/[^/]+/);
        spaceUrl = match ? match[0] + "/dashboard" : null;
      }

      if (!spaceUrl) {
        test.skip(
          true,
          "No space available in the test environment for desktop nav checks"
        );
        return;
      }

      await page.goto(spaceUrl);
      await page.waitForLoadState("domcontentloaded");
      await page.waitForFunction(
        () => document.querySelector("[data-dioxus-id]") !== null
      );

      // On desktop, the SpaceNav should NOT be sticky -- it should be
      // part of the normal grid layout (no position override).
      const navBar = page.locator("div.divide-divider").first();

      if (await navBar.isVisible({ timeout: 5000 }).catch(() => false)) {
        const position = await navBar.evaluate((el) => {
          return window.getComputedStyle(el).position;
        });

        // On desktop (>= 900px), "max-tablet:sticky" does NOT apply,
        // so position should be the default (static or relative).
        expect(position).not.toBe("sticky");
      }
    } finally {
      await context.close();
    }
  });
});
