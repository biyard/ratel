import { Answer } from '@/lib/api/models/response';
import { SurveyCreateRequest } from '@/lib/api/models/survey';

export const PollTab = {
  POLL: 'Poll',
  ANALYZE: 'Analyze',
} as const;

export type PollTabType = (typeof PollTab)[keyof typeof PollTab];

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface SurveyAnswer {
  answers: Answer[];
  is_completed: boolean;
}
