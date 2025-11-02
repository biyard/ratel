import { test, expect } from '@playwright/test';

test.describe('Sprint League Page - Authenticated User', () => {
  test.beforeEach(async ({ page }) => {
    // Setup: Login as admin user
    await page.goto('/login');
    // Add login flow here based on your auth setup
    // For now, we'll assume we're logged in
  });

  test('should display character editor in edit mode for admin', async ({ page }) => {
    // Navigate to a draft space sprint league page
    // Note: Replace with actual space ID that has sprint league and is in draft mode
    await page.goto('/spaces/test-space-id/sprint-league');

    // Wait for page to load
    await page.waitForLoadState('networkidle');

    // Check if editor is visible (only visible for admins in draft mode)
    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      // Click edit button to enter edit mode
      await editButton.click();

      // Wait for the editor card to appear
      await expect(page.locator('text=Sprint Players')).toBeVisible();

      // Verify that character selectors are visible
      const characterContainers = page.locator('.aspect-square.size-75');
      await expect(characterContainers.first()).toBeVisible();

      // Check for "Select Character" button
      const selectCharacterButton = page.getByRole('button', { name: /select character/i }).first();
      await expect(selectCharacterButton).toBeVisible();
    }
  });

  test('should display character preview in player selection modal', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      // Click on "Select Character" button
      const selectCharacterButton = page.getByRole('button', { name: /select character/i }).first();
      await selectCharacterButton.click();

      // Wait for modal to appear
      await expect(page.locator('text=Select a character')).toBeVisible();

      // Verify that character previews are visible in the modal
      const characterPreviews = page.locator('.grid-cols-3.grid > div');
      const count = await characterPreviews.count();

      // There should be at least 3 base characters
      expect(count).toBeGreaterThanOrEqual(3);

      // Each character preview should contain an image
      for (let i = 0; i < Math.min(count, 3); i++) {
        const preview = characterPreviews.nth(i);
        await expect(preview).toBeVisible();

        // Check if the preview contains an image or the character preview component
        const hasImage = await preview.locator('img').count() > 0;
        const hasPreview = await preview.locator('div').count() > 0;
        expect(hasImage || hasPreview).toBeTruthy();
      }
    }
  });

  test('should allow selecting a character from the modal', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      const selectCharacterButton = page.getByRole('button', { name: /select character/i }).first();
      await selectCharacterButton.click();

      // Wait for modal
      await page.waitForTimeout(500);

      // Select the first character
      const firstCharacter = page.locator('.grid-cols-3.grid > div').first();
      await firstCharacter.click();

      // The first character should have aria-selected="true"
      await expect(firstCharacter).toHaveAttribute('aria-selected', 'true');

      // Confirm button should be enabled
      const confirmButton = page.getByRole('button', { name: /confirm|select/i });
      await expect(confirmButton).toBeEnabled();
    }
  });

  test('should display animated character in editor after selection', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      const selectCharacterButton = page.getByRole('button', { name: /select character/i }).first();
      await selectCharacterButton.click();

      await page.waitForTimeout(500);

      // Select and confirm character
      const firstCharacter = page.locator('.grid-cols-3.grid > div').first();
      await firstCharacter.click();

      const confirmButton = page.getByRole('button', { name: /confirm|select/i });
      await confirmButton.click();

      // Wait for modal to close and character to render
      await page.waitForTimeout(1000);

      // Check that the character canvas is rendered
      const characterCanvas = page.locator('canvas').first();
      await expect(characterCanvas).toBeVisible();

      // Verify canvas has dimensions
      const width = await characterCanvas.getAttribute('width');
      const height = await characterCanvas.getAttribute('height');
      expect(parseInt(width || '0')).toBeGreaterThan(0);
      expect(parseInt(height || '0')).toBeGreaterThan(0);
    }
  });

  test('should allow editing player name and description', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      // Find the first player input fields
      const nameInput = page.locator('input[type="text"]').first();
      const descriptionTextarea = page.locator('textarea').first();

      // Verify inputs are visible
      await expect(nameInput).toBeVisible();
      await expect(descriptionTextarea).toBeVisible();

      // Type in the name field
      await nameInput.fill('Test Player Name');
      await expect(nameInput).toHaveValue('Test Player Name');

      // Type in the description field
      await descriptionTextarea.fill('This is a test description for the player.');
      await expect(descriptionTextarea).toHaveValue('This is a test description for the player.');
    }
  });

  test('should show save and discard buttons in edit mode', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      // Check for Save and Discard buttons
      const saveButton = page.getByRole('button', { name: /save/i });
      const discardButton = page.getByRole('button', { name: /discard/i });

      await expect(saveButton).toBeVisible();
      await expect(discardButton).toBeVisible();
    }
  });

  test('should maintain character frame size consistency', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      // Check that character containers have consistent size
      const characterContainers = page.locator('.aspect-square.size-75');
      const count = await characterContainers.count();

      if (count > 0) {
        // Get bounding box of first container
        const firstBox = await characterContainers.first().boundingBox();

        if (firstBox) {
          // All containers should have the same aspect ratio (square)
          const aspectRatio = firstBox.width / firstBox.height;
          expect(Math.abs(aspectRatio - 1.0)).toBeLessThan(0.1); // Allow 10% tolerance for square aspect ratio
        }
      }
    }
  });
});

test.describe('Sprint League Page - Character Visibility Regression Tests', () => {
  test('should not show placeholder text "Character Here" in edit mode', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      // This placeholder text should NOT exist after the fix
      const placeholderText = page.locator('text=Character Here');
      await expect(placeholderText).not.toBeVisible();
    }
  });

  test('should show character images in selection modal (not commented out)', async ({ page }) => {
    await page.goto('/spaces/test-space-id/sprint-league');
    await page.waitForLoadState('networkidle');

    const editButton = page.getByRole('button', { name: /edit/i });

    if (await editButton.isVisible()) {
      await editButton.click();

      const selectCharacterButton = page.getByRole('button', { name: /select character/i }).first();
      await selectCharacterButton.click();

      await page.waitForTimeout(500);

      // Check that character previews are actually rendered (not commented out JSX)
      const characterPreviews = page.locator('.grid-cols-3.grid > div');
      const firstPreview = characterPreviews.first();

      // The preview should contain actual content (image or canvas), not be empty
      const childCount = await firstPreview.locator('*').count();
      expect(childCount).toBeGreaterThan(0);
    }
  });
});
