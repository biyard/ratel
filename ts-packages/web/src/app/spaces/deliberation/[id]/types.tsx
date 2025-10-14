import { Answer } from '@/lib/api/models/response';
import { File } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';

export const DeliberationTab = {
  SUMMARY: 'Summary',
  DELIBERATION: 'Deliberation',
  POLL: 'Poll',
  RECOMMANDATION: 'Recommendation',
  ANALYZE: 'Analyze',
} as const;

export type DeliberationTab =
  (typeof DeliberationTab)[keyof typeof DeliberationTab];

export type DeliberationTabType = DeliberationTab;

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface Thread {
  html_contents: string;
  files: File[];
}

export interface FinalConsensus {
  drafts: RecommendationCreateRequest;
}

export interface DiscussionInfo {
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  discussion_pk?: string;

  participants: DiscussionUser[];
}

export interface DiscussionUser {
  user_pk: string;
  display_name: string;
  profile_url: string;
  username: string;
}

export interface Deliberation {
  discussions: DiscussionInfo[];
  elearnings: ElearningCreateRequest;
}

export interface ElearningCreateRequest {
  files: File[];
}

export interface RecommendationCreateRequest {
  html_contents: string;
  files: File[];
}

export interface MappedResponse {
  question: Question;
  answers: Answer[];
}
