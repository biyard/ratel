'use client';
import { ShapeArrowDown } from '@/components/icons';
import React from 'react';

export type AnswerType =
  | 'single_choice'
  | 'multiple_choice'
  | 'short_answer'
  | 'subjective';

export const AnswerTypeLabels: Record<AnswerType, string> = {
  single_choice: 'Single Choice Question',
  multiple_choice: 'Multiple Choice Question',
  short_answer: 'Short Answer Question',
  subjective: 'Subjective Answer Question',
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
        className="appearance-none border border-[#464646] focus:border-primary px-[20px] py-[10.5px] min-w-[260px] font-medium text-[15px]/[22.5px] text-neutral-600 rounded-lg"
      >
        {(Object.entries(AnswerTypeLabels) as [AnswerType, string][]).map(
          ([key, label]) => (
            <option key={key} value={key}>
              {label}
            </option>
          ),
        )}
      </select>

      <ShapeArrowDown className="pointer-events-none absolute right-[16px] top-1/2 transform -translate-y-1/2 text-neutral-500 w-[20px] h-[20px]" />
    </div>
  );
}
