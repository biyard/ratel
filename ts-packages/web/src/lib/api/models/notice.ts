// // Quiz Types - Read-only versions for display (matches backend NoticeQuestion/NoticeOption)
// export interface QuizQuestion {
//   id: string; // UUID from backend
//   title: string;
//   images: string[]; // Backend sends string[] URLs directly
//   options: QuizOption[];
// }

// export interface QuizOption {
//   id: string; // UUID from backend
//   content: string;
// }

// // Quiz Request Types - For creating/updating quizzes (matches backend NoticeQuizRequest)
// export interface NoticeQuizRequest {
//   questions: NoticeQuestionRequest[];
// }

// export interface NoticeQuestionRequest {
//   title: string;
//   images: string[];
//   options: NoticeOptionRequest[];
// }

// export interface NoticeOptionRequest {
//   content: string;
//   is_correct: boolean;
// }

// // Answer Storage Types (matches backend NoticeAnswer)
// export interface NoticeAnswer {
//   answers: { [questionId: string]: string[] };
// }

// // Quiz Submission Request
// export interface SpaceSubmitQuizAnswersRequest {
//   answers: NoticeAnswer;
// }

// export interface ImgFile {
//   name: string;
//   size: string;
//   ext: ImgFileExtension;
//   url: string | null;
// }

// export const ImgFileExtension = {
//   JPG: 1,
//   PNG: 2,
// } as const;

// export type ImgFileExtension =
//   (typeof ImgFileExtension)[keyof typeof ImgFileExtension];

// // Quiz Attempt Types (matches backend NoticeQuizAttempt)
// export interface QuizAttempt {
//   id: number;
//   created_at: number;
//   updated_at: number;
//   space_id: number;
//   user_id: number;
//   user_answers: NoticeAnswer; // Updated to use new format
//   is_successful: boolean;
// }

// // Quiz Attempts Response (matches backend QueryResponse<NoticeQuizAttempt>)
// export interface QuizAttemptsResponse {
//   total_count: number;
//   items: QuizAttempt[];
// }

// // Quiz Answer Type (matches backend NoticeQuizAnswer)
// export interface NoticeQuizAnswer {
//   id: number;
//   created_at: number;
//   updated_at: number;
//   space_id: number;
//   answers: NoticeAnswer;
// }

// export function spaceSubmitQuizAnswersRequest(
//   answers: NoticeAnswer,
// ): SpaceSubmitQuizAnswersRequest {
//   return {
//     answers,
//   };
// }

// /**
//  * @deprecated The type are deprecated. Use BoosterType from /features/space/types/booster-type.tsx
//  */
// export const BoosterType = {
//   NoBoost: 1,
//   X2: 2,
//   X10: 3,
//   X100: 4,
// } as const;

// /**
//  * @deprecated The type are deprecated. Use BoosterType from /features/space/types/booster-type.tsx
//  */
// export type BoosterType = (typeof BoosterType)[keyof typeof BoosterType];

// export const PublishingScope = {
//   Private: 1,
//   Public: 2,
// } as const;

// export type PublishingScope =
//   (typeof PublishingScope)[keyof typeof PublishingScope];

// // We need to import SpaceType from spaces to avoid duplication
// import { SpaceType } from './spaces';

// // Notice Space Creation
// export interface NoticeSpaceCreateRequest {
//   create_space: {
//     space_type: SpaceType;
//     feed_id: number;
//     user_ids: number[];
//     num_of_redeem_codes: number;
//     started_at: number | null;
//     ended_at: number | null;
//     booster_type: BoosterType | null;
//   };
// }

// export function noticeSpaceCreateRequest(
//   space_type: SpaceType,
//   feed_id: number,
//   user_ids: number[] = [],
//   num_of_redeem_codes: number = 0,
//   started_at: number | null = null,
//   ended_at: number | null = null,
//   booster_type: BoosterType | null = null,
// ): NoticeSpaceCreateRequest {
//   return {
//     create_space: {
//       space_type,
//       feed_id,
//       user_ids,
//       num_of_redeem_codes,
//       started_at,
//       ended_at,
//       booster_type,
//     },
//   };
// }
