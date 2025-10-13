'use client';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@radix-ui/react-select';
import { TFunction } from 'i18next';

import { SurveyQuestion } from '@/types/survey-type';

type LinearScaleQuestion = Extract<
  SurveyQuestion,
  { answer_type: 'linear_scale' }
>;

interface LinearScaleQuestionEditorProps {
  t: TFunction<'Survey', undefined>;
  question: LinearScaleQuestion;
  onUpdate: (newQuestion: LinearScaleQuestion) => void;
}

export default function LinearScaleQuestionEditor({
  t,
  question,
  onUpdate,
}: LinearScaleQuestionEditorProps) {
  const { content } = question;

  const handleMaxValueChange = (val: number) => {
    onUpdate({ ...question, content: { ...content, max_value: val } });
  };

  const handleLabelChange = (target: 'min' | 'max', label: string) => {
    if (target === 'min') {
      onUpdate({ ...question, content: { ...content, min_label: label } });
    } else {
      onUpdate({ ...question, content: { ...content, max_label: label } });
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-row items-center gap-2">
        <div className="bg-input-box-bg border border-input-box-border rounded-md px-3 py-2 text-text-primary text-sm text-start min-w-20 ">
          {content.min_value}
        </div>
        <span className="text-text-primary text-sm">~</span>
        <Select
          value={content.max_value.toString()}
          onValueChange={(value) => {
            const parsed = parseInt(value, 10);
            if (!isNaN(parsed)) {
              handleMaxValueChange(parsed);
            }
          }}
        >
          <SelectTrigger className="w-full max-w-70">
            <SelectValue placeholder={t('choose')} />
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
        <div className="flex flex-row items-center justify-start gap-5 w-full mb-3">
          <span className="font-medium text-text-primary text-sm w-5 text-center">
            {content.min_value}
          </span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            placeholder={t('label_hint')}
            value={content.min_label || ''}
            onChange={(e) => {
              const val = e.target.value;
              handleLabelChange('min', val);
            }}
          />
        </div>

        <div className="flex flex-row items-center justify-start gap-5 w-full">
          <span className="font-medium text-text-primary text-sm w-5 text-center">
            {content.max_value}
          </span>
          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            placeholder={t('label_hint')}
            value={content.max_label || ''}
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
