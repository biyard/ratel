'use client';
import { ShapeArrowDown } from '@/components/icons';
import React from 'react';

export type AnswerType =
  | 'single_choice'
  | 'multiple_choice'
  | 'short_answer'
  | 'subjective'
  | 'checkbox'
  | 'dropdown';

export const AnswerTypeLabels: Record<AnswerType, string> = {
  single_choice: 'Single Choice Question',
  multiple_choice: 'Multiple Choice Question',
  short_answer: 'Short Answer Question',
  subjective: 'Subjective Answer Question',
  checkbox: 'Check box',
  dropdown: 'Drop down',
};

export default function AnswerTypeSelect({
  value,
  onChange,
}: {
  value: AnswerType;
  onChange: (val: AnswerType) => void;
}) {
  return (
    <div className="relative w-[300px]">
      <select
        value={value}
        onChange={(e) => onChange(e.target.value as AnswerType)}
        className="appearance-none border border-btn-o focus:border-primary px-5 py-[10.5px] min-w-[260px] font-medium text-[15px]/[22.5px] text-neutral-600 rounded-lg"
      >
        {(Object.entries(AnswerTypeLabels) as [AnswerType, string][])
          .filter(
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
            ([_, label]) =>
              label !== 'Single Choice Question' &&
              label !== 'Multiple Choice Question',
          )
          .map(([key, label]) => (
            <option key={key} value={key}>
              {label}
            </option>
          ))}
      </select>

      <ShapeArrowDown className="pointer-events-none absolute right-[16px] top-1/2 transform -translate-y-1/2 text-neutral-500 w-5 h-5" />
    </div>
  );
}
