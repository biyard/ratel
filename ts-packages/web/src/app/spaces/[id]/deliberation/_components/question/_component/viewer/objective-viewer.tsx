'use client';
import React from 'react';
import Image from 'next/image';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Answer } from '@/lib/api/models/response';
import { Question } from '@/lib/api/models/survey';
import Wrapper from './_components/wrapper';

export default function ObjectiveViewer({
  answerType,
  isMulti,
  title,
  imageUrl,
  options,
  selected,
  selectedIndexes,
  index,
  isRequired,
  isCompleted,
  handleSelect,
}: {
  answerType: Answer['answer_type'];
  isRequired: boolean;
  isMulti?: boolean;
  title: string;
  imageUrl?: string;
  options?: string[];
  selected: Answer;
  selectedIndexes: number[];
  index: number;

  isCompleted: boolean;
  handleSelect: (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => void;
}) {
  return (
    <>
      <Wrapper
        isRequired={isRequired}
        answerType={answerType}
        isMulti={isMulti}
        title={title}
      />
      {imageUrl ? (
        <Image
          width={700}
          height={280}
          className="object-contain max-h-70 w-fit rounded-lg"
          src={imageUrl}
          alt={title || 'Question Title'}
        />
      ) : (
        <></>
      )}
      <div className="flex flex-col gap-2">
        {options?.map((opt, idx) => {
          let isChecked = selectedIndexes.includes(idx);

          if (!isChecked) {
            isChecked =
              answerType === 'single_choice'
                ? selected?.answer === idx
                : selectedIndexes.includes(idx);
          }

          return (
            <div
              key={`${answerType}-${index}-${idx}`}
              className="flex flex-row w-full h-fit justify-start items-center gap-3"
            >
              <div className="w-4.5 h-4.5">
                <CustomCheckbox
                  checked={isChecked}
                  onChange={() => handleSelect(index, idx, answerType)}
                  disabled={isCompleted}
                />
              </div>
              <div className="font-normal text-neutral-300 text-[15px]/[22.5px]">
                {opt}
              </div>
            </div>
          );
        })}
      </div>
    </>
  );
}
