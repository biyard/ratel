// Quiz Types - Read-only versions for display
export interface QuizQuestion {
  title: string;
  images: ImgFile[];
  options: QuizOption[];
}

export interface QuizOption {
  content: string;
}

// Quiz Types - With answers for creation/submission (matches backend NoticeQuestionWithAnswer)
export interface NoticeQuestionWithAnswer {
  title: string;
  images: ImgFile[];
  options: NoticeOptionWithAnswer[];
}

export interface NoticeOptionWithAnswer {
  content: string;
  is_correct: boolean;
}

export interface ImgFile {
  name: string;
  size: string;
  ext: ImgFileExtension;
  url: string | null;
}

export enum ImgFileExtension {
  JPG = 1,
  PNG = 2,
}

// Quiz Attempt Types (matches backend NoticeQuizAttempt)
export interface QuizAttempt {
  id: number;
  created_at: number;
  updated_at: number;
  space_id: number;
  user_id: number;
  user_answers: NoticeQuestionWithAnswer[];
  is_successful: boolean;
}

// Quiz Attempts Response (matches backend QueryResponse<NoticeQuizAttempt>)
export interface QuizAttemptsResponse {
  total_count: number;
  items: QuizAttempt[];
}

// Quiz Answer Type (matches backend NoticeQuizAnswer)
export interface NoticeQuizAnswer {
  id: number;
  created_at: number;
  updated_at: number;
  space_id: number;
  user_id: number;
  notice_quiz: NoticeQuestionWithAnswer[];
}

// Quiz submission interfaces
export interface SpaceSubmitQuizAnswersRequest {
  answers: NoticeQuestionWithAnswer[];
}

export function spaceSubmitQuizAnswersRequest(
  answers: NoticeQuestionWithAnswer[],
): SpaceSubmitQuizAnswersRequest {
  return {
    answers,
  };
}

// Notice specific enums
export enum BoosterType {
  NoBoost = 1,
  X2 = 2,
  X10 = 3,
  X100 = 4,
}

export enum PublishingScope {
  Private = 1,
  Public = 2,
}

// We need to import SpaceType from spaces to avoid duplication
import { SpaceType } from './spaces';

// Notice Space Creation
export interface NoticeSpaceCreateRequest {
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

export function noticeSpaceCreateRequest(
  space_type: SpaceType,
  feed_id: number,
  user_ids: number[] = [],
  num_of_redeem_codes: number = 0,
  started_at: number | null = null,
  ended_at: number | null = null,
  booster_type: BoosterType | null = null,
): NoticeSpaceCreateRequest {
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
