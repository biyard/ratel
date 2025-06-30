import { Badge } from './badge';
import { SpaceComment } from './comments';
import { Discussion, DiscussionCreateRequest } from './discussion';
import { Elearning, ElearningCreateRequest } from './elearning';
import { FileInfo } from './feeds';
import { SurveyResponse } from './response';
import { SpaceDraft, SpaceDraftCreateRequest } from './space_draft';
import { Survey, SurveyCreateRequest } from './survey';
import { UserType } from './user';

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
  drafts: SpaceDraft[];
}

export interface PostingSpaceRequest {
  posting_space: object;
}

export function postingSpaceRequest(): PostingSpaceRequest {
  return {
    posting_space: {},
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
    },
  };
}

export interface Author {
  id: number;
  nickname: string;
  principal: string;
  email: string;
  profile_url: string;

  user_type: UserType;
}

export enum SpaceType {
  Legislation = 1,
  Poll = 2,
  Deliberation = 3,
  Nft = 4,
  Committee = 5,
}

export enum SpaceStatus {
  Draft = 1,
  InProgress = 2,
  Finish = 3,
}
