import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';
import { click } from '@tests/utils';

/**
 * Test suite for Space Home Page - Edit Icons Unification
 * Issue #742: Verifies that edit icons are unified for both title and content
 */

test.describe.serial('[SpaceHomePage] Edit Icons Unification - Issue #742', () => {
  let context: import('@playwright/test').BrowserContext;
  let page: import('@playwright/test').Page;
  let spaceUrl = '';

  test.beforeAll('Create a test space', async ({ browser }) => {
    context = await browser.newContext({ storageState: 'user.json' });
    page = await context.newPage();
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Create a test post first
    const testTitle = 'Test Post for Space Edit Icons Test';
    const testContent =
      'This is a test post created to verify that edit icons are properly ' +
      'unified across the space page. Both title and content should use the ' +
      'same Edit1 icon for consistency. This content is long enough to meet ' +
      'the minimum requirements for post creation and will be used to create ' +
      'a space where we can test the edit icon functionality.';

    await click(page, { label: 'Create Post' });
    await page.waitForURL(/\/posts\/new/, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });

    await page.fill('#post-title-input', testTitle);

    const editorSelector = '[data-pw="post-content-editor"] .ProseMirror';
    await page.waitForSelector(editorSelector, {
      timeout: CONFIGS.PAGE_WAIT_TIME,
    });
    await page.click(editorSelector);
    await page.fill(editorSelector, testContent);

    await page.click('#publish-post-button');
    await page.waitForURL(/\/threads\/.+/, { timeout: 15000 });

    // Navigate to thread and create space
    await page.waitForTimeout(3000);

    await expect(page.getByText('Create a Space', { exact: true })).toBeVisible(
      { timeout: 20000 },
    );
    await page.getByText('Create a Space', { exact: true }).click();

    const modal = page.getByRole('dialog', { name: 'Select a Space Type' });
    await modal
      .locator('div.cursor-pointer', { hasText: 'Deliberation' })
      .click();

    await modal.getByRole('button', { name: 'Create' }).click();

    await page.waitForURL(/\/spaces\/[^/]+(?:\?.*)?$/, { timeout: 15000 });
    spaceUrl = page.url();
  });

  test('[SHP-001] Title section should display Edit1 icon when user is admin', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Look for edit icons with the specific styling
    const editIcons = page.locator(
      'svg[class*="text-gray-400"][class*="cursor-pointer"]',
    );

    // Should have at least one edit icon visible (for title)
    const count = await editIcons.count();
    expect(count).toBeGreaterThanOrEqual(1);

    // Verify the first edit icon (title edit) is visible
    await expect(editIcons.first()).toBeVisible();
  });

  test('[SHP-002] Title edit icon should have correct styling and role', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Find the edit icon with role="button"
    const titleEditIcon = page.locator('[role="button"]').first();

    // Verify it has the correct styling classes
    const classes = await titleEditIcon.getAttribute('class');
    expect(classes).toContain('text-gray-400');
    expect(classes).toContain('cursor-pointer');
    expect(classes).toContain('hover:text-gray-600');

    // Verify it has the correct dimensions
    expect(classes).toContain('w-5');
    expect(classes).toContain('h-5');
  });

  test('[SHP-003] Content section should display Edit1 icon when user is admin', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Look for the content editor's edit icon
    // It should be inside or near the Card component
    const editIcons = page.locator(
      'svg[class*="text-gray-400"][class*="cursor-pointer"]',
    );

    // Should have at least two edit icons (title + content)
    const count = await editIcons.count();
    expect(count).toBeGreaterThanOrEqual(2);
  });

  test('[SHP-004] Both title and content should use the same Edit1 icon style', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const editIcons = page.locator(
      'svg[class*="text-gray-400"][class*="cursor-pointer"]',
    );

    const count = await editIcons.count();
    expect(count).toBeGreaterThanOrEqual(2);

    // Get classes from first two icons
    const firstIconClasses = await editIcons.nth(0).getAttribute('class');
    const secondIconClasses = await editIcons.nth(1).getAttribute('class');

    // Both should have the same core styling classes
    expect(firstIconClasses).toContain('text-gray-400');
    expect(secondIconClasses).toContain('text-gray-400');
    expect(firstIconClasses).toContain('cursor-pointer');
    expect(secondIconClasses).toContain('cursor-pointer');
    expect(firstIconClasses).toContain('w-5');
    expect(secondIconClasses).toContain('w-5');
    expect(firstIconClasses).toContain('h-5');
    expect(secondIconClasses).toContain('h-5');
  });

  test('[SHP-005] Clicking title edit icon should enable edit mode', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Click the first edit icon (title)
    const titleEditIcon = page.locator('[role="button"]').first();
    await titleEditIcon.click();

    // Wait for input to appear
    await page.waitForTimeout(1000);

    // Verify that an input field appears (edit mode)
    const titleInput = page.locator('input').first();
    await expect(titleInput).toBeVisible({ timeout: 5000 });
  });

  test('[SHP-006] Clicking content edit icon should enable editor toolbar', async () => {
    await page.goto(spaceUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Find edit icons
    const editIcons = page.locator(
      'svg[class*="text-gray-400"][class*="cursor-pointer"]',
    );

    // Click the content edit icon (usually the second one)
    if ((await editIcons.count()) >= 2) {
      await editIcons.nth(1).click();
      await page.waitForTimeout(1000);

      // Check if editor toolbar or editing interface appears
      // The exact selector depends on the TiptapEditor implementation
      // Looking for any indication that edit mode is active
      const editorContent = page.locator('.ProseMirror, [role="toolbar"]');
      const isVisible = await editorContent.count();
      expect(isVisible).toBeGreaterThan(0);
    }
  });

  test.afterAll(async () => {
    await context.close();
  });
});
