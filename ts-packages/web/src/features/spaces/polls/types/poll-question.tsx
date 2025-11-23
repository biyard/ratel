/**
 * Represents a choice option in a survey question.
 */
export interface ChoiceQuestion {
  title: string;
  description?: string; // Option<String> maps to optional string
  image_url?: string;
  options: string[]; // Vec<String> maps to string[]
  is_required?: boolean; // Option<bool> maps to optional boolean
  allow_other?: boolean;
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

export enum SurveyAnswerType {
  SingleChoice = 'single_choice',
  MultipleChoice = 'multiple_choice',
  ShortAnswer = 'short_answer',
  Subjective = 'subjective',
  Checkbox = 'checkbox',
  Dropdown = 'dropdown',
  LinearScale = 'linear_scale',
}

/**
 * SurveyQuestions
 */

export interface SingleChoiceQuestionType extends ChoiceQuestion {
  answer_type: SurveyAnswerType.SingleChoice;
}
export interface MultipleChoiceQuestionType extends ChoiceQuestion {
  answer_type: SurveyAnswerType.MultipleChoice;
}
export interface ShortAnswerQuestionType extends SubjectiveQuestion {
  answer_type: SurveyAnswerType.ShortAnswer;
}
export interface SubjectiveQuestionType extends SubjectiveQuestion {
  answer_type: SurveyAnswerType.Subjective;
}
export interface CheckboxQuestionType extends CheckboxQuestion {
  answer_type: SurveyAnswerType.Checkbox;
}
export interface DropdownQuestionType extends DropdownQuestion {
  answer_type: SurveyAnswerType.Dropdown;
}
export interface LinearScaleQuestionType extends LinearScaleQuestion {
  answer_type: SurveyAnswerType.LinearScale;
}

export type ObjectiveQuestionUnion =
  | SingleChoiceQuestionType
  | MultipleChoiceQuestionType
  | CheckboxQuestionType
  | DropdownQuestionType
  | LinearScaleQuestionType;

export type SubjectiveQuestionUnion =
  | ShortAnswerQuestionType
  | SubjectiveQuestionType;

export type SurveyQuestion = PollQuestion;
export type SurveyAnswer = PollAnswer;

export type PollQuestion = ObjectiveQuestionUnion | SubjectiveQuestionUnion;

export type PollAnswer =
  | {
      answer_type: SurveyAnswerType.SingleChoice;
      answer?: number;
      other?: string;
    }
  | {
      answer_type: SurveyAnswerType.MultipleChoice;
      answer?: number[];
      other?: string;
    }
  | { answer_type: SurveyAnswerType.ShortAnswer; answer?: string }
  | { answer_type: SurveyAnswerType.Subjective; answer?: string }
  | { answer_type: SurveyAnswerType.Checkbox; answer?: number[] }
  | { answer_type: SurveyAnswerType.Dropdown; answer?: number }
  | { answer_type: SurveyAnswerType.LinearScale; answer?: number };

export type SurveyQuestionWithAnswer = {
  [T in SurveyAnswerType]: {
    answer_type: T;
    question: Extract<PollQuestion, { answer_type: T }>;
    answer?: Extract<PollAnswer, { answer_type: T }>;
  };
}[SurveyAnswerType];

export interface BaseSubjectiveSummary {
  type: SurveyAnswerType.ShortAnswer | SurveyAnswerType.Subjective;
  total_count: number;
  answers: Record<string, number>; // (answer, count)
}

export interface ShortAnswerSummary extends BaseSubjectiveSummary {
  answer_type: SurveyAnswerType.ShortAnswer;
}

export interface SubjectiveSummary extends BaseSubjectiveSummary {
  answer_type: SurveyAnswerType.Subjective;
}

export interface BaseObjectiveSummary {
  answer_type:
    | SurveyAnswerType.SingleChoice
    | SurveyAnswerType.MultipleChoice
    | SurveyAnswerType.Checkbox
    | SurveyAnswerType.Dropdown
    | SurveyAnswerType.LinearScale;
  total_count: number;
  answers: Record<number, number>; // (option_idx or scale_value, count)
}

export interface SingleChoiceSummary extends BaseObjectiveSummary {
  answer_type: SurveyAnswerType.SingleChoice;
}
export interface MultipleChoiceSummary extends BaseObjectiveSummary {
  answer_type: SurveyAnswerType.MultipleChoice;
}
export interface CheckboxSummary extends BaseObjectiveSummary {
  answer_type: SurveyAnswerType.Checkbox;
}
export interface DropdownSummary extends BaseObjectiveSummary {
  answer_type: SurveyAnswerType.Dropdown;
}
export interface LinearScaleSummary extends BaseObjectiveSummary {
  answer_type: SurveyAnswerType.LinearScale;
}

// The final discriminated union type
export type SurveySummary =
  | SingleChoiceSummary
  | MultipleChoiceSummary
  | ShortAnswerSummary
  | SubjectiveSummary
  | CheckboxSummary
  | DropdownSummary
  | LinearScaleSummary;

export function createEmptyAnswer(type: SurveyAnswerType): PollAnswer {
  switch (type) {
    case SurveyAnswerType.SingleChoice:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.MultipleChoice:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.ShortAnswer:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.Subjective:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.Checkbox:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.Dropdown:
      return { answer_type: type, answer: undefined };
    case SurveyAnswerType.LinearScale:
      return { answer_type: type, answer: undefined };
    default:
      throw new Error(`Unsupported SurveyAnswerType: ${type}`);
  }
}

export function createDefaultQuestion(type: SurveyAnswerType): PollQuestion {
  switch (type) {
    case SurveyAnswerType.SingleChoice:
    case SurveyAnswerType.MultipleChoice:
      return {
        answer_type: type,
        title: '',
        options: [''],
        is_required: false,
      };
    case SurveyAnswerType.Checkbox:
      return {
        answer_type: type,
        title: '',
        description: '',
        options: [''],
        is_multi: false,
        is_required: false,
      };
    case SurveyAnswerType.Dropdown:
      return {
        answer_type: type,
        title: '',
        is_required: false,
        options: [''],
      };
    case SurveyAnswerType.LinearScale:
      return {
        answer_type: type,
        title: '',
        min_value: 1,
        max_value: 5,
        min_label: '',
        max_label: '',
        is_required: false,
      };
    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective:
      return {
        answer_type: type,
        title: '',
        description: '',
        is_required: false,
      };
    default:
      throw new Error(`Unsupported answer type: ${type}`);
  }
}
