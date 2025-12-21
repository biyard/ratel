import { CONFIGS } from '../../tests/config';

export const TEAM_NAME = `Rewards Test Team ${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_ID = `rewards-test-${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_DESCRIPTION = 'A team for testing reward configuration flow';

// Password for all test users
export const TEST_PASSWORD = 'admin!234';

export const POST_TITLE = `${CONFIGS.PLAYWRIGHT.ID} Reward Test Poll`;
export const POST_CONTENT = `This is a test poll for reward configuration.

1. Background
Testing the reward system from membership subscription to reward setup.

2. Questions
Q1. Do you like rewards?
Q2. What type of rewards do you prefer?`;

// Survey questions for poll
export const POLL_QUESTIONS = [
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title: 'Do you like receiving rewards for participation?',
    options: ['Yes, definitely', 'Somewhat', 'Not really', 'No'],
  },
  {
    type: 'multiple_choice',
    displayName: 'Multiple Choice',
    required: true,
    title: 'What types of rewards motivate you to participate?',
    options: [
      'Points/Credits',
      'Badges',
      'Recognition',
      'Exclusive access',
      'Physical prizes',
    ],
  },
];

// Reward configuration
export const REWARD_CONFIG = {
  label: 'Poll Participation Reward',
  description: 'Earn points for completing this poll',
  credits: 10,
};
