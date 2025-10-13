import { defineConfig, devices } from '@playwright/test';
import { CONFIGS } from './tests/config';

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// import dotenv from 'dotenv';
// import path from 'path';
// dotenv.config({ path: path.resolve(__dirname, '.env') });

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: './',
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: 2,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [['html', { host: '0.0.0.0', port: 8900 }]],
  timeout: CONFIGS.PLAYWRIGHT.TIMEOUT,
  /* Global setup and teardown */
  // globalSetup: require.resolve("./tests/global-setup"),
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: CONFIGS.PLAYWRIGHT.BASE_URL,
    navigationTimeout: CONFIGS.PLAYWRIGHT.NAVIGATION_TIME_OUT,
    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on',
    video: 'on',
  },

  /* Configure projects for major browsers */
  projects: [
    // Anonymous tests (no setup required)
    {
      name: 'anonymous',
      testMatch: ['**/*.anon.spec.ts', '**/*.anon.spec.tsx'],
      use: {
        ...devices['Desktop Chrome'],
        viewport: {
          width: 1440,
          height: 950,
        },
      },
    },

    // Authenticated tests (requires global setup)
    {
      name: 'auth-setup',
      testMatch: ['**/*.auth.setup.ts', '**/*.auth.setup.tsx'],
    },
    {
      name: 'authenticated',
      testMatch: ['**/*.auth.spec.ts', '**/*.auth.spec.tsx'],
      dependencies: ['auth-setup'],
      use: {
        ...devices['Desktop Chrome'],
        viewport: {
          width: 1440,
          height: 950,
        },
        // This will be loaded in the beforeEach of authenticated tests
        storageState: 'user.json',
      },
    },
  ],
});
