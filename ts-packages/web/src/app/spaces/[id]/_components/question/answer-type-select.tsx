'use client';
import React from 'react';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

export type AnswerType =
  | 'single_choice'
  | 'multiple_choice'
  | 'short_answer'
  | 'subjective'
  | 'checkbox'
  | 'dropdown'
  | 'linear_scale';

export const AnswerTypeLabels: Record<AnswerType, string> = {
  single_choice: 'Single Choice Question',
  multiple_choice: 'Multiple Choice Question',
  short_answer: 'Short Answer Question',
  subjective: 'Subjective Answer Question',
  checkbox: 'Check box',
  dropdown: 'Drop down',
  linear_scale: 'Linear Scale',
};

export default function AnswerTypeSelect({
  value,
  onChange,
}: {
  value: AnswerType;
  onChange: (val: AnswerType) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="border-input-box-border bg-card-bg focus:border-primary px-5 py-[10.5px] w-[260px] max-mobile:!w-full font-medium text-[15px]/[22.5px] text-neutral-600 rounded-lg focus:ring-primary !h-full">
        <SelectValue placeholder="Select an answer type" />
      </SelectTrigger>
      <SelectContent>
        {(Object.entries(AnswerTypeLabels) as [AnswerType, string][])
          .filter(
            ([key]) => key !== 'single_choice' && key !== 'multiple_choice',
          )
          .map(([key, label]) => (
            <SelectItem key={key} value={key} className="text-neutral-600">
              {label}
            </SelectItem>
          ))}
      </SelectContent>
    </Select>
  );
}
