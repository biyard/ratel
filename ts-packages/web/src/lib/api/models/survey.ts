export interface Survey {
  id: number;
  created_at: number;
  updated_at: number;

  space_id: number;
  status: number;
  started_at: number;
  ended_at: number;
  questions: Question[];
}

export interface SurveyCreateRequest {
  started_at: number;
  ended_at: number;
  questions: Question[];
}

export type Question =
  | SingleChoiceQuestion
  | MultipleChoiceQuestion
  | ShortAnswerQuestion
  | SubjectiveQuestion;

export interface SingleChoiceQuestion {
  answer_type: 'single_choice';
  title: string;
  description?: string;
  options: string[];
}

export interface MultipleChoiceQuestion {
  answer_type: 'multiple_choice';
  title: string;
  description?: string;
  options: string[];
}

export interface ShortAnswerQuestion {
  answer_type: 'short_answer';
  title: string;
  description: string;
}

export interface SubjectiveQuestion {
  answer_type: 'subjective';
  title: string;
  description: string;
}
