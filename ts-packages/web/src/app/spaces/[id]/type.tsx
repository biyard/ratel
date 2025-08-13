import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';
import { useDeliberationSpaceContext } from './deliberation/provider.client';
import { usePollSpaceContext } from './poll/provider.client';
import { Answer } from '@/lib/api/models/response';

export type PollContextType = ReturnType<typeof usePollSpaceContext>;
export type DeliberationContextType = ReturnType<
  typeof useDeliberationSpaceContext
>;

export type SpaceContextType = PollContextType | DeliberationContextType;

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface SurveyAnswer {
  answers: Answer[];
  is_completed: boolean;
}

export interface MappedResponse {
  question: Question;
  answers: Answer[];
}
