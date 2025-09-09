import { test, expect, Page, ConsoleMessage } from "@playwright/test";
import { wrap } from "./utils";
import { CONFIGS } from "./config";

// Helper to log console messages
const setupConsoleLogging = async (page: Page) => {
  page.on('console', (msg: ConsoleMessage) => {
    console.log(`[Browser Console] ${msg.type()}: ${msg.text()}`);
  });
  page.on('pageerror', (error) => {
    console.log(`[Page Error] ${error.message}`);
  });
  page.on('response', (response) => {
    if (response.status() >= 400) {
      console.log(`[${response.status()}] ${response.url()}`);
    }
  });
};

// Helper functions
async function login(page: Page) {
  await page.goto("/login");
  await page.fill('input[name="email"]', process.env.TEST_EMAIL || 'test@example.com');
  await page.fill('input[name="password"]', process.env.TEST_PASSWORD || 'password');
  await page.click('button[type="submit"]');
  await page.waitForURL('**/home');
}

async function createTestThread(page: Page): Promise<number> {
  // Navigate to create thread page or use an existing thread
  await page.goto('/create-thread');
  await page.fill('input[name="title"]', 'Test Thread');
  await page.click('button[type="submit"]');
  
  // Wait for thread creation and get the thread ID from the URL
  await page.waitForURL(/\/threads\/\d+$/);
  const url = page.url();
  console.log('Thread URL:', url); // Added debug logging
  const threadId = parseInt(url.split('/').pop() || '1');
  console.log('Thread ID:', threadId); // Added debug logging
  return threadId;
}

test("[Space-001] Verify Poll space creation and navigation", async ({
  page,
  browserName,
}, testInfo) => {
  // Setup console logging
  await setupConsoleLogging(page);
  
  // Enable request/response logging
  await page.route('**', route => {
    console.log(`Request: ${route.request().method()} ${route.request().url()}`);
    return route.continue();
  });
  try {
    const p = wrap(page, testInfo.project.name, "space-creation/001-poll");
    
    // Login and navigate to a thread
    await login(page);
    await p.fullCapture("after-login");
    
    // Create a test thread or use an existing one
    const threadId = await createTestThread(page);
    console.log('Created thread ID:', threadId); // Added debug logging
    await p.goto(`/threads/${threadId}`, { waitUntil: "load" });
    await p.fullCapture("thread-page");
    
    // Open space creation modal
    await p.clickXpathAndCapture(
      "//button[contains(., 'Create Space')]",
      "open-space-creation"
    );
    
    // Select Poll space type
    await p.clickXpathAndCapture(
      "//button[contains(., 'Poll')]",
      "select-poll-space"
    );
    
    // Click the Send button
    await p.clickXpathAndCapture(
      "//button[contains(., 'Send')]",
      "submit-space-creation"
    );
    
    // Wait for navigation to space page
    await page.waitForURL(/\/spaces\/\d+$/, { timeout: CONFIGS.PAGE_WAIT_TIME });
    await p.fullCapture("space-page");
    
    // Verify we're on a space page
    await expect(page).toHaveURL(/\/spaces\/\d+$/);
    
    // Verify the space type is Poll
    const spaceType = await page.locator('[data-testid="space-type"]').textContent();
    expect(spaceType?.toLowerCase()).toContain('poll');
    
  } catch (error) {
    console.error('Test failed at step:', error);
    console.log('Current URL:', page.url());
    console.log('Page content:', await page.content());
    throw error;
  }
});
