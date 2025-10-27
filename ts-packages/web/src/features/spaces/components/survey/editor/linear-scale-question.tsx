import { Input } from '@/components/ui/input';
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from '@/components/ui/select';

import { PollQuestion } from '@/features/spaces/polls/types/poll-question';
import { I18nFunction } from '..';

type LinearScaleQuestion = Extract<
  PollQuestion,
  { answer_type: 'linear_scale' }
>;

interface LinearScaleQuestionEditorProps {
  t: I18nFunction;
  question: LinearScaleQuestion;
  onUpdate: (newQuestion: LinearScaleQuestion) => void;
}

export default function LinearScaleQuestionEditor({
  t,
  question,
  onUpdate,
}: LinearScaleQuestionEditorProps) {
  const handleMaxValueChange = (val: number) => {
    onUpdate({ ...question, max_value: val });
  };

  const handleLabelChange = (target: 'min' | 'max', label: string) => {
    if (target === 'min') {
      onUpdate({ ...question, min_label: label });
    } else {
      onUpdate({ ...question, max_label: label });
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-row gap-2 items-center">
        <div className="py-2 px-3 text-sm rounded-md border bg-input-box-bg border-input-box-border text-text-primary text-start min-w-20">
          {question.min_value}
        </div>
        <span className="text-sm text-text-primary">~</span>
        <Select
          value={question.max_value.toString()}
          onValueChange={(value) => {
            const parsed = parseInt(value, 10);
            if (!isNaN(parsed)) {
              handleMaxValueChange(parsed);
            }
          }}
        >
          <SelectTrigger className="w-full h-[50px] max-w-30">
            <SelectValue placeholder={t('dropdown_select_placeholder')} />
          </SelectTrigger>
          <SelectContent>
            {Array.from({ length: 9 }, (_, i) => i + 2).map(
              (option, optIndex) => (
                <SelectItem
                  key={`dropdown-${optIndex}`}
                  value={option.toString()}
                >
                  {option}
                </SelectItem>
              ),
            )}
          </SelectContent>
        </Select>
      </div>

      <div className="flex flex-col justify-start items-start w-full">
        <div className="flex flex-row gap-5 justify-start items-center mb-3 w-full">
          <span className="w-5 text-sm font-medium text-center text-text-primary">
            {question.min_value}
          </span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            placeholder={t('option_input_placeholder')}
            value={question.min_label || ''}
            onChange={(e) => {
              const val = e.target.value;
              handleLabelChange('min', val);
            }}
          />
        </div>

        <div className="flex flex-row gap-5 justify-start items-center w-full">
          <span className="w-5 text-sm font-medium text-center text-text-primary">
            {question.max_value}
          </span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            placeholder={t('option_input_placeholder')}
            value={question.max_label || ''}
            onChange={(e) => {
              const val = e.target.value;
              handleLabelChange('max', val);
            }}
          />
        </div>
      </div>
    </div>
  );
}
