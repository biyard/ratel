'use client';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import React from 'react';

export default function LinearScaleViewer({
  answerType,
  title,
  minLabel,
  minValue,
  maxLabel,
  maxValue,
  selected,
  isCompleted,
  index,
  handleSelect,
}: {
  answerType: Answer['answer_type'];
  title: string;
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
      <div className="flex flex-row w-full mt-1.5 mb-3 font-semibold text-base/[22.5px] text-white gap-1">
        <div className="text-[#ff6467]">[Linear Scale]</div>
        <div>{title}</div>
      </div>

      <div className="flex flex-row justify-start gap-5 px-2 items-center">
        <div className="w-10 text-left font-medium text-sm text-neutral-400 break-words">
          {minLabel ?? ''}
        </div>

        {Array.from(
          { length: (maxValue ?? 0) - (minValue ?? 0) + 1 },
          (_, i) => {
            const val = (minValue ?? 0) + i;
            const answer =
              selected && selected.answer ? Number(selected?.answer) + 1 : 0;
            const isChecked =
              answerType === 'linear_scale' &&
              selected &&
              selected.answer &&
              answer === val;

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

        <div className="w-10 text-right font-medium text-sm text-neutral-400 break-words">
          {maxLabel ?? ''}
        </div>
      </div>
    </div>
  );
}

function RadioButton({
  onClick,
  selected,
}: {
  onClick: () => void;
  selected: boolean;
}) {
  return (
    <div className="flex items-center">
      <button
        onClick={onClick}
        className={`w-5 h-5 rounded-full flex items-center justify-center transition-colors ${
          selected
            ? 'bg-[#fcb300] hover:bg-[#fcb300]/90'
            : 'border-2 border-[#6b6b6b] hover:border-white'
        }`}
      >
        {selected && (
          <svg
            className="w-3 h-3 text-black"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clipRule="evenodd"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
