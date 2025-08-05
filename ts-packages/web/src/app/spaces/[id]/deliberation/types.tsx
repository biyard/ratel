import { ElearningCreateRequest } from '@/lib/api/models/elearning';
import { FileInfo } from '@/lib/api/models/feeds';
import { SpaceDraftCreateRequest } from '@/lib/api/models/space_draft';
import { TotalUser } from '@/lib/api/models/user';

export const DeliberationTab = {
  SUMMARY: 'Summary',
  DELIBERATION: 'Deliberation',
  POLL: 'Poll',
  RECOMMANDATION: 'Recommendation',
  ANALYZE: 'Analyze',
} as const;

export type DeliberationTabType =
  (typeof DeliberationTab)[keyof typeof DeliberationTab];

export interface Thread {
  html_contents: string;
  files: FileInfo[];
}

export interface FinalConsensus {
  drafts: SpaceDraftCreateRequest[];
}

export interface DiscussionInfo {
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  discussion_id?: number;

  participants: TotalUser[];
}

export interface Deliberation {
  discussions: DiscussionInfo[];
  elearnings: ElearningCreateRequest[];
}
