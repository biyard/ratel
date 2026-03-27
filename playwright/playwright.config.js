// @ts-check
import { defineConfig, devices } from "@playwright/test";
import { CONFIGS } from "./tests/config";

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// import dotenv from 'dotenv';
// import path from 'path';
// dotenv.config({ path: path.resolve(__dirname, '.env') });

/**
 * @see https://playwright.dev/docs/test-configuration
 */
export default defineConfig({
  testDir: ".",
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [["html", { open: "never", host: "0.0.0.0" }]],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  timeout: CONFIGS.TIMEOUT,
  use: {
    baseURL: CONFIGS.BASE_URL,
    navigationTimeout: CONFIGS.TIMEOUT,
    locale: "en-US",
    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on",
    video: "on",
    screenshot: "on",
  },

  /* Configure projects for major browsers */
  projects: [
    // Authenticated tests (requires global setup)
    {
      name: "auth-setup",
      testMatch: ["**/*.auth.setup.js"],
    },
    {
      name: "Individual user tests",
      testMatch: ["tests/users/**/*.spec.js", "tests/spaces/**/*.spec.js"],
      dependencies: ["auth-setup"],
      use: {
        ...devices["Desktop Chrome"],
        viewport: {
          width: 1440,
          height: 950,
        },
        // This will be loaded in the beforeEach of authenticated tests
        storageState: "user.json",
      },
    },
    // Component tests manage their own browser contexts and auth state
    {
      name: "Component tests",
      testMatch: ["tests/components/**/*.spec.js"],
      dependencies: ["auth-setup"],
      use: {
        ...devices["Desktop Chrome"],
      },
    },
  ],

  /* Run your local dev server before starting the tests */
  // webServer: {
  //   command: 'cd ../app/shell && make run',
  //   url: 'http://localhost:8000',
  //   reuseExistingServer: !process.env.CI,
  // },
});
