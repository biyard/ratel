'use client';
import { ShapeArrowDown } from '@/components/icons';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import React from 'react';

export default function DropdownViewer({
  title,
  isRequired,
  isCompleted,
  selected,
  index,
  options,
  handleSelect,
}: {
  title: string;
  isRequired: boolean;
  isCompleted: boolean;
  selected: Answer;
  index: number;
  options: string[];
  handleSelect: (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => void;
}) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <div className="flex flex-row w-full mt-1.75 mb-3.75 font-semibold text-base/[22.5px] text-white gap-1">
        {isRequired ? (
          <div className="text-[#ff6467]">[Required]</div>
        ) : (
          <div className="text-blue-500">[Optional]</div>
        )}
        <div>{title}</div>
      </div>
      <div className="relative w-65">
        <select
          disabled={isCompleted}
          value={
            selected?.answer_type === 'dropdown' ? (selected.answer ?? '') : ''
          }
          onChange={(e) =>
            handleSelect(index, parseInt(e.target.value, 10), 'dropdown')
          }
          className="w-full appearance-none border border-btn-o focus:border-primary px-5 py-[10.5px] pr-10 font-medium text-[15px]/[22.5px] text-neutral-600 rounded-lg truncate bg-transparent"
        >
          <option className="truncate" value="" disabled>
            Choose
          </option>
          {options?.map((opt, idx) => (
            <option
              key={`dropdown-${index}-${idx}`}
              value={idx}
              className="truncate"
            >
              {opt}
            </option>
          ))}
        </select>

        <div className="pointer-events-none absolute right-4 top-1/2 transform -translate-y-1/2 text-neutral-500">
          <ShapeArrowDown className="w-4 h-4" />
        </div>
      </div>
    </div>
  );
}
