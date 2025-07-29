'use client';
import RadioButton from '@/components/radio-button';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import React from 'react';
import Wrapper from './_components/wrapper';

export default function LinearScaleViewer({
  answerType,
  title,
  minLabel,
  minValue,
  maxLabel,
  maxValue,
  selected,
  isRequired,
  isCompleted,
  index,
  handleSelect,
}: {
  answerType: Answer['answer_type'];
  title: string;
  isRequired: boolean;
  minLabel?: string;
  minValue?: number;
  maxLabel?: string;
  maxValue?: number;
  selected: Answer;
  isCompleted: boolean;
  index: number;
  handleSelect: (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => void;
}) {
  return (
    <div className="flex flex-col w-full gap-4">
      <Wrapper
        isRequired={isRequired}
        answerType={'linear_scale'}
        isMulti={false}
        title={title}
      />

      <div className="flex flex-row justify-start gap-5 px-2 items-center">
        <div className="w-10 text-center font-medium text-sm text-neutral-400 break-words">
          {minLabel ?? ''}
        </div>

        {Array.from(
          { length: (maxValue ?? 0) - (minValue ?? 0) + 1 },
          (_, i) => {
            const val = (minValue ?? 0) + i;

            const answer =
              selected && selected.answer
                ? Number(selected?.answer) + 1
                : selected && selected.answer === 0
                  ? 1
                  : 0;
            const isChecked =
              answerType === 'linear_scale' && selected && answer === val;

            return (
              <div
                key={`scale-${val}`}
                className="flex flex-col items-center gap-1 w-8"
              >
                <div className="text-sm text-neutral-400 font-medium">
                  {val}
                </div>
                <RadioButton
                  selected={isChecked ? isChecked : false}
                  onClick={() =>
                    !isCompleted && handleSelect(index, val - 1, 'linear_scale')
                  }
                />
              </div>
            );
          },
        )}

        <div className="w-10 text-center font-medium text-sm text-neutral-400 break-words">
          {maxLabel ?? ''}
        </div>
      </div>
    </div>
  );
}
