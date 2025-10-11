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
  is_multi: boolean; // Corresponds to the is_multi field
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
  | { answer_type: 'single_choice'; content: ChoiceQuestion }
  | { answer_type: 'multiple_choice'; content: ChoiceQuestion }
  | { answer_type: 'short_answer'; content: SubjectiveQuestion }
  | { answer_type: 'subjective'; content: SubjectiveQuestion }
  | { answer_type: 'checkbox'; content: CheckboxQuestion }
  | { answer_type: 'dropdown'; content: DropdownQuestion }
  | { answer_type: 'linear_scale'; content: LinearScaleQuestion };

export type SurveyAnswer =
  | { answer_type: 'single_choice'; answer?: number }
  | { answer_type: 'multiple_choice'; answer?: number[] }
  | { answer_type: 'short_answer'; answer?: string }
  | { answer_type: 'subjective'; answer?: string }
  | { answer_type: 'checkbox'; answer?: number[] }
  | { answer_type: 'dropdown'; answer?: number }
  | { answer_type: 'linear_scale'; answer?: number };
