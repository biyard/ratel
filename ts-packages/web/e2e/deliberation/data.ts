import { CONFIGS } from '../../tests/config';

export const TEAM_NAME = `Legislative Research Pilot Team ${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_ID = `iitp-poc-1-${CONFIGS.PLAYWRIGHT.ID}`;
export const TEAM_DESCRIPTION =
  'A team preparing deliberative polling in collaboration with the National Assembly Research Service.';

// Attribute codes for credential verification
export const ATTRIBUTE_CODES = {
  SOGANG_MALE: 'j94EA1',
  SOGANG_FEMALE: 'bIFviB',
  KONKUK_MALE: 'bVn0Vq',
  KONKUK_FEMALE: 'wKFegq',
};

// Password for all test users
export const TEST_PASSWORD = 'admin!234';

export const POST_TITLE = `${CONFIGS.PLAYWRIGHT.ID} Discussion on Reward Space`;
export const POST_CONTENT = `1. Background
Ratel provides a RewardSpace feature that rewards user participation.
Currently, most spaces distribute points based on activity count (participation).
However, some creators recently argue that "quality of participation should be valued more."

2. Issues
Option A: Participation-based distribution
Option B: Quality-based distribution

3. Questions
Q1. Which reward criteria do you think is more appropriate?
Q2. If quality evaluation is introduced, who should be the evaluator?`;

export const YOUTUBE_LINK = 'https://www.youtube.com/watch?v=R2X4BJ1KNM4';

// Survey questions
export const SURVEY_QUESTIONS = [
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title:
      'Have you ever participated in online deliberation platforms (discussion/voting platforms) like Ratel?',
    options: [
      'I participate frequently',
      'I participate occasionally',
      'I have heard of it but never participated',
      'I have never participated at all',
    ],
  },
  {
    type: 'single_choice',
    displayName: 'Single Choice',
    required: true,
    title:
      'What do you think is the most important factor when participating in deliberative polling?',
    options: [
      'A structure where diverse opinions are fairly reflected',
      'Public interest of the discussion topic',
      'Rewards for participation',
      'Convenient participation environment (UI, time, etc.)',
      'Anonymity and privacy protection',
    ],
  },
  {
    type: 'multiple_choice',
    displayName: 'Multiple Choice',
    required: true,
    title:
      'Please select all the reasons why you actively participate in deliberative polling.',
    options: [
      'Because I can influence social decision-making',
      'Because I like that my opinions are recorded',
      'Because I can gain new perspectives through discussion',
      'Because there are rewards like points',
      'Because of recommendations from friends or community',
    ],
  },
  {
    type: 'short_answer',
    displayName: 'Short Answer',
    required: true,
    title:
      'What do you think is the most important factor for trusting online deliberative polling or discussion platforms?',
  },
  {
    type: 'subjective',
    displayName: 'Subjective',
    required: true,
    title:
      'If you could propose a topic for deliberative polling, what topic would you like to suggest?',
  },
];

// Board posts
export const BOARD_POSTS = [
  {
    category: 'Fairness and Efficiency of Reward Criteria',
    title: 'Why activity-based rewards are fair',
    content:
      "RewardSpace is a system designed to encourage 'participation' itself.",
  },
  {
    category: 'Fairness and Efficiency of Reward Criteria',
    title: 'Quality-based rewards make the community more mature',
    content:
      'What matters more than quantitative participation is the depth of contribution.',
  },
  {
    category: 'Balance between AI Evaluation and User Autonomy',
    title: 'Introducing AI evaluation, the first step to improving fairness',
    content: 'AI has no emotions or private interests.',
  },
  {
    category: 'Balance between AI Evaluation and User Autonomy',
    title: 'AI evaluation may limit autonomy and creativity',
    content:
      "If AI starts quantifying and grading all participation, there is a risk that people will only make 'statements to look good.'",
  },
];
