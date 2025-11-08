import { test, expect } from '@playwright/test';
import { CONFIGS } from '@tests/config';

test.describe.serial('[User Settings - My Settings] Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    test.setTimeout(60000); // Increase test timeout to 60 seconds
    await page.goto('/settings', { timeout: 30000 });
    await page.waitForLoadState('networkidle', { timeout: 30000 });

    const settingsTab = page.getByRole('tab', { name: /settings/i });
    await expect(settingsTab).toBeVisible();
    await settingsTab.click();
    await page.waitForTimeout(CONFIGS.PAGE_WAIT_TIME / 20);
    await expect(settingsTab).toHaveAttribute('aria-selected', 'true');
  });

  test('[MS-001] Should display all settings options correctly', async ({
    page,
  }) => {
    const appearanceHeader = page.getByText(/appearance/i).first();
    await expect(appearanceHeader).toBeVisible();

    const languageBox = page.locator('[data-pw="language-setting-box"]');
    await expect(languageBox).toBeVisible();

    const themeBox = page.locator('[data-pw="theme-setting-box"]');
    await expect(themeBox).toBeVisible();
  });

  test('[MS-002] Should open language modal when clicking language setting', async ({
    page,
  }) => {
    const languageButton = page.locator(
      '[data-pw="language-setting-box-button"]',
    );
    await expect(languageButton).toBeVisible();
    await languageButton.click();

    // Wait for modal to appear by checking for the options
    const englishOption = page.locator('[data-pw="locale-option-en"]');
    const koreanOption = page.locator('[data-pw="locale-option-ko"]');

    await expect(englishOption).toBeVisible({
      timeout: CONFIGS.SELECTOR_WAIT_TIME,
    });
    await expect(koreanOption).toBeVisible({
      timeout: CONFIGS.SELECTOR_WAIT_TIME,
    });

    const cancelButton = page.locator('[data-pw="locale-cancel-button"]');
    await expect(cancelButton).toBeVisible();
    await cancelButton.click();
  });

  test('[MS-003] Should open theme modal when clicking theme setting', async ({
    page,
  }) => {
    const themeButton = page.locator('[data-pw="theme-setting-box-button"]');
    await expect(themeButton).toBeVisible();
    await themeButton.click();
    await page.waitForTimeout(CONFIGS.MODAL_WAIT_TIME);

    const darkOption = page.locator('[data-pw="theme-option-dark"]');
    const lightOption = page.locator('[data-pw="theme-option-light"]');

    await expect(darkOption).toBeVisible({ timeout: 5000 });
    await expect(lightOption).toBeVisible({ timeout: 5000 });

    const cancelButton = page.locator('[data-pw="theme-cancel-button"]');
    await cancelButton.click();
  });

  test('[MS-004] Should display correct page URL', async ({ page }) => {
    await expect(page).toHaveURL(/\/settings$/);
  });
});
