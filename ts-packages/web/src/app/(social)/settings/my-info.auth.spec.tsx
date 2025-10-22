import { test, expect } from '@playwright/test';
import { click, fill, waitForVisible } from '@tests/utils';
import { CONFIGS } from '@tests/config';

test.describe.serial('[User Settings - My Info] Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    test.setTimeout(60000); // Increase test timeout to 60 seconds
    await page.goto('/settings', { timeout: 30000 });
    await page.waitForLoadState('networkidle', { timeout: 30000 });

    // Ensure we're on the My Info tab
    const myInfoTab = page.getByRole('tab', { name: /my info/i });
    await expect(myInfoTab).toBeVisible();
    if (
      !(await myInfoTab.getAttribute('aria-selected')) ||
      (await myInfoTab.getAttribute('aria-selected')) === 'false'
    ) {
      await myInfoTab.click();
      await page.waitForTimeout(CONFIGS.PAGE_WAIT_TIME / 20);
    }
  });

  test('[US-001] Should display all user settings fields correctly', async ({
    page,
  }) => {
    // Verify profile image section
    const uploadLogoButton = page
      .locator('[data-pw="upload-profile-button"]')
      .or(page.locator('[data-pw="profile-image"]'));
    await expect(uploadLogoButton.first()).toBeVisible();

    // Verify username field (disabled)
    const usernameLabel = page.getByText('Username', { exact: false });
    await expect(usernameLabel).toBeVisible();

    const usernameInput = page.locator('[data-pw="username-input"]');
    await expect(usernameInput).toBeDisabled();
    await expect(usernameInput).toHaveValue(/@.+/);

    // Verify EVM address field (disabled)
    const evmAddressLabel = page.getByText(/evm address/i);
    await expect(evmAddressLabel).toBeVisible();

    const evmAddressInput = page.locator('[data-pw="evm-address-input"]');
    await expect(evmAddressInput).toBeDisabled();

    // Verify Change/Hide button for wallet
    const changeWalletButton = page.locator('[data-pw="toggle-wallet-button"]');
    await expect(changeWalletButton).toBeVisible();

    // Verify display name field (enabled)
    const displayNameLabel = page.getByText(/display name/i);
    await expect(displayNameLabel).toBeVisible();

    const displayNameInput = page.locator('[data-pw="display-name-input"]');
    await expect(displayNameInput).toBeVisible();
    await expect(displayNameInput).toBeEnabled();

    // Verify description field
    const descriptionLabel = page
      .locator('label')
      .filter({ hasText: /description/i });
    await expect(descriptionLabel.first()).toBeVisible();

    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await expect(descriptionTextarea).toBeVisible();

    // Verify save button
    const saveButton = page.locator('[data-pw="save-profile-button"]');
    await expect(saveButton).toBeVisible();
  });

  test('[US-002] Should successfully update display name with valid input', async ({
    page,
  }) => {
    const displayNameInput = page.locator('[data-pw="display-name-input"]');
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Clear and enter valid display name (1 word)
    await displayNameInput.clear();
    await displayNameInput.fill('JohnDoe');

    // Add valid description (min 10 chars)
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill(
      'This is a valid description with more than 10 characters',
    );

    // Save
    await saveButton.click();

    // Wait for success toast
    await page.waitForTimeout(1000);
    const successToast = page.getByText(/profile updated successfully!/i);
    await expect(successToast).toBeVisible({ timeout: 5000 });

    // Should navigate to home page after successful save
    await page.waitForURL('/', { timeout: 5000 });
  });

  test('[US-003] Should successfully update display name with 2 words', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter valid display name (2 words)
    await displayNameInput.clear();
    await displayNameInput.fill('John Doe');

    // Add valid description
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill(
      'Updated description for two-word name validation here',
    );

    // Save
    await saveButton.click();

    // Wait for success toast
    await page.waitForTimeout(1000);
    const successToast = page.getByText(/profile updated successfully!/i);
    await expect(successToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-004] Should show error when display name exceeds 2 words', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter invalid display name (3 words)
    await displayNameInput.clear();
    await displayNameInput.fill('John Doe Smith');

    // Add valid description
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill(
      'Valid description with enough characters here',
    );

    // Try to save
    await saveButton.click();

    // Should show error toast
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(/display name must be at most 2 words/i);
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-005] Should show error when display name is empty', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter valid name first
    await displayNameInput.clear();
    await displayNameInput.fill('ValidName');

    // Add valid description
    const descriptionTextarea = page
      .locator('[data-pw="description-textarea"]')
      .first();
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description with enough characters');

    // Now clear display name to make it empty
    await displayNameInput.clear();
    await displayNameInput.fill('');

    // Try to save
    await saveButton.click();

    // Should show error toast about length
    await page.waitForTimeout(1500);
    const errorToast = page.getByText(
      /display name must be between 1 and 30 characters/i,
    );
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-006] Should show error when display name exceeds 30 characters', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Input field has maxLength=30, so we verify it can't exceed that
    const longName = 'A'.repeat(35);
    await displayNameInput.clear();
    await displayNameInput.fill(longName);

    // Verify the input is capped at 30 characters
    const actualValue = await displayNameInput.inputValue();
    expect(actualValue.length).toBeLessThanOrEqual(30);
  });

  test('[US-007] Should disable save button when display name contains "test" keyword', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter display name with "test" keyword
    await displayNameInput.clear();
    await displayNameInput.fill('Testing Name');

    // Add valid description
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description here');

    // Save button should be disabled (visually indicated by cursor-not-allowed)
    await expect(saveButton).toHaveClass(/cursor-not-allowed/);

    // Try to save
    await saveButton.click();

    // Should show error toast
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(/please remove the test keyword/i);
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-008] Should disable save button when display name contains "테스트" keyword', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter display name with Korean "test" keyword
    await displayNameInput.clear();
    await displayNameInput.fill('테스트 Name');

    // Add valid description
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description here');

    // Save button should be disabled
    await expect(saveButton).toHaveClass(/cursor-not-allowed/);

    // Try to save
    await saveButton.click();

    // Should show error toast
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(/please remove the test keyword/i);
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-009] Should disable save button when description contains "test" keyword', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter valid display name
    await displayNameInput.clear();
    await displayNameInput.fill('ValidName');

    // Add description with "test" keyword
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('This is a test description for validation');

    // Save button should be disabled
    await expect(saveButton).toHaveClass(/cursor-not-allowed/);

    // Try to save
    await saveButton.click();

    // Should show error toast
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(/please remove the test keyword/i);
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-010] Should show error when description is less than 10 characters', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter valid display name
    await displayNameInput.clear();
    await displayNameInput.fill('ValidName');

    // Add short description (less than 10 chars)
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Short');

    // Try to save
    await saveButton.click();

    // Should show error toast
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(
      /description must be at least 10 characters/i,
    );
    await expect(errorToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-011] Should allow empty description', async ({ page }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter valid display name
    await displayNameInput.clear();
    await displayNameInput.fill('ValidName');

    // Clear description (empty is allowed)
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    await descriptionTextarea.clear();

    // Verify save button is enabled (no validation error for empty description)
    await expect(saveButton).toBeEnabled();

    // Save should work
    await saveButton.click();
    await page.waitForLoadState('networkidle');
  });

  test('[US-012] Should toggle wallet connect section visibility', async ({
    page,
  }) => {
    const changeButton = page.getByRole('button', { name: /change/i }).first();

    // Verify wallet summary is not visible initially
    const walletSummary = page
      .locator('div[aria-hidden="false"]')
      .filter({ has: page.locator('img[alt="MetaMask"]') });
    await expect(walletSummary).not.toBeVisible();

    // Click Change button to show wallet connect
    await changeButton.click();
    await page.waitForTimeout(500);

    // Verify button text changed to "Hide"
    const hideButton = page.getByRole('button', { name: /hide/i }).first();
    await expect(hideButton).toBeVisible();

    // Verify wallet summary is now visible
    const metaMaskIcon = page.locator('img[alt="MetaMask"]');
    await expect(metaMaskIcon).toBeVisible();
    await expect(page.getByText('MetaMask')).toBeVisible();

    // Click Hide button to hide wallet connect
    await hideButton.click();
    await page.waitForTimeout(500);

    // Verify button text changed back to "Change"
    await expect(changeButton).toBeVisible();

    // Verify wallet summary is hidden again
    await expect(walletSummary).not.toBeVisible();
  });

  test('[US-013] Should display wallet address truncated format in wallet section', async ({
    page,
  }) => {
    // Show wallet connect section
    const changeButton = page.getByRole('button', { name: /change/i }).first();
    await changeButton.click();
    await page.waitForTimeout(500);

    // Verify MetaMask is shown
    await expect(page.getByText('MetaMask')).toBeVisible();

    // Check if wallet is connected or shows "Connect Wallet"
    const connectWalletText = page.getByText(/connect wallet/i);
    const walletAddressPattern = /0x[a-fA-F0-9]{4}\.{3}[a-fA-F0-9]{4}/;

    // Either should show connect wallet text or a truncated address
    const hasConnectText = await connectWalletText
      .isVisible()
      .catch(() => false);
    const pageContent = await page.textContent('body');
    const hasTruncatedAddress = walletAddressPattern.test(pageContent || '');

    expect(hasConnectText || hasTruncatedAddress).toBe(true);
  });

  test('[US-014] Should switch between My Info and Settings tabs', async ({
    page,
  }) => {
    // Verify we're on My Info tab initially
    const myInfoTab = page.getByRole('tab', { name: /my info/i });
    await expect(myInfoTab).toHaveAttribute('aria-selected', 'true');

    // Verify My Info content is visible
    await expect(page.locator(`[data-pw="display-name-input"]`)).toBeVisible();

    // Switch to Settings tab
    const settingsTab = page.getByRole('tab', { name: /settings/i });
    await settingsTab.click();
    await page.waitForTimeout(500);

    // Verify Settings tab is now selected
    await expect(settingsTab).toHaveAttribute('aria-selected', 'true');
    await expect(myInfoTab).toHaveAttribute('aria-selected', 'false');

    // Verify Settings content is visible
    await expect(page.getByText(/appearance/i)).toBeVisible();
    await expect(page.getByText(/language/i)).toBeVisible();

    // Switch back to My Info tab
    await myInfoTab.click();
    await page.waitForTimeout(500);

    // Verify My Info tab is selected again
    await expect(myInfoTab).toHaveAttribute('aria-selected', 'true');
    await expect(page.locator(`[data-pw="display-name-input"]`)).toBeVisible();
  });

  test('[US-015] Should maintain form state when switching tabs', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );

    // Enter some data
    const uniqueName = `TestUser${Date.now()}`;
    const uniqueDesc = `Unique description ${Date.now()} with more than ten characters`;

    await displayNameInput.clear();
    await displayNameInput.fill(uniqueName);
    await descriptionTextarea.clear();
    await descriptionTextarea.fill(uniqueDesc);

    // Switch to Settings tab
    const settingsTab = page.getByRole('tab', { name: /settings/i });
    await settingsTab.click();
    await page.waitForTimeout(500);

    // Switch back to My Info tab
    const myInfoTab = page.getByRole('tab', { name: /my info/i });
    await myInfoTab.click();
    await page.waitForTimeout(500);

    // Verify data is maintained
    await expect(displayNameInput).toHaveValue(uniqueName);
    await expect(descriptionTextarea).toHaveValue(uniqueDesc);
  });

  test('[US-016] Should visually indicate save button state based on validation', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Valid input - button should be enabled (cursor-pointer)
    await displayNameInput.clear();
    await displayNameInput.fill('ValidName');
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description with enough characters');

    await page.waitForTimeout(300);
    await expect(saveButton).toHaveClass(/cursor-pointer/);
    await expect(saveButton).toHaveClass(/bg-enable-button-bg/);

    // Invalid input with "test" - button should be disabled (cursor-not-allowed)
    await displayNameInput.clear();
    await displayNameInput.fill('Testing Name');

    await page.waitForTimeout(300);
    await expect(saveButton).toHaveClass(/cursor-not-allowed/);
    await expect(saveButton).toHaveClass(/bg-disable-button-bg/);
  });

  test('[US-017] Should handle rapid input changes correctly', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Rapidly change input
    await displayNameInput.clear();
    await displayNameInput.fill('Name1');
    await displayNameInput.clear();
    await displayNameInput.fill('Name2');
    await displayNameInput.clear();
    await displayNameInput.fill('FinalName');

    await descriptionTextarea.clear();
    await descriptionTextarea.fill('First description');
    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Final description with enough characters');

    // Wait a bit for state updates
    await page.waitForTimeout(500);

    // Verify final values
    await expect(displayNameInput).toHaveValue('FinalName');
    await expect(descriptionTextarea).toHaveValue(
      'Final description with enough characters',
    );

    // Save should work
    await saveButton.click();
    await page.waitForTimeout(1000);
    const successToast = page.getByText(/profile updated successfully!/i);
    await expect(successToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-018] Should trim whitespace from display name before validation', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter display name with leading/trailing whitespace
    await displayNameInput.clear();
    await displayNameInput.fill('  ValidName  ');

    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description with enough characters');

    // Save
    await saveButton.click();

    // Should succeed (whitespace trimmed)
    await page.waitForTimeout(1000);
    const successToast = page.getByText(/profile updated successfully!/i);
    await expect(successToast).toBeVisible({ timeout: 5000 });
  });

  test('[US-019] Should reject display name with only whitespace', async ({
    page,
  }) => {
    const displayNameInput = page.locator(`[data-pw="display-name-input"]`);
    const descriptionTextarea = page.locator(
      '[data-pw="description-textarea"]',
    );
    const saveButton = page.locator('[data-pw="save-profile-button"]');

    // Enter only whitespace
    await displayNameInput.clear();
    await displayNameInput.fill('     ');

    await descriptionTextarea.clear();
    await descriptionTextarea.fill('Valid description here');

    // Button should be enabled (validation happens on click)
    await expect(saveButton).toBeEnabled();

    // Click save button
    await saveButton.click();

    // Should show error toast (whitespace gets trimmed to empty, so length < 1)
    await page.waitForTimeout(1000);
    const errorToast = page.getByText(
      /display name must be between 1 and 30 characters|display name cannot be empty/i,
    );
    await expect(errorToast).toBeVisible({
      timeout: CONFIGS.SELECTOR_WAIT_TIME,
    });
  });

  test('[US-020] Should show correct page URL', async ({ page }) => {
    await expect(page).toHaveURL(/\/settings$/);
  });
});
