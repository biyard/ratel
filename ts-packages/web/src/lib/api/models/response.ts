export type Answer =
  | { answer_type: 'single_choice'; answer: number }
  | { answer_type: 'multiple_choice'; answer: number[] }
  | { answer_type: 'short_answer'; answer: string }
  | { answer_type: 'subjective'; answer: string };

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
