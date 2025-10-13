/**
 * Represents a choice option in a survey question.
 */
export interface ChoiceQuestion {
  title: string;
  description?: string; // Option<String> maps to optional string
  image_url?: string;
  options: string[]; // Vec<String> maps to string[]
  is_required?: boolean; // Option<bool> maps to optional boolean
}

/**
 * Represents a question that allows text input (short answer or subjective).
 */
export interface SubjectiveQuestion {
  title: string;
  description: string;
  is_required?: boolean;
}

/**
 * Represents a question with checkboxes.
 */
export interface CheckboxQuestion {
  title: string;
  description?: string;
  image_url?: string;
  options: string[];
  is_multi?: boolean; // Corresponds to the is_multi field
  is_required?: boolean;
}

/**
 * Represents a dropdown question.
 */
export interface DropdownQuestion {
  title: string;
  description?: string;
  image_url?: string;
  options: string[];
  is_required?: boolean;
}

/**
 * Represents a linear scale (rating) question.
 */
export interface LinearScaleQuestion {
  title: string;
  description?: string;
  image_url?: string;
  min_value: number; // i64 maps to number in TypeScript
  max_value: number; // i64 maps to number in TypeScript
  min_label: string;
  max_label: string;
  is_required?: boolean;
}

// --- SurveyQuestion Enum Mapping ---

/**
 * The core SurveyQuestion type, mapped from the Rust enum.
 * It uses a discriminated union based on the 'answer_type' tag,
 * which is derived from the #[serde(tag = "answer_type")] attribute.
 * #[serde(rename_all = "snake_case")] ensures the type names are snake_cased.
 */
export type SurveyQuestion =
  | { answer_type: SurveyAnswerType.SingleChoice; content: ChoiceQuestion }
  | { answer_type: SurveyAnswerType.MultipleChoice; content: ChoiceQuestion }
  | { answer_type: SurveyAnswerType.ShortAnswer; content: SubjectiveQuestion }
  | { answer_type: SurveyAnswerType.Subjective; content: SubjectiveQuestion }
  | { answer_type: SurveyAnswerType.Checkbox; content: CheckboxQuestion }
  | { answer_type: SurveyAnswerType.Dropdown; content: DropdownQuestion }
  | { answer_type: SurveyAnswerType.LinearScale; content: LinearScaleQuestion };

export type SurveyAnswer =
  | { answer_type: SurveyAnswerType.SingleChoice; answer?: number }
  | { answer_type: SurveyAnswerType.MultipleChoice; answer?: number[] }
  | { answer_type: SurveyAnswerType.ShortAnswer; answer?: string }
  | { answer_type: SurveyAnswerType.Subjective; answer?: string }
  | { answer_type: SurveyAnswerType.Checkbox; answer?: number[] }
  | { answer_type: SurveyAnswerType.Dropdown; answer?: number }
  | { answer_type: SurveyAnswerType.LinearScale; answer?: number };

export enum SurveyAnswerType {
  SingleChoice = 'single_choice',
  MultipleChoice = 'multiple_choice',
  ShortAnswer = 'short_answer',
  Subjective = 'subjective',
  Checkbox = 'checkbox',
  Dropdown = 'dropdown',
  LinearScale = 'linear_scale',
}
