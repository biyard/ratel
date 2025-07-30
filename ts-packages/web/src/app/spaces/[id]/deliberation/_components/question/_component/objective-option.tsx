'use client';
import { DialPad2, Remove } from '@/components/icons';
import { Input } from '@/components/ui/input';
import React from 'react';
import { AnswerType } from '../answer-type-select';

export default function ObjectiveOption({
  questionOptions,
  index,
  questionType,
  handleOptionChange,
  handleRemoveOption,
  addOption,
}: {
  questionOptions: string[];
  index: number;
  questionType: AnswerType;
  handleOptionChange: (index: number, value: string) => void;
  handleRemoveOption: (index: number) => void;
  addOption: () => void;
}) {
  return (
    <div className="flex flex-col gap-2">
      {questionOptions.map((opt, idx) => (
        <div
          key={`option-${index}-${idx}`}
          className="flex gap-2.5 items-center"
        >
          <DialPad2 className="w-6 h-6" />

          {questionType == 'checkbox' && (
            <div className="w-6 h-6 rounded-sm border border-c-wg-50" />
          )}

          <Input
            className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 rounded-none"
            type="text"
            placeholder={`Type something...`}
            value={opt}
            onChange={(e) => handleOptionChange(idx, e.target.value)}
          />
          <Remove
            className="cursor-pointer w-5 h-5 stroke-neutral-400 text-neutral-400"
            onClick={() => handleRemoveOption(idx)}
          />
        </div>
      ))}
      <button
        onClick={addOption}
        className="cursor-pointer text-sm text-neutral-500 font-semibold text-left mt-2"
      >
        + Add Option
      </button>
    </div>
  );
}
