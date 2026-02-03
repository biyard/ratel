import { CONFIGS } from '../../tests/config';

export const REWARD_TEST_PASSWORD = 'admin!234';

// Team configuration for reward tests
export const TEAM_NAME = `Reward E2E Team ${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_ID = `reward-e2e-${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_DESCRIPTION = 'Team for reward functionality verification';

export const POLL_TITLE = `${CONFIGS.PLAYWRIGHT.ID} Reward Poll`;
export const POLL_CONTENT = `Poll for reward functionality verification`;

export const POLL_QUESTIONS = [
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title: 'What is your favorite color?',
    options: ['Red', 'Blue', 'Green', 'Yellow'],
  },
];

export const REWARD_CONFIG = {
  credits: 1,
  description: 'Reward for poll response',
};
