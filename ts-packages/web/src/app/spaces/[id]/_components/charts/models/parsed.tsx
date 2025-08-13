import {
  CheckboxQuestion,
  DropdownQuestion,
  LinearScaleQuestion,
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';

type ParsedOption = {
  label: string;
  count: number;
  ratio: number;
};

export type ParsedResult = {
  question:
    | SingleChoiceQuestion
    | MultipleChoiceQuestion
    | CheckboxQuestion
    | DropdownQuestion
    | LinearScaleQuestion;
  totalParticipants: number;
  options: ParsedOption[];
};
