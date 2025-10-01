import { Answer } from '@/lib/api/models/response';
import { File } from '@/lib/api/models/spaces/deliberation-spaces';
import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';

export enum DeliberationTab {
  SUMMARY = 'Summary',
  DELIBERATION = 'Deliberation',
  POLL = 'Poll',
  RECOMMANDATION = 'Recommendation',
  ANALYZE = 'Analyze',
}

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
  elearnings: ElearningCreateRequest[];
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
