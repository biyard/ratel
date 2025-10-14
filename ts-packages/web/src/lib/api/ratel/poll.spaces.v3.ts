import {
  SurveyQuestion,
  SurveyAnswer,
  SurveySummary,
} from '@/types/survey-type';
import { call } from './call';
import { TimeRange } from '@/types/time-range';
import { SpaceCommon } from '@/types/space-common';

export function getPollSpace(spacePk: string): Promise<PollSpaceResponse> {
  return call('GET', `/v3/spaces/poll/${encodeURIComponent(spacePk)}`);
}

export function updatePollSpace(
  spacePk: string,
  title: string,
  htmlContents: string,
  timeRange: TimeRange,
  questions: SurveyQuestion[],
) {
  return call('PUT', `/v3/spaces/poll/${encodeURIComponent(spacePk)}`, {
    title,
    html_contents: htmlContents,
    time_range: timeRange,
    questions,
  });
}

export interface PollSurveySummariesResponse {
  created_at: string;
  summaries: SurveySummary[];
}
export function getPollSurveySummaries(
  spacePk: string,
): Promise<PollSurveySummariesResponse> {
  return call('GET', `/v3/spaces/poll/${encodeURIComponent(spacePk)}/summary`);
}

// export function listPollSurveyAnswers(
//   spacePk: string,
//   bookmark?: string,
//   limit?: number,
// ) {
//   const params = new URLSearchParams();
//   if (bookmark) params.append('bookmark', bookmark);
//   if (limit) params.append('limit', limit.toString());

//   const queryString = params.toString();
//   const url = `/v3/spaces/poll/${encodeURIComponent(spacePk)}/response${
//     queryString ? `?${queryString}` : ''
//   }`;

//   return call('GET', url);
// }

export function submitPollSurveyResponse(
  spacePk: string,
  answers: SurveyAnswer[],
) {
  return call(
    'POST',
    `/v3/spaces/poll/${encodeURIComponent(spacePk)}/response`,
    {
      answers,
    },
  );
}

export interface PollSpaceResponse extends SpaceCommon {
  user_response_count: number;

  questions: SurveyQuestion[];
  my_response?: SurveyAnswer[];
}
