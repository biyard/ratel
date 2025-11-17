import React, { useId, useRef } from 'react';
import {
  LinearScaleQuestionType,
  ObjectiveQuestionUnion,
  SurveyAnswerType,
} from '@/features/spaces/polls/types/poll-question';
import { DialPad2, Remove } from '@/components/icons';
import { Input } from '@/components/ui/input';
import { I18nFunction } from '..';
import { Checkbox } from '@/components/checkbox/checkbox';

export type ObjectiveQuestionWithOptions = Exclude<
  ObjectiveQuestionUnion,
  LinearScaleQuestionType
>;
interface ObjectiveQuestionEditorProps {
  t: I18nFunction;
  question: ObjectiveQuestionWithOptions;
  onUpdate: (newQuestion: ObjectiveQuestionWithOptions) => void;
}

const OTHER_LABEL = 'Others';

export default function ObjectiveQuestionEditor({
  t,
  question,
  onUpdate,
}: ObjectiveQuestionEditorProps) {
  const inputRefs = useRef<(HTMLInputElement | null)[]>([]);
  const checkboxId = useId();

  const handleUpdateOption = (idx: number, value: string) => {
    const newOptions = [...question.options];
    newOptions[idx] = value;
    onUpdate({ ...question, options: newOptions });
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const allowOtherChecked = !!(question as any).allow_other;

  const handleChangeAllowedOtherOption = (allowed: boolean) => {
    const hasOther = question.options.includes(OTHER_LABEL);

    let newOptions = question.options;
    if (allowed && !hasOther) {
      newOptions = [...question.options, OTHER_LABEL];
    } else if (!allowed && hasOther) {
      newOptions = question.options.filter((opt) => opt !== OTHER_LABEL);
    }

    if (
      question.answer_type === SurveyAnswerType.SingleChoice ||
      question.answer_type === SurveyAnswerType.MultipleChoice
    ) {
      onUpdate({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ...(question as any),
        options: newOptions,
        allow_other: allowed,
      } as ObjectiveQuestionWithOptions);
    } else {
      onUpdate({
        ...question,
        options: newOptions,
      });
    }
  };

  const handleAddOption = (focusAfterAdd = false) => {
    const otherIndex = question.options.indexOf(OTHER_LABEL);

    const insertIndex =
      otherIndex === -1 ? question.options.length : otherIndex;

    const newOptions = [
      ...question.options.slice(0, insertIndex),
      '',
      ...question.options.slice(insertIndex),
    ];

    onUpdate({ ...question, options: newOptions });

    if (focusAfterAdd) {
      setTimeout(() => {
        inputRefs.current[insertIndex]?.focus();
      }, 0);
    }
  };

  const handleRemoveOption = (idx: number) => {
    if (question.options.length <= 1) {
      return;
    }

    const newOptions = question.options.filter((_, index) => index !== idx);
    onUpdate({ ...question, options: newOptions });

    setTimeout(() => {
      const lastIndex = newOptions.length - 1;
      inputRefs.current[lastIndex]?.focus();
    }, 0);
  };

  const getLastEditableIndex = () => {
    let i = question.options.length - 1;
    while (i >= 0 && question.options[i] === OTHER_LABEL) {
      i--;
    }
    return i;
  };

  const handleOptionKeyDown = (
    idx: number,
    e: React.KeyboardEvent<HTMLInputElement>,
  ) => {
    if (e.key === 'Tab' && !e.shiftKey) {
      const lastEditableIndex = getLastEditableIndex();
      const isLastEditable = idx === lastEditableIndex;

      if (isLastEditable) {
        e.preventDefault();
        handleAddOption(true);
      }
    }
  };

  return (
    <div className="flex flex-col gap-2">
      {question.options.map((opt, idx) => (
        <div key={`option--${idx}`} className="flex gap-2.5 items-center">
          <DialPad2 className="w-6 h-6" />

          {question.answer_type === SurveyAnswerType.Checkbox && (
            <div className="w-6 h-6 rounded-sm border border-c-wg-50 light:border-[#e5e5e5] max-tablet:hidden" />
          )}

          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
            type="text"
            placeholder={t('option_input_placeholder')}
            value={opt}
            onChange={(e) => handleUpdateOption(idx, e.target.value)}
            onKeyDown={(e) => handleOptionKeyDown(idx, e)}
            disabled={opt === OTHER_LABEL}
            ref={(el) => {
              inputRefs.current[idx] = el;
            }}
          />
          {opt !== OTHER_LABEL && (
            <Remove
              className="w-5 h-5 cursor-pointer stroke-neutral-400 text-neutral-400"
              onClick={() => handleRemoveOption(idx)}
            />
          )}
        </div>
      ))}
      {question.answer_type === SurveyAnswerType.SingleChoice && (
        <Checkbox
          id={`allow-other-${checkboxId}`}
          value={allowOtherChecked}
          onChange={(v) => handleChangeAllowedOtherOption(!!v)}
        >
          <span className="font-medium text-text-primary">
            {t('allowed_other_option')}
          </span>
        </Checkbox>
      )}

      <button
        onClick={() => handleAddOption(true)}
        className="mt-2 text-sm font-semibold text-left cursor-pointer text-neutral-500"
      >
        + {t('add_option_button_label')}
      </button>
    </div>
  );
}
