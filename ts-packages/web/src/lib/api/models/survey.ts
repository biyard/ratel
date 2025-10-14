import { SurveyStatus } from '@/features/deliberation-space/utils/deliberation.spaces.v3';

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

export interface NewSurveyCreateRequest {
  survey_pk: string | undefined;
  started_at: number;
  ended_at: number;
  status: SurveyStatus;
  questions: Question[];
}

export type Question =
  | SingleChoiceQuestion
  | MultipleChoiceQuestion
  | ShortAnswerQuestion
  | SubjectiveQuestion
  | CheckboxQuestion
  | DropdownQuestion
  | LinearScaleQuestion;

export interface LinearScaleQuestion {
  answer_type: 'linear_scale';
  title: string;
  description?: string;
  image_url?: string;
  min_value: number;
  max_value: number;
  min_label: string;
  max_label: string;
  is_required?: boolean;
}

export interface DropdownQuestion {
  answer_type: 'dropdown';
  title: string;
  description?: string;
  image_url?: string;
  options: string[];
  is_required?: boolean;
}

export interface CheckboxQuestion {
  answer_type: 'checkbox';
  title: string;
  description?: string;
  image_url?: string;
  options: string[];
  is_multi?: boolean;
  is_required?: boolean;
}
export interface SingleChoiceQuestion {
  answer_type: 'single_choice';
  title: string;
  description?: string;
  image_url?: string;
  options: string[];
  is_required?: boolean;
}

export interface MultipleChoiceQuestion {
  answer_type: 'multiple_choice';
  title: string;
  description?: string;
  options: string[];
  is_required?: boolean;
}

export interface ShortAnswerQuestion {
  answer_type: 'short_answer';
  title: string;
  description: string;
  is_required?: boolean;
}

export interface SubjectiveQuestion {
  answer_type: 'subjective';
  title: string;
  description: string;
  is_required?: boolean;
}
