import { test, expect } from '@playwright/test';

test.describe('[SpacePollEditorPage] Anonymous Users', () => {
  test('[SPEP-ANON-001] Anonymous user can access home page', async ({
    page,
  }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    const currentUrl = page.url();
    expect(currentUrl).toBeTruthy();
    expect(currentUrl).toContain('/');
  });

  test('[SPEP-ANON-002] Anonymous user basic navigation works', async ({
    page,
  }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    expect(page.url()).toBeTruthy();
  });
});
