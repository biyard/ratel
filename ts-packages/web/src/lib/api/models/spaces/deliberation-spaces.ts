import { NewDiscussionCreateRequest } from '../discussion';
import { Answer } from '../response';
import { NewSurveyCreateRequest, Question } from '../survey';

export type PartitionString = string;

export interface DeliberationSpace {
  pk: string;
  created_at: number;
  updated_at: number;
  likes: number;
  comments: number;
  rewards: number;
  shares: number;
  visibility: Visibility;
  title: string;
  post_pk: string;
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  summary: DeliberationContentResponse;
  discussions: DeliberationDiscussionResponse[];
  elearnings: ElearningResponse;
  surveys: DeliberationSurveyResponse;
  recommendation: DeliberationContentResponse;
}

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
  pk: PartitionString;
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

export enum FileExtension {
  JPG = 1,
  PNG = 2,
  PDF = 3,
  ZIP = 4,
  WORD = 5,
  PPTX = 6,
  EXCEL = 7,
  MP4 = 8,
  MOV = 9,
}

export type Visibility =
  | 'Public'
  | `Team:${string}`
  | `TeamGroupMember:${string}`;

type SurveyType = 1 | 2;

export interface DeliberationResponse {
  pk: string;
  created_at: number;
  updated_at: number;
  likes: number;
  comments: number;
  rewards: number;
  shares: number;
  visibility: Visibility;
  title: string;
  post_pk: string;
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export type SurveyStatus = 1 | 2 | 3; //1: ready, 2: in progress, 3: finish

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
  survey_type: number;
  answers: Answer[];
}

export function responseCreateRequest(
  survey_pk: string,
  answers: Answer[],
): ResponseCreateRequest {
  return {
    survey_pk,
    survey_type: 2,
    answers,
  };
}

export interface UpdateSpaceRequest {
  title?: string;
  html_contents: string;
  files: File[];

  discussions: NewDiscussionCreateRequest[];
  elearning_files: File[];

  surveys: NewSurveyCreateRequest[];

  recommendation_html_contents?: string;
  recommendation_files: File[];

  visibility: Visibility;
  started_at: number;
  ended_at: number;
}

export function updateSpaceRequest(
  html_contents: string,
  files: File[],

  discussions: NewDiscussionCreateRequest[],
  elearning_files: File[],

  surveys: NewSurveyCreateRequest[],

  recommendation_files: File[],

  visibility: Visibility,
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
