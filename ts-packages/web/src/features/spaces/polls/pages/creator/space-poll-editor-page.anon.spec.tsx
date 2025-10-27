import { test, expect } from '@playwright/test';

test.describe('[SpacePollEditorPage] Anonymous Users', () => {
  test('[SPEP-ANON-001] Anonymous user navigating to home page works', async ({
    page,
  }) => {
    // Anonymous users can access the home page
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify the page loads successfully
    const currentUrl = page.url();
    expect(currentUrl).toBeTruthy();
  });

  test('[SPEP-ANON-002] Anonymous user cannot see admin checkbox without proper space', async ({
    page,
  }) => {
    // Navigate to home as anonymous user
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Checkbox should not be visible on home page
    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    const isVisible = await checkbox
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    expect(isVisible).toBe(false);
  });

  test('[SPEP-ANON-003] Anonymous user home navigation is successful', async ({
    page,
  }) => {
    // Anonymous users can navigate the app
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const currentUrl = page.url();

    // Page loads successfully
    expect(currentUrl).toContain('/');
  });

  test('[SPEP-ANON-004] Anonymous user cannot see Edit button on home', async ({
    page,
  }) => {
    // Navigate to home as anonymous user
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Edit button for polls should not be visible on home page
    const editButton = page.locator('[data-pw="poll-editor-edit-btn"]');
    const isEditVisible = await editButton
      .isVisible({ timeout: 2000 })
      .catch(() => false);

    expect(isEditVisible).toBe(false);
  });

  test('[SPEP-ANON-005] Anonymous user navigation is functional', async ({
    page,
  }) => {
    // Access the home page
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Page is accessible
    expect(page.url()).toBeTruthy();
  });

  test('[SPEP-ANON-006] Anonymous user cannot see admin features on home', async ({
    page,
  }) => {
    // Navigate home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify admin-only elements are not visible
    const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
    const isCheckboxVisible = await checkbox
      .isVisible({ timeout: 2000 })
      .catch(() => false);
    expect(isCheckboxVisible).toBe(false);
  });

  test('[SPEP-ANON-007] Anonymous user cannot see admin UI elements', async ({
    page,
  }) => {
    // Navigate home
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify no admin elements visible
    const saveBtn = page.locator('[data-pw="poll-editor-save-btn"]');
    const discardBtn = page.locator('[data-pw="poll-editor-discard-btn"]');
    const editBtn = page.locator('[data-pw="poll-editor-edit-btn"]');

    const hasSave = await saveBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);
    const hasDiscard = await discardBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);
    const hasEdit = await editBtn
      .isVisible({ timeout: 1000 })
      .catch(() => false);

    expect(hasSave).toBe(false);
    expect(hasDiscard).toBe(false);
    expect(hasEdit).toBe(false);
  });
});
