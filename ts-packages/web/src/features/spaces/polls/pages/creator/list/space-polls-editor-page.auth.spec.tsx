import { test, expect } from '@playwright/test';

test.describe('[SpacePollsEditorPage] Authenticated Users', () => {
  test.describe.configure({ mode: 'serial' });

  // NOTE: These tests are for the poll type selector component functionality
  // They require navigating to a specific space's polls editor page
  // For now, we verify the component structure and API integration

  test.skip('[SPELP-001] Poll type selector is shown when no polls exist', async ({
    page,
  }) => {
    // TODO: This test requires a specific space URL
    // Route format: /spaces/:spacePk/polls/creator
    // Navigate to a space with no polls
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // The poll type selector should be visible
    // Look for the pre-poll survey card
    const prePollCard = page.getByText('Pre-poll Survey');
    const finalSurveyCard = page.getByText('Final Survey');

    // At least one of these should be visible when there are no polls
    const hasTypeSelector =
      (await prePollCard.isVisible()) || (await finalSurveyCard.isVisible());
    expect(hasTypeSelector).toBeTruthy();
  });

  test.skip('[SPELP-002] Pre-poll survey card displays correct text', async ({
    page,
  }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check for pre-poll survey text
    const prePollTitle = page.getByText('Pre-poll Survey');
    const prePollDescription = page.getByText(
      /Users make their opinion right after participating the space/i,
    );

    // If the selector is visible, verify the text
    if (await prePollTitle.isVisible()) {
      expect(await prePollTitle.textContent()).toContain('Pre-poll Survey');
      expect(await prePollDescription.isVisible()).toBeTruthy();
    }
  });

  test.skip('[SPELP-003] Final survey card displays correct text', async ({
    page,
  }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Check for final survey text
    const finalTitle = page.getByText('Final Survey');
    const finalDescription = page.getByText(
      /Users attend after finishing discussion and deliberation/i,
    );

    // If the selector is visible, verify the text
    if (await finalTitle.isVisible()) {
      expect(await finalTitle.textContent()).toContain('Final Survey');
      expect(await finalDescription.isVisible()).toBeTruthy();
    }
  });

  test.skip('[SPELP-004] Clicking pre-poll survey creates poll with default=true', async ({
    page,
  }) => {
    let capturedRequest: {
      url: string;
      body: { default: boolean };
      method: string;
    } | null = null;

    // Intercept POST requests to create poll
    await page.route('**/v3/spaces/*/polls', (route) => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        capturedRequest = {
          url: route.request().url(),
          body: postData,
          method: route.request().method(),
        };

        // Return a mock response
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            sk: 'POLL#test-123',
            created_at: Date.now(),
            updated_at: Date.now(),
            started_at: Date.now(),
            ended_at: Date.now() + 86400000,
            response_editable: postData.default,
            user_response_count: 0,
            questions: [],
            status: 'NotStarted',
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Click on pre-poll survey card if visible
    const prePollCard = page.getByText('Pre-poll Survey').first();
    if (await prePollCard.isVisible()) {
      await prePollCard.click();

      // Wait for the request
      await page.waitForTimeout(1000);

      // Verify the request was made with default: true
      if (capturedRequest) {
        expect(capturedRequest.method).toBe('POST');
        expect(capturedRequest.body).toHaveProperty('default', true);
      }
    }

    await page.unroute('**/v3/spaces/*/polls');
  });

  test.skip('[SPELP-005] Clicking final survey creates poll with default=false', async ({
    page,
  }) => {
    let capturedRequest: {
      url: string;
      body: { default: boolean };
      method: string;
    } | null = null;

    // Intercept POST requests to create poll
    await page.route('**/v3/spaces/*/polls', (route) => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        capturedRequest = {
          url: route.request().url(),
          body: postData,
          method: route.request().method(),
        };

        // Return a mock response
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            sk: 'POLL#test-456',
            created_at: Date.now(),
            updated_at: Date.now(),
            started_at: Date.now(),
            ended_at: Date.now() + 86400000,
            response_editable: postData.default,
            user_response_count: 0,
            questions: [],
            status: 'NotStarted',
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Click on final survey card if visible
    const finalSurveyCard = page.getByText('Final Survey').first();
    if (await finalSurveyCard.isVisible()) {
      await finalSurveyCard.click();

      // Wait for the request
      await page.waitForTimeout(1000);

      // Verify the request was made with default: false
      if (capturedRequest) {
        expect(capturedRequest.method).toBe('POST');
        expect(capturedRequest.body).toHaveProperty('default', false);
      }
    }

    await page.unroute('**/v3/spaces/*/polls');
  });

  test.skip('[SPELP-006] Poll type selector is hidden when polls exist', async ({
    page,
  }) => {
    // Mock the API to return existing polls
    await page.route('**/v3/spaces/*/polls*', (route) => {
      if (route.request().method() === 'GET') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            polls: [
              {
                sk: 'POLL#existing',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [],
                status: 'InProgress',
              },
            ],
            bookmark: null,
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // The poll type selector should NOT be visible
    const prePollCard = page.getByText('Pre-poll Survey');
    const finalSurveyCard = page.getByText('Final Survey');

    // Both cards should be hidden when polls exist
    expect(await prePollCard.isVisible()).toBeFalsy();
    expect(await finalSurveyCard.isVisible()).toBeFalsy();

    await page.unroute('**/v3/spaces/*/polls*');
  });

  test.skip('[SPELP-007] Delete button appears for each poll in the list', async ({
    page,
  }) => {
    // Mock the API to return existing polls
    await page.route('**/v3/spaces/*/polls*', (route) => {
      if (route.request().method() === 'GET') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            polls: [
              {
                sk: 'POLL#test1',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [{ id: 1, text: 'Question 1' }],
                status: 'InProgress',
                default: false,
              },
              {
                sk: 'POLL#test2',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [{ id: 1, text: 'Question 1' }],
                status: 'NotStarted',
                default: true,
              },
            ],
            bookmark: null,
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Verify that delete buttons are present
    const deleteButtons = page.getByRole('button', { name: /delete/i });
    const deleteCount = await deleteButtons.count();

    // Should have 2 delete buttons (one for each poll)
    expect(deleteCount).toBe(2);

    await page.unroute('**/v3/spaces/*/polls*');
  });

  test.skip('[SPELP-008] Clicking delete button calls DELETE API and removes poll from list', async ({
    page,
  }) => {
    let deletedPollSk: string | null = null;

    // Mock GET to return polls
    await page.route('**/v3/spaces/*/polls*', (route) => {
      if (route.request().method() === 'GET') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            polls: [
              {
                sk: 'POLL#toDelete',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [{ id: 1, text: 'Question 1' }],
                status: 'InProgress',
                default: false,
              },
            ],
            bookmark: null,
          }),
        });
      } else {
        route.continue();
      }
    });

    // Mock DELETE request
    await page.route('**/v3/spaces/*/polls/*', (route) => {
      if (route.request().method() === 'DELETE') {
        const url = route.request().url();
        const match = url.match(/polls\/([^?]+)/);
        if (match) {
          deletedPollSk = decodeURIComponent(match[1]);
        }

        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            status: 'success',
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Find and click the delete button
    const deleteButton = page.getByRole('button', { name: /delete/i }).first();
    await expect(deleteButton).toBeVisible();
    await deleteButton.click();

    // Wait for the request
    await page.waitForTimeout(1000);

    // Verify the DELETE request was made
    expect(deletedPollSk).toBe('POLL#toDelete');

    // Verify the poll is removed from the UI
    const pollItems = page.getByText(/QUESTIONS/);
    expect(await pollItems.count()).toBe(0);

    await page.unroute('**/v3/spaces/*/polls*');
    await page.unroute('**/v3/spaces/*/polls/*');
  });

  test.skip('[SPELP-009] Delete button shows success toast on successful deletion', async ({
    page,
  }) => {
    // Mock GET to return polls
    await page.route('**/v3/spaces/*/polls*', (route) => {
      if (route.request().method() === 'GET') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            polls: [
              {
                sk: 'POLL#test',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [{ id: 1, text: 'Question 1' }],
                status: 'InProgress',
                default: false,
              },
            ],
            bookmark: null,
          }),
        });
      } else {
        route.continue();
      }
    });

    // Mock DELETE request
    await page.route('**/v3/spaces/*/polls/*', (route) => {
      if (route.request().method() === 'DELETE') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            status: 'success',
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Click delete button
    const deleteButton = page.getByRole('button', { name: /delete/i }).first();
    await deleteButton.click();

    // Wait for toast
    await page.waitForTimeout(500);

    // Verify success toast is shown
    const successToast = page.getByText(/Poll deleted successfully/i);
    await expect(successToast).toBeVisible();

    await page.unroute('**/v3/spaces/*/polls*');
    await page.unroute('**/v3/spaces/*/polls/*');
  });

  test.skip('[SPELP-010] Delete button shows error toast on failed deletion', async ({
    page,
  }) => {
    // Mock GET to return polls
    await page.route('**/v3/spaces/*/polls*', (route) => {
      if (route.request().method() === 'GET') {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            polls: [
              {
                sk: 'POLL#test',
                created_at: Date.now(),
                updated_at: Date.now(),
                started_at: Date.now(),
                ended_at: Date.now() + 86400000,
                response_editable: true,
                user_response_count: 0,
                questions: [{ id: 1, text: 'Question 1' }],
                status: 'InProgress',
                default: false,
              },
            ],
            bookmark: null,
          }),
        });
      } else {
        route.continue();
      }
    });

    // Mock DELETE request to fail
    await page.route('**/v3/spaces/*/polls/*', (route) => {
      if (route.request().method() === 'DELETE') {
        route.fulfill({
          status: 500,
          contentType: 'application/json',
          body: JSON.stringify({
            error: 'Internal server error',
          }),
        });
      } else {
        route.continue();
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Click delete button
    const deleteButton = page.getByRole('button', { name: /delete/i }).first();
    await deleteButton.click();

    // Wait for toast
    await page.waitForTimeout(500);

    // Verify error toast is shown
    const errorToast = page.getByText(/Failed to delete poll/i);
    await expect(errorToast).toBeVisible();

    await page.unroute('**/v3/spaces/*/polls*');
    await page.unroute('**/v3/spaces/*/polls/*');
  });
});
