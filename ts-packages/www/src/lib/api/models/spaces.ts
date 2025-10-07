import type { Badge } from './badge';
import type { SpaceComment } from './comments';
import type { Discussion, DiscussionCreateRequest } from './discussion';
import type { Elearning, ElearningCreateRequest } from './elearning';
import type { FileInfo } from './feeds';
import type { SurveyResponse } from './response';
import type { SpaceDraft, SpaceDraftCreateRequest } from './space_draft';
import type { SprintLeague } from './sprint_league';
import type { Survey, SurveyCreateRequest } from './survey';
import type { UserType } from './user';
import type {
  QuizQuestion,
  BoosterType,
  NoticeQuizRequest,
} from './notice';
import { PublishingScope } from './notice';

export interface Space {
  id: number;
  created_at: number;
  updated_at: number;
  title?: string;
  started_at?: number;
  ended_at?: number;
  html_contents: string;
  space_type: SpaceType;
  owner_id: number;
  owner_username: string;
  industry_id: number;
  feed_id: number;
  author: Author[];
  status: SpaceStatus;
  files: FileInfo[];

  badges: Badge[];
  feed_comments: SpaceComment[];
  discussions: Discussion[];
  elearnings: Elearning[];
  surveys: Survey[];
  user_responses: SurveyResponse[];
  responses: SurveyResponse[];
  drafts: SpaceDraft[];

  sprint_leagues?: SprintLeague[];

  likes: number;
  shares: number;
  is_liked: boolean;

  // Quiz
  notice_quiz: QuizQuestion[];
  booster_type?: BoosterType;
  // Publishing scope
  publishing_scope: PublishingScope;
}

export interface PostingSpaceRequest {
  posting_space: object;
}

export function postingSpaceRequest(): PostingSpaceRequest {
  return {
    posting_space: {},
  };
}

export interface SpaceDeleteRequest {
  confirmation: boolean;
  space_name: string;
}

export function spaceDeleteRequest(space_name: string): SpaceDeleteRequest {
  return {
    confirmation: true,
    space_name: space_name,
  };
}

export interface SpaceUpdateRequest {
  update_space: {
    title?: string;
    started_at?: number;
    ended_at?: number;
    html_contents: string;
    files: FileInfo[];
    discussions: DiscussionCreateRequest[];
    elearnings: ElearningCreateRequest[];
    surveys: SurveyCreateRequest[];
    drafts: SpaceDraftCreateRequest[];
    publishing_scope: PublishingScope;
    quiz?: NoticeQuizRequest | null; // Updated to use new format
  };
}

export function spaceUpdateRequest(
  html_contents: string,
  files: FileInfo[],
  discussions: DiscussionCreateRequest[],
  elearnings: ElearningCreateRequest[],
  surveys: SurveyCreateRequest[],
  drafts: SpaceDraftCreateRequest[],
  title?: string,
  started_at?: number,
  ended_at?: number,
  publishing_scope: PublishingScope = PublishingScope.Private,
  quiz?: NoticeQuizRequest | null,
): SpaceUpdateRequest {
  return {
    update_space: {
      title,
      started_at,
      ended_at,
      html_contents,
      files,
      discussions,
      elearnings,
      surveys,
      drafts,
      publishing_scope,
      quiz,
    },
  };
}

export interface CreateSpaceRequest {
  create_space: {
    space_type: SpaceType;
    feed_id: number;
    user_ids: number[];
    num_of_redeem_codes: number;
    started_at: number | null;
    ended_at: number | null;
    booster_type: BoosterType | null;
  };
}

export function createSpaceRequest(
  space_type: SpaceType,
  feed_id: number,
  user_ids: number[] = [],
  num_of_redeem_codes: number = 0,
  started_at: number | null = null,
  ended_at: number | null = null,
  booster_type: BoosterType | null = null,
): CreateSpaceRequest {
  return {
    create_space: {
      space_type,
      feed_id,
      user_ids,
      num_of_redeem_codes,
      started_at,
      ended_at,
      booster_type,
    },
  };
}
export interface Author {
  id: number;
  nickname: string;
  username: string;
  principal: string;
  email: string;
  profile_url: string;

  user_type: UserType;
}

export const SpaceType = {
  Legislation: 1,
  Poll: 2,
  Deliberation: 3,
  Nft: 4,
  Committee: 5,
  SprintLeague: 6,
  Notice: 7,
  dAgit: 8,
} as const;

export type SpaceType = typeof SpaceType[keyof typeof SpaceType];

export const SpaceStatus = {
  Draft: 1,
  InProgress: 2,
  Finish: 3,
} as const;

export type SpaceStatus = typeof SpaceStatus[keyof typeof SpaceStatus];
