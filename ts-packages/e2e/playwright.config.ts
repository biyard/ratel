import { defineConfig, devices } from "@playwright/test";
import { BASE_URL, TIMEOUT } from "./configs";

/**
 * Playwright configuration for E2E tests
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "./deliberation",
  /* Run tests in files in parallel */
  fullyParallel: false, // Deliberation tests have dependencies
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: 1,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [["html", { open: "never", host: "0.0.0.0" }], ["list"]],
  timeout: TIMEOUT,
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: BASE_URL,
    navigationTimeout: TIMEOUT,
    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on-first-retry",
    video: "retain-on-failure",
    screenshot: "only-on-failure",
  },

  /* Configure projects for major browsers */
  projects: [
    // Desktop tests
    {
      name: "desktop",
      testMatch: ["**/*.desktop.spec.ts"],
      use: {
        ...devices["Desktop Chrome"],
      },
    },

    // Mobile tests
    {
      name: "mobile",
      testMatch: ["**/*.mobile.spec.ts"],
      use: {
        ...devices["Pixel 5"],
      },
    },
  ],
});
