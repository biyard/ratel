import { NewDiscussionCreateRequest } from '../../models/discussion';
import { Answer } from '../../models/response';
import { NewSurveyCreateRequest, Question } from '../../models/survey';

export type PartitionString = string;

export interface DeliberationSpace {
  pk: string;
  sk: string;
  created_at: number;
  updated_at: number;
  status: SpaceStatus | undefined | null;
  publish_state: SpacePublishState;
  likes: number;
  comments: number;
  rewards: number;
  shares: number;
  visibility: SpaceVisibility;
  post_pk: string;
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  info: DeliberationSpace;
  summary: DeliberationContentResponse;
  discussions: DeliberationDiscussionResponse[];
  elearnings: ElearningResponse;
  surveys: DeliberationSurveyResponse;
  recommendation: DeliberationContentResponse;
}

export const SpacePublishState = {
  Draft: 'Draft',
  Published: 'Published',
} as const;

export type SpacePublishState = typeof SpacePublishState[keyof typeof SpacePublishState];

export const SpaceStatus = {
  Waiting: 'Waiting',
  InProgress: 'InProgress',
  Finish: 'Finish',
} as const;

export type SpaceStatus = typeof SpaceStatus[keyof typeof SpaceStatus];

export interface ElearningResponse {
  files: File[];
}

export interface DiscussionMemberResponse {
  user_pk: PartitionString;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export interface DiscussionParticipantResponse {
  user_pk: PartitionString;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  participant_id: string;
}

export interface DeliberationDiscussionResponse {
  pk: string | undefined;
  started_at: number;
  ended_at: number;
  name: string;
  description: string;
  meeting_id: string | null;
  pipeline_id: string;
  media_pipeline_arn: string | null;
  record: string | null;
  user_pk: PartitionString;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  members: DiscussionMemberResponse[];
  participants: DiscussionParticipantResponse[];
}

export interface DeliberationSpace {
  pk: string;
  sk: string;
}

export interface DeliberationContentResponse {
  html_contents: string;
  files: File[];
}

export interface File {
  name: string;
  size: string;
  ext: FileExtension;
  url?: string | null;
}

export const FileExtension = {
  JPG: 'JPG',
  PNG: 'PNG',
  PDF: 'PDF',
  ZIP: 'ZIP',
  WORD: 'WORD',
  PPTX: 'PPTX',
  EXCEL: 'EXCEL',
  MP4: 'MP4',
  MOV: 'MOV',
} as const;

export type FileExtension = typeof FileExtension[keyof typeof FileExtension];

export const FileExtensionMap: Record<FileExtension, string> = {
  [FileExtension.JPG]: 'JPG',
  [FileExtension.PNG]: 'PNG',
  [FileExtension.PDF]: 'PDF',
  [FileExtension.ZIP]: 'ZIP',
  [FileExtension.WORD]: 'WORD',
  [FileExtension.PPTX]: 'PPTX',
  [FileExtension.EXCEL]: 'EXCEL',
  [FileExtension.MP4]: 'MP4',
  [FileExtension.MOV]: 'MOV',
};

type BackendFile = Omit<File, 'ext'> & { ext: string };

export const toBackendFile = (f: File): BackendFile => ({
  ...f,
  ext: FileExtensionMap[f.ext],
});

export type SpaceVisibility =
  | 'PRIVATE'
  | 'PUBLIC'
  | `Team:${string}`
  | `TeamGroupMember:${string}`;

type SurveyType = 'SAMPLE' | 'SURVEY';

export interface DeliberationResponse {
  pk: string;
  created_at: number;
  updated_at: number;
  likes: number;
  comments: number;
  rewards: number;
  shares: number;
  visibility: SpaceVisibility;
  title: string;
  post_pk: string;
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export type SurveyStatus = 'READY' | 'IN_PROGRESS' | 'FINISH'; //1: ready, 2: in progress, 3: finish

export interface SurveyResponseResponse {
  pk?: string;
  user_pk?: string;
  author_display_name?: string;
  author_profile_url?: string;
  author_username?: string;
  survey_type: SurveyType;
  answers?: Answer[];
}

export interface DeliberationSurveyResponse {
  pk: PartitionString;
  started_at: number;
  ended_at: number;
  status: SurveyStatus;
  questions: Question[];
  responses: SurveyResponseResponse[];
  user_responses: SurveyResponseResponse[];
}

export interface ResponseCreateRequest {
  survey_pk: string;
  survey_type: string;
  answers: Answer[];
}

export function responseCreateRequest(
  survey_pk: string,
  answers: Answer[],
): ResponseCreateRequest {
  return {
    survey_pk,
    survey_type: 'SURVEY',
    answers,
  };
}

export interface UpdateSpaceRequest {
  title?: string;
  html_contents: string;
  files: BackendFile[];

  discussions: NewDiscussionCreateRequest[];
  elearning_files: BackendFile[];
  surveys: NewSurveyCreateRequest[];

  recommendation_html_contents?: string;
  recommendation_files: BackendFile[];

  visibility: SpaceVisibility;
  started_at: number;
  ended_at: number;
}

export function updateSpaceRequest(
  html_contents: string,
  files: BackendFile[],

  discussions: NewDiscussionCreateRequest[],
  elearning_files: BackendFile[],

  surveys: NewSurveyCreateRequest[],

  recommendation_files: BackendFile[],

  visibility: SpaceVisibility,
  started_at: number,
  ended_at: number,
  title?: string,
  recommendation_html_contents?: string,
): UpdateSpaceRequest {
  return {
    title,
    html_contents,
    files,

    discussions,
    elearning_files,

    surveys,

    recommendation_files,
    recommendation_html_contents,

    visibility,
    started_at,
    ended_at,
  };
}
