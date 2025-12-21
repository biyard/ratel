// Re-export all common helpers from shared location
export {
  login,
  logout,
  mobileLogin,
  TIMEOUT,
  verifyCredential,
  createTeam,
  clickTeamSidebarMenu,
  goToSpace,
  publishSpacePrivately,
  startDeliberation,
  inviteMembers,
  setupPanels,
  enableAnonymousParticipation,
  viewAnalysis,
  createDeliberationPost,
  replyToPost,
  writeNewPost,
  setEndTimeOneHourLater,
  createPrePollSurvey,
  createFinalSurvey,
  conductSurvey,
  goToFinalSurvey,
  createBoardPosts,
} from '../helpers';

import { expect, Page } from '@playwright/test';
import { POST_TITLE, TEAM_ID } from './data';
import { goToTeam as goToTeamBase, goToTeamSpace as goToTeamSpaceBase } from '../helpers';

// Deliberation-specific wrappers that use TEAM_ID from data
export async function goToTeam(page: Page) {
  await goToTeamBase(page, TEAM_ID);
}

export async function goToTeamSpace(page: Page) {
  await goToTeamSpaceBase(page, TEAM_ID);
}

// Deliberation-specific goToMySpaces that checks for POST_TITLE
export async function goToMySpaces(page: Page) {
  await page.getByText('My Spaces', { exact: true }).click();

  await page.waitForURL(/.*/, { waitUntil: 'networkidle', timeout: 10000 });

  await expect(page.getByTestId('space-card').first()).toBeVisible();
  await expect(page.getByTestId('space-card').first()).toContainText(
    POST_TITLE,
  );
}
