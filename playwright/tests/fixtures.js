import { test as base, expect } from "@playwright/test";

// Extend the default `page` fixture so every spec that imports from this
// file gets automatic browser-side error logging. We only surface
// `console.error` + uncaught page errors — warnings/info/log are ignored
// to keep the output focused on actionable failures.
export const test = base.extend({
  page: async ({ page }, use, testInfo) => {
    const tag = `[${testInfo.titlePath.slice(-2).join(" \u203a ")}]`;

    page.on("console", (msg) => {
      if (msg.type() === "error") {
        console.log(`${tag} [console.error]`, msg.text());
      }
    });
    page.on("pageerror", (err) => {
      console.log(`${tag} [pageerror]`, err.message);
    });

    await use(page);
  },
});

export { expect };
