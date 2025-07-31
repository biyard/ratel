'use client';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Answer } from '@/lib/api/models/response';
import React from 'react';
import Wrapper from './_components/wrapper';

export default function SubjectiveViewer({
  title,
  isRequired,
  answerType,
  selected,
  index,
  isCompleted,

  handleInput,
}: {
  title: string;
  isRequired: boolean;
  answerType: Answer['answer_type'];
  selected: Answer;
  index: number;
  isCompleted: boolean;

  handleInput: (
    qIdx: number,
    value: string,
    type: 'short_answer' | 'subjective',
  ) => void;
}) {
  return (
    <div className="flex flex-col w-full gap-[10px]">
      <Wrapper
        isRequired={isRequired}
        answerType={selected?.answer_type}
        isMulti={false}
        title={title}
      />
      {answerType === 'short_answer' ? (
        <Input
          type="text"
          placeholder="Please share your opinion."
          className="bg-neutral-800 border border-neutral-700 text-base text-white placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={
            selected?.answer_type === 'short_answer'
              ? (selected.answer ?? '')
              : ''
          }
          onChange={(e) => handleInput(index, e.target.value, 'short_answer')}
          disabled={isCompleted}
        />
      ) : (
        <Textarea
          rows={7}
          placeholder="Please share your opinion."
          className="bg-neutral-800 min-h-[185px] border border-neutral-700 text-base text-white placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={
            selected?.answer_type === 'subjective'
              ? (selected.answer ?? '')
              : ''
          }
          onChange={(e) => handleInput(index, e.target.value, 'subjective')}
          disabled={isCompleted}
        />
      )}
    </div>
  );
}
