import { ElearningCreateRequest } from '@/lib/api/models/elearning';
import { FileInfo } from '@/lib/api/models/feeds';
import { Answer } from '@/lib/api/models/response';
import { SpaceDraftCreateRequest } from '@/lib/api/models/space_draft';
import { SurveyCreateRequest } from '@/lib/api/models/survey';
import { TotalUser } from '@/lib/api/models/user';

export const DeliberationTab = {
  SUMMARY: 'Summary',
  DELIBERATION: 'Deliberation',
  POLL: 'Poll',
  RECOMMANDATION: 'Recommandation',
} as const;

export type DeliberationTabType =
  (typeof DeliberationTab)[keyof typeof DeliberationTab];

export interface Thread {
  html_contents: string;
  files: FileInfo[];
}

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface SurveyAnswer {
  answers: Answer[];
  is_completed: boolean;
}

export interface FinalConsensus {
  drafts: SpaceDraftCreateRequest[];
}

export interface DiscussionInfo {
  started_at: number;
  ended_at: number;
  name: string;
  description: string;

  participants: TotalUser[];
}

export interface Deliberation {
  discussions: DiscussionInfo[];
  elearnings: ElearningCreateRequest[];
}
