import type { Meta, StoryObj } from '@storybook/react-vite';

import { Analyze, I18nFunction } from './index';
import {
  SurveyAnswerType,
  SurveyQuestion,
  SurveySummary,
} from '@/types/survey-type';

const mockT: I18nFunction = ((key: string) => key) as any;

const mockQuestions: SurveyQuestion[] = [
  {
    answer_type: SurveyAnswerType.SingleChoice,
    content: {
      title: 'What is your favorite color?',
      options: ['Red', 'Blue', 'Green'],
      is_required: true,
    },
  },
  {
    answer_type: SurveyAnswerType.Subjective,
    content: {
      title: 'Tell us about yourself',
      description: 'Please share your thoughts',
      is_required: false,
    },
  },
];

const mockSummaries: SurveySummary[] = [
  {
    type: SurveyAnswerType.SingleChoice,
    question: mockQuestions[0],
    total_count: 10,
    answers: { 0: 4, 1: 3, 2: 3 },
  },
  {
    type: SurveyAnswerType.Subjective,
    question: mockQuestions[1],
    answers: { 'I love coding': 5, 'I enjoy reading': 3 },
  },
];

const meta: Meta<typeof Analyze> = {
  title: 'Features/Spaces/Analyze',
  component: Analyze,
  parameters: {
    layout: 'padded',
  },
  args: {
    startedAt: Date.now() - 86400000, // as Milliseconds
    endedAt: Date.now(),
    totalResponses: 10,
    questions: mockQuestions,
    summaries: mockSummaries,
    t: mockT,
  },
};

export default meta;
type Story = StoryObj<typeof Analyze>;

export const Default: Story = {};

export const NoResponses: Story = {
  args: {
    totalResponses: 0,
    summaries: [],
  },
};

export const WithResponses: Story = {
  args: {
    totalResponses: 15,
  },
};
