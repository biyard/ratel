'use client';
import { ShapeArrowDown } from '@/components/icons';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import React from 'react';
import Wrapper from './_components/wrapper';
import { useTranslations } from 'next-intl';

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
  const t = useTranslations('PollSpace');
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Wrapper
        isRequired={isRequired}
        answerType={'dropdown'}
        isMulti={false}
        title={title}
      />
      <div className="relative w-65">
        <select
          disabled={isCompleted}
          value={
            selected?.answer_type === 'dropdown' ? (selected.answer ?? '') : ''
          }
          onChange={(e) =>
            handleSelect(index, parseInt(e.target.value, 10), 'dropdown')
          }
          className="w-full appearance-none bg-input-box-bg border border-input-box-border focus:border-primary px-5 py-[10.5px] pr-10 font-medium text-[15px]/[22.5px] text-neutral-600 light:text-text-primary rounded-lg truncate"
        >
          <option className="truncate" value="" disabled>
            {t('choose')}
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
