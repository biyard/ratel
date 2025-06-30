'use client';
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
    <select
      value={value}
      onChange={(e) => onChange(e.target.value as AnswerType)}
      className="border border-neutral-700 focus:border-primary px-2 py-1 font-medium text-sm text-neutral-500 rounded-sm"
    >
      {(Object.entries(AnswerTypeLabels) as [AnswerType, string][]).map(
        ([key, label]) => (
          <option key={key} value={key}>
            {label}
          </option>
        ),
      )}
    </select>
  );
}
