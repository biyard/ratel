export type Answer =
  | { answer_type: 'single_choice'; answer: number | null }
  | { answer_type: 'multiple_choice'; answer: number[] | null }
  | { answer_type: 'short_answer'; answer: string | null }
  | { answer_type: 'subjective'; answer: string | null }
  | { answer_type: 'checkbox'; answer: number[] | null }
  | { answer_type: 'dropdown'; answer: number | null }
  | { answer_type: 'linear_scale'; answer: number | null };

type SurveyType = 1 | 2;

export interface SurveyResponse {
  id: number;
  created_at: number;
  updated_at: number;

  space_id: number;
  user_id: number;
  answers: Answer[];
  survey_type: SurveyType;
}

export interface SurveyResponseCreateRequest {
  respond_answer: {
    answers: Answer[];
    survey_type: SurveyType;
  };
}

export function surveyResponseCreateRequest(
  answer: Answer[],
): SurveyResponseCreateRequest {
  return {
    respond_answer: {
      answers: answer,
      survey_type: 2,
    },
  };
}
