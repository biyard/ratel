'use client';

import React from 'react';
import {
  CheckboxQuestion,
  DropdownQuestion,
  LinearScaleQuestion,
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';

type ParsedOption = {
  label: string;
  count: number;
  ratio: number;
};

type ParsedResult = {
  question:
    | SingleChoiceQuestion
    | MultipleChoiceQuestion
    | CheckboxQuestion
    | DropdownQuestion
    | LinearScaleQuestion;
  totalParticipants: number;
  options: ParsedOption[];
};

export default function BarChartResponse({ parsed }: { parsed: ParsedResult }) {
  const { options } = parsed;

  return (
    <>
      {options.map((opt, idx) => (
        <div key={idx} className="flex items-center gap-3">
          <div
            className="max-w-[100px] w-full text-sm font-medium text-neutral-400 truncate overflow-hidden whitespace-nowrap"
            title={opt.label}
          >
            {opt.label}
          </div>
          <div className="flex-1 h-4 bg-neutral-700 rounded-full overflow-hidden">
            <div
              className="h-full rounded-full bg-neutral-400 transition-[width] duration-500 ease-out"
              style={{ width: `${opt.ratio}%` }}
            ></div>
          </div>
          <div className="w-[80px] text-sm text-left text-neutral-400">
            {opt.count} ({opt.ratio.toFixed(1)}%)
          </div>
        </div>
      ))}
    </>
  );
}
