import { Page } from '@playwright/test';
import { TIMEOUT } from './auth';

export async function verifyCredential(page: Page, code: string) {
  await page.goto('/credentials');
  await page.waitForLoadState('networkidle');

  // Click the verify button to open method selection modal
  const verifyButton = page.getByTestId('credential-verify-button');
  await verifyButton.waitFor({ state: 'visible', timeout: TIMEOUT });
  await verifyButton.click();

  // Select code verification option
  const codeVerificationOption = page.getByTestId('code-verification-option');
  await codeVerificationOption.waitFor({ state: 'visible', timeout: TIMEOUT });
  await codeVerificationOption.click();

  // Enter the code
  const codeInput = page.getByTestId('credential-code-input');
  await codeInput.waitFor({ state: 'visible', timeout: TIMEOUT });
  await codeInput.fill(code);

  // Submit the code
  const submitButton = page.getByTestId('credential-code-submit');
  await submitButton.click();

  // Wait for verification to complete
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(500);
}
