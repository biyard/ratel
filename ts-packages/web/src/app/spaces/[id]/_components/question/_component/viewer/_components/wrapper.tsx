import { Answer } from '@/lib/api/models/response';
import React from 'react';

export default function Wrapper({
  isRequired,
  answerType,
  isMulti,
  title,
}: {
  isRequired: boolean;
  answerType: Answer['answer_type'];
  isMulti?: boolean;
  title: string;
}) {
  return (
    <div>
      <div className="flex flex-row w-full mt-1.75 mb-3.75 font-semibold text-base/[22.5px] text-white gap-1">
        {isRequired ? (
          <div className="text-[#ff6467]">[Required]</div>
        ) : (
          <div className="text-blue-500">[Optional]</div>
        )}
        <div className="text-blue-500">
          {answerType === 'single_choice' ||
          (answerType === 'checkbox' && !isMulti)
            ? '[Single Choice]'
            : answerType === 'checkbox' && isMulti
              ? '[Multiple Choice]'
              : ''}
        </div>
        <div>{title}</div>
      </div>
    </div>
  );
}
