// FIXME: fix testcode workflow
// import { test, expect } from '@playwright/test';

// test.describe('[SpacePollEditorPage] Anonymous Users', () => {
//   test('[SPEP-ANON-001] Anonymous user can access poll editor page but cannot edit', async ({
//     page,
//   }) => {
//     // Anonymous users can access the poll editor page
//     // Using a plausible space and poll ID format
//     const testSpacePk = 'SPACE#test-space';
//     const testPollSk = 'POLL#test-poll';
//     const pollEditorUrl = `/spaces/${encodeURIComponent(testSpacePk)}/polls/${encodeURIComponent(testPollSk)}/edit`;

//     await page.goto(pollEditorUrl);
//     await page.waitForLoadState('networkidle');

//     // Verify the page loads (may show 404 or error for non-existent poll)
//     // The page itself is accessible, but editing features should be restricted
//     const currentUrl = page.url();
//     expect(currentUrl).toContain('/edit');
//   });

//   test('[SPEP-ANON-002] Anonymous user cannot see response_editable checkbox', async ({
//     page,
//   }) => {
//     // Try to access a poll editor page
//     const testSpacePk = 'SPACE#test-space';
//     const testPollSk = 'POLL#test-poll';
//     const pollEditorUrl = `/spaces/${encodeURIComponent(testSpacePk)}/polls/${encodeURIComponent(testPollSk)}/edit`;

//     await page.goto(pollEditorUrl);
//     await page.waitForLoadState('networkidle');

//     // If somehow the page loads (unlikely), verify the checkbox is not visible
//     const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
//     const isVisible = await checkbox.isVisible({ timeout: 2000 }).catch(() => false);

//     expect(isVisible).toBe(false);
//   });

//   test('[SPEP-ANON-003] Anonymous user can access poll editor URLs but sees no admin features', async ({
//     page,
//   }) => {
//     // Anonymous users can access poll editor URLs
//     const testUrl = '/spaces/SPACE%23test/polls/POLL%23test/edit';

//     await page.goto(testUrl);
//     await page.waitForLoadState('networkidle');

//     const currentUrl = page.url();

//     // Page loads successfully
//     expect(currentUrl).toContain('/edit');

//     // Verify admin-only checkbox is not visible
//     const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
//     const isVisible = await checkbox.isVisible({ timeout: 2000 }).catch(() => false);
//     expect(isVisible).toBe(false);
//   });

//   test('[SPEP-ANON-004] Anonymous user cannot see Edit button', async ({
//     page,
//   }) => {
//     // Navigate to a poll viewer page (not editor) as anonymous user
//     const testSpacePk = 'SPACE#test-space';
//     const testPollSk = 'POLL#test-poll';
//     const pollViewerUrl = `/spaces/${encodeURIComponent(testSpacePk)}/polls/${encodeURIComponent(testPollSk)}`;

//     await page.goto(pollViewerUrl);
//     await page.waitForLoadState('networkidle');

//     // Even if the viewer page loads, Edit button should not be visible
//     const editButton = page.getByRole('button', { name: 'Edit' });
//     const isEditVisible = await editButton.isVisible({ timeout: 2000 }).catch(() => false);

//     expect(isEditVisible).toBe(false);
//   });

//   test('[SPEP-ANON-005] Anonymous user can view poll editor page but lacks editing permissions', async ({
//     page,
//   }) => {
//     // Access the poll editor URL
//     const pollEditorUrl = '/spaces/test-space/polls/test-poll/edit';

//     await page.goto(pollEditorUrl);
//     await page.waitForLoadState('networkidle');

//     // Page is accessible
//     expect(page.url()).toContain(pollEditorUrl);

//     // Verify response_editable checkbox (admin-only) is not visible
//     const checkbox = page.locator('[data-pw="response-editable-checkbox"]');
//     const isCheckboxVisible = await checkbox.isVisible({ timeout: 2000 }).catch(() => false);
//     expect(isCheckboxVisible).toBe(false);
//   });

//   test('[SPEP-ANON-006] Anonymous user cannot access admin-only poll editing features', async ({
//     page,
//   }) => {
//     // Try to access the poll editor
//     const pollEditorUrl = '/spaces/test-space/polls/test-poll/edit';

//     await page.goto(pollEditorUrl);
//     await page.waitForLoadState('networkidle');

//     // Verify admin-only elements are not present
//     const adminElements = [
//       '[data-pw="response-editable-checkbox"]',
//       'button:has-text("Save")',
//       'button:has-text("Discard")',
//     ];

//     for (const selector of adminElements) {
//       const element = page.locator(selector);
//       const isVisible = await element.isVisible({ timeout: 1000 }).catch(() => false);
//       expect(isVisible).toBe(false);
//     }
//   });

//   test('[SPEP-ANON-007] Anonymous user cannot see admin UI elements', async ({
//     page,
//   }) => {
//     // Try to access poll editor
//     const pollEditorUrl = '/spaces/test-space/polls/test-poll/edit';

//     await page.goto(pollEditorUrl);
//     await page.waitForLoadState('networkidle');

//     // Page loads successfully
//     expect(page.url()).toContain('/edit');

//     // Verify that the response_editable checkbox label is also not visible
//     const labelText = 'Allow users to edit their responses';
//     const label = page.getByText(labelText);
//     const isLabelVisible = await label.isVisible({ timeout: 2000 }).catch(() => false);

//     // Label should not be visible for anonymous users (admin-only feature)
//     expect(isLabelVisible).toBe(false);
//   });
// });
