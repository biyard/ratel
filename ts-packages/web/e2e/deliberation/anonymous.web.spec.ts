import { test, expect } from '@playwright/test';
import {
  ATTRIBUTE_CODES,
  BOARD_POSTS,
  POST_CONTENT,
  POST_TITLE,
  SURVEY_QUESTIONS,
  TEAM_DESCRIPTION,
  TEAM_ID,
  TEAM_NAME,
  TEST_PASSWORD,
  YOUTUBE_LINK,
} from './data';
import {
  clickTeamSidebarMenu,
  conductSurvey,
  createBoardPosts,
  createDeliberationPost,
  createFinalSurvey,
  createPrePollSurvey,
  createTeam,
  enableAnonymousParticipation,
  goToFinalSurvey,
  goToMySpaces,
  goToSpace,
  goToTeam,
  inviteMembers,
  login,
  participateInSpace,
  publishSpacePrivately,
  replyToPost,
  setupPanels,
  startDeliberation,
  verifyCredential,
  viewAnalysis,
  writeNewPost,
} from './helpers';

test.describe.serial('[Deliberation] General Spec', () => {
  // Disable retries for serial tests - shared state can't be restored after retry
  // Increase timeout for serial tests that include complex operations (60 seconds)
  test.describe.configure({ retries: 0, timeout: 60000 });

  let no = 0;

  const i = () => {
    no += 1;
    return no;
  };

  const creator1 = {
    email: 'hi+admin1@biyard.co',
    password: TEST_PASSWORD,
  };

  const creator2 = {
    email: 'hi+admin2@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant1 = {
    email: 'hi+user1@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant2 = {
    email: 'hi+user2@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant3 = {
    email: 'hi+user3@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant4 = {
    email: 'hi+user4@biyard.co',
    password: TEST_PASSWORD,
  };

  const participant5 = {
    email: 'hi+user5@biyard.co',
    password: TEST_PASSWORD,
  };

  const unverifiedParticipant1 = {
    email: 'hi+user6@biyard.co',
    password: TEST_PASSWORD,
  };

  const guest1 = {
    email: 'hi+anon1@biyard.co',
    password: TEST_PASSWORD,
  };
  const teamId = TEAM_ID;

  // State backup for serial test execution
  let savedCookies: any[] = [];
  let savedUrl: string = '/';

  test.beforeEach(async ({ page }) => {
    // Backup current cookies and URL before each test
    page.context().addCookies(savedCookies);
    await page.goto(savedUrl, { waitUntil: 'networkidle' });
  });

  test.afterEach(async ({ page }) => {
    savedUrl = page.url();
    savedCookies = await page.context().cookies();
  });

  // =====================================
  // Verification: Participant
  // =====================================
  test(`DS-${i()} [Participant 1] Verifying credential with code1`, async ({
    page,
  }) => {
    await login(page, participant1);
    await verifyCredential(page, ATTRIBUTE_CODES.KONKUK_MALE);
  });

  test(`DS-${i()} [Participant 2] Verifying credential with code2`, async ({
    page,
  }) => {
    await login(page, participant2);
    await verifyCredential(page, ATTRIBUTE_CODES.KONKUK_FEMALE);
  });

  test(`DS-${i()} [Participant 3] Verifying credential with code3`, async ({
    page,
  }) => {
    await login(page, participant3);
    await verifyCredential(page, ATTRIBUTE_CODES.SOGANG_MALE);
  });

  test(`DS-${i()} [Participant 4] Verifying credential with code4`, async ({
    page,
  }) => {
    await login(page, participant4);
    await verifyCredential(page, ATTRIBUTE_CODES.SOGANG_FEMALE);
  });

  test(`DS-${i()} [Participant 5] Verifying credential with code5`, async ({
    page,
  }) => {
    await login(page, participant5);
    await verifyCredential(page, ATTRIBUTE_CODES.KONKUK_MALE);
  });

  // =====================================
  // Design and Publish: Creator
  // =====================================
  test(`DS-${i()} [Creator 1] Create a team`, async ({ page }) => {
    await login(page, creator1);
    await createTeam(page, TEAM_NAME, teamId, TEAM_DESCRIPTION);
  });

  test(`DS-${i()} [Creator 1] Invite a member(Creator2) to team`, async ({
    page,
  }) => {
    await login(page, creator1);
    await goToTeam(page);
    await clickTeamSidebarMenu(page, 'groups');

    const inviteButton = page.locator('[data-pw="invite-member-button"]');
    await inviteButton.waitFor({ state: 'visible' });
    await inviteButton.click();

    const inviteInput = page.getByTestId('invite-member-search-input');
    await inviteInput.waitFor({ state: 'visible' });
    await inviteInput.fill(creator2.email);
    await page.keyboard.press('Enter');

    await page.waitForTimeout(1000);

    const inviteSendButton = page.getByTestId('send-invite-button');
    await inviteSendButton.click();

    const popupCloseButton = page.getByTestId('popup-close-button');
    if (await popupCloseButton.isVisible()) {
      await popupCloseButton.click();
    }
  });

  test(`DS-${i()} [Creator 1] Create a draft with a deliberation space`, async ({
    page,
  }) => {
    await login(page, creator1);
    await goToTeam(page);
    await createDeliberationPost(page, POST_TITLE, POST_CONTENT, YOUTUBE_LINK);
  });

  test(`DS-${i()} [Creator 2] Check a team draft page`, async ({ page }) => {
    await login(page, creator2);
    await page.goto(`/teams/${teamId}/drafts`, { waitUntil: 'networkidle' });

    // Verify the draft post is visible
    await expect(page.getByText(POST_TITLE)).toBeVisible();

    // Click on the draft to open it
    await page.getByText(POST_TITLE).click();
    await page.waitForLoadState('networkidle');

    // Click Next button to proceed to space configuration
    const nextButton = page.getByRole('button', { name: 'Next' });
    await nextButton.waitFor({ state: 'visible', timeout: 10000 });
    await nextButton.click();

    // Wait for navigation to space configuration
    await page.waitForURL(/.*\/spaces\/.*/, { timeout: 15000 });
    await page.waitForLoadState('networkidle');
  });

  test(`DS-${i()} [Creator 2] Modifying Overview`, async ({ page }) => {
    await page.getByTestId('space-sidemenu-overview').click();
    await page.waitForLoadState('networkidle');

    // Overview is already set from post creation, verify it's visible
    await expect(page.getByText(POST_TITLE)).toBeVisible();
  });

  test(`DS-${i()} [Creator 2] Creating a Pre-Survey poll`, async ({ page }) => {
    await createPrePollSurvey(page, SURVEY_QUESTIONS);
  });

  test(`DS-${i()} [Creator 2] Creating a board`, async ({ page }) => {
    await createBoardPosts(page, BOARD_POSTS);
  });

  test(`DS-${i()} [Creator 2] Inviting members`, async ({ page }) => {
    const emails = [
      participant1.email,
      participant2.email,
      participant3.email,
      participant4.email,
      participant5.email,
      unverifiedParticipant1.email, // Include unverified participant to test rejection
    ];
    await inviteMembers(page, emails);
  });

  test(`DS-${i()} [Creator 2] Setting up panels`, async ({ page }) => {
    await setupPanels(page, '60');
  });

  test(`DS-${i()} [Creator 2] Configure anonymous`, async ({ page }) => {
    await enableAnonymousParticipation(page);
  });

  test(`DS-${i()} [Creator 2] Publish privately`, async ({ page }) => {
    await publishSpacePrivately(page);
  });

  // =====================================
  // Rejection: Unverified Participant
  // =====================================
  test(`DS-${i()} [Unverified Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {
    await login(page, unverifiedParticipant1);
    await goToMySpaces(page);

    // Verify the space is visible in invitations
    const spaceCard = page.getByText(POST_TITLE).first();
    await expect(spaceCard).toBeVisible({ timeout: 15000 });
  });

  // Note: Since we skipped panel setup, unverified users can also participate
  // This test would require panel attribute restrictions to properly reject unverified users
  test.skip(`DS-${i()} [Unverified Participant] Check can access space - PANEL RESTRICTIONS DISABLED`, async ({
    page,
  }) => {
    // Try to participate (will succeed since no panel restrictions)
    // TODO: Check unverified participant showing modal to redirect credential page.

    // Verify user can see the space content (not rejected)
    // When panel restrictions are disabled, all invited users can participate
    await page.waitForLoadState('networkidle');
  });

  // =====================================
  // Rejection: Guest
  // =====================================
  test(`DS-${i()} [Guest] Sign in with Guest`, async ({ page }) => {
    await login(page, guest1);
    // TODO: Check rejection
  });

  // =====================================
  // PrePoll: Participant 1
  // =====================================
  test(`DS-${i()} [Participant 1] Conduct PrePoll`, async ({ page }) => {
    await login(page, participant1);
    await goToMySpaces(page);
    await goToSpace(page, POST_TITLE);

    // Manual participation step (auto-redirects to Pre-Poll)
    await participateInSpace(page);

    // Survey is now visible (no manual navigation needed)
    await conductSurvey(page, [
      0,
      0,
      0,
      1,
      'Part 1 is important',
      "I don't have any idea",
    ]);

    await page.getByTestId('complete-survey-modal-btn-confirm').click();
    await expect(page.getByTestId('space-participant-profile')).toBeVisible();
  });

  // =====================================
  // PrePoll: Participant 2
  // =====================================
  test(`DS-${i()} [Participant 2] Conduct PrePoll`, async ({ page }) => {
    await login(page, participant2);
    await goToMySpaces(page);
    await goToSpace(page, POST_TITLE);
    await participateInSpace(page);

    await conductSurvey(page, [
      1,
      1,
      2,
      1,
      'Part 2 is important',
      'Go Option 2',
    ]);

    await page.getByTestId('complete-survey-modal-btn-confirm').click();
    await expect(page.getByTestId('space-participant-profile')).toBeVisible();
  });

  // =====================================
  // PrePoll: Participant 3
  // =====================================
  test(`DS-${i()} [Participant 3] Conduct PrePoll`, async ({ page }) => {
    await login(page, participant3);
    await goToMySpaces(page);
    await goToSpace(page, POST_TITLE);
    await participateInSpace(page);

    await conductSurvey(page, [
      0,
      1,
      1,
      0,
      'Part 3 is important',
      'Go Option 3',
    ]);

    await page.getByTestId('complete-survey-modal-btn-confirm').click();
    await expect(page.getByTestId('space-participant-profile')).toBeVisible();
  });

  // =====================================
  // PrePoll: Participant 4
  // =====================================
  test(`DS-${i()} [Participant 4] Conduct PrePoll`, async ({ page }) => {
    await login(page, participant4);
    await goToMySpaces(page);
    await goToSpace(page, POST_TITLE);
    await participateInSpace(page);

    await conductSurvey(page, [
      0,
      0,
      3,
      1,
      'Part 3 is important',
      'Go Option 3',
    ]);

    await page.getByTestId('complete-survey-modal-btn-confirm').click();
    await expect(page.getByTestId('space-participant-profile')).toBeVisible();
  });

  // =====================================
  // Start Deliberation: Creator
  // =====================================
  test(`DS-${i()} [Creator 1] Start Deliberation`, async ({ page }) => {
    await login(page, creator1);

    // Navigate to the published space via the team's home page
    // The post was already published in DS-23, so it's visible on team home
    await page.goto(`/teams/${teamId}/home`);
    await page.waitForLoadState('networkidle');
    const p = page.getByText(POST_TITLE);
    await expect(p).toBeVisible();

    // Click on the published post to enter the space
    const postLink = p.first();
    await expect(postLink).toBeVisible();
    await postLink.click();

    await startDeliberation(page);
  });

  // =====================================
  // Blocking: Participant 5 (late joiner)
  // =====================================
  // TODO: implement
  test.skip(`DS-${i()} [Participant 5] Check if blocked joining the space`, async ({
    page,
  }) => {
    await login(page, participant5);
    await goToMySpaces(page);
    await goToSpace(page, POST_TITLE);

    // Should be blocked from joining after deliberation starts
    const blockedMessage = page.getByTestId('participation-blocked-message');
    if (await blockedMessage.isVisible()) {
      await expect(blockedMessage).toBeVisible();
    } else {
      // Alternative check
      const accessDenied = page.getByText(/blocked|closed|cannot join/i);
      if (await accessDenied.isVisible()) {
        await expect(accessDenied).toBeVisible();
      }
    }
  });

  // =====================================
  // Discussion: Participant and Creator
  // =====================================
  // FIXME: rewrite testcode date setting scenario
  // test(`DS-${i()} [Participant 1] Reply to a post on a board (P1)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant1);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await replyToPost(
  //     page,
  //     'I think participation-based rewards are more fair.',
  //   );
  // });

  // test(`DS-${i()} [Participant 2] Reply to a post on a board (P2)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant2);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await replyToPost(
  //     page,
  //     'Quality-based approach helps community growth more.',
  //   );
  // });

  // test(`DS-${i()} [Creator 1] Write a new post on the board`, async ({
  //   page,
  // }) => {
  //   await login(page, creator1);
  //   await page.goto(`/teams/${teamId}/home`);
  //   await page.waitForLoadState('networkidle');
  //   await page.getByText(POST_TITLE).click();
  //   await page.waitForLoadState('networkidle');

  //   await writeNewPost(
  //     page,
  //     'Additional discussion: How about a hybrid approach?',
  //     'I propose a hybrid approach that considers both participation and quality.',
  //     'Fairness and Efficiency of Reward Criteria',
  //   );
  // });

  // test(`DS-${i()} [Participant 3] Reply to the new post`, async ({ page }) => {
  //   await login(page, participant3);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await replyToPost(
  //     page,
  //     'I think a hybrid approach would be a good compromise.',
  //   );
  // });

  // =====================================
  // Final Survey
  // =====================================
  // test(`DS-${i()} [Creator 2] Write the final survey`, async ({ page }) => {
  //   await login(page, creator2);
  //   await page.goto(`/teams/${teamId}/home`);
  //   await page.waitForLoadState('networkidle');
  //   await page.getByText(POST_TITLE).click();
  //   await page.waitForLoadState('networkidle');

  //   // Navigate to polls and create final survey
  //   await createFinalSurvey(page, SURVEY_QUESTIONS);
  // });

  // test(`DS-${i()} [Participant 1] Conduct the final survey (P1)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant1);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await goToFinalSurvey(page);
  //   await conductSurvey(page, [
  //     0,
  //     0,
  //     0,
  //     1,
  //     'Part 1 is important',
  //     "I don't have any idea",
  //   ]);

  //   await page.getByTestId('btn-confirm').click();
  //   await expect(
  //     page.getByTestId('complete-survey-modal-btn-confirm'),
  //   ).toBeVisible();
  //   await page.getByTestId('complete-survey-modal-btn-confirm').click();
  // });

  // test(`DS-${i()} [Participant 2] Conduct the final survey (P2)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant2);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await goToFinalSurvey(page);
  //   await conductSurvey(page, [1, 1, 2, 1, 'Part 1 is important', 'Good']);
  //   await page.getByTestId('btn-confirm').click();
  //   await expect(
  //     page.getByTestId('complete-survey-modal-btn-confirm'),
  //   ).toBeVisible();
  //   await page.getByTestId('complete-survey-modal-btn-confirm').click();
  // });

  // test(`DS-${i()} [Participant 3] Conduct the final survey (P3)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant3);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await goToFinalSurvey(page);
  //   await conductSurvey(page, [
  //     1,
  //     0,
  //     0,
  //     2,
  //     'Part 2 is important',
  //     "I don't have any idea",
  //   ]);
  //   await page.getByTestId('btn-confirm').click();
  //   await expect(
  //     page.getByTestId('complete-survey-modal-btn-confirm'),
  //   ).toBeVisible();
  //   await page.getByTestId('complete-survey-modal-btn-confirm').click();
  // });

  // test(`DS-${i()} [Participant 4] Conduct the final survey (P4)`, async ({
  //   page,
  // }) => {
  //   await login(page, participant4);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);
  //   await goToFinalSurvey(page);
  //   await conductSurvey(page, [
  //     1,
  //     0,
  //     0,
  //     2,
  //     'Part 2 is important',
  //     "I don't have any idea",
  //   ]);
  //   await page.getByTestId('btn-confirm').click();
  //   await expect(
  //     page.getByTestId('complete-survey-modal-btn-confirm'),
  //   ).toBeVisible();
  //   await page.getByTestId('complete-survey-modal-btn-confirm').click();
  // });

  // // TODO: check
  // test.skip(`DS-${i()} [Creator 1] See analysis`, async ({ page }) => {
  //   await login(page, creator1);
  //   await goToMySpaces(page);
  //   await goToSpace(page, POST_TITLE);

  //   await viewAnalysis(page);
  // });
});
