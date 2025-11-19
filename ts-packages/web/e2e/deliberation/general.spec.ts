import { test, expect, Page } from '@playwright/test';
import {
  BOARD_POSTS,
  POST_CONTENT,
  POST_TITLE,
  SURVEY_QUESTIONS,
  TEAM_DESCRIPTION,
  TEAM_ID,
  TEAM_NAME,
  YOUTUBE_LINK,
} from './data';
import { clickTeamSidebarMenu, login, setEndTimeOneHourLater } from './helpers';

test.describe.serial('[Deliberation] General Spec', () => {
  let no = 0;

  const i = () => {
    no += 1;

    return no;
  };

  // Verification: Participant
  test(`DS-${i()} [Participant] Sign in with Participant1`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Verifying credential with code1`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant2`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Verifying credential with code2`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant3`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Verifying credential with code3`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant4`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Verifying credential with code4`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant5`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Verifying credential with code1`, async ({
    page,
  }) => {});

  // Design and Publish: Creator
  test(`DS-${i()} [Creator] Sign in with Creator1`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Crate a team`, async ({ page }) => {});

  test(`DS-${i()} [Creator] Invite a member(Creator2) to team`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Creator] Create a draft with a deliberation space`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Creator] Sign in with Creator2`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Check a team draft page`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Modifying Overview`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Creating a Pre-Survey poll`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Creator] Creating a board`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Inviting members`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Setting up panels`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Configure anonymous`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Publish privately`, async ({ page }) => {});

  // Rejection: Unverified Partitipant
  test(`DS-${i()} [Unverified Participant] Sign in with Unverified participant`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Unverified Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Unverified Participant] Should be rejected`, async ({
    page,
  }) => {});

  // PrePoll: Participant
  test(`DS-${i()} [Participant] Sign in with Participant1`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Participate the space`, async ({ page }) => {});
  test(`DS-${i()} [Participant] Cannot see any contents`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Can see Pre-Poll Survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct Pre-Poll Survey`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant2`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Participate the space`, async ({ page }) => {});
  test(`DS-${i()} [Participant] Cannot see any contents`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Can see Pre-Poll Survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct Pre-Poll Survey`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant3`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Participate the space`, async ({ page }) => {});
  test(`DS-${i()} [Participant] Cannot see any contents`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Can see Pre-Poll Survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct Pre-Poll Survey`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant4`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Check invitation in My Spaces`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Participate the space`, async ({ page }) => {});
  test(`DS-${i()} [Participant] Cannot see any contents`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Can see Pre-Poll Survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct Pre-Poll Survey`, async ({
    page,
  }) => {});

  // Start Deliberation: Creator
  test(`DS-${i()} [Creator] Sign in with Creator1`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Start Deliberation`, async ({ page }) => {});

  // Blocking: Participant
  test(`DS-${i()} [Participant] Sign in with Participant5`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Check if blocked joining the space`, async ({
    page,
  }) => {});

  // Discussion: Participant and Creator
  test(`DS-${i()} [Participant] Sign in with Participant1`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Reply to a post on a board`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant2`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Reply to a post on a board`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Creator] Sign in with Creator1`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Write a new post on the board`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant3`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Reply to the new post`, async ({ page }) => {});

  // Final Survey
  test(`DS-${i()} [Creator] Sign in with Creator2`, async ({ page }) => {});
  test(`DS-${i()} [Creator] Write the final survey`, async ({ page }) => {});

  test(`DS-${i()} [Participant] Sign in with Participant1`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct the final survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Sign in with Participant2`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct the final survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Sign in with Participant3`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct the final survey`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Sign in with Participant4`, async ({
    page,
  }) => {});
  test(`DS-${i()} [Participant] Conduct the final survey`, async ({
    page,
  }) => {});

  test(`DS-${i()} [Creator] Sign in with Creator1`, async ({ page }) => {});
  test(`DS-${i()} [Creator] See analysis`, async ({ page }) => {});
});
