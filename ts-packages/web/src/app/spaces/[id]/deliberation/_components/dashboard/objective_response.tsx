'use client';

import {
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';
import { Answer } from '@/lib/api/models/response';
import React from 'react';
import BarChart from '../charts/bar_chart';

function parseObjectiveAnswers(
  question: SingleChoiceQuestion | MultipleChoiceQuestion,
  answers: Answer[],
) {
  const optionCount = question.options.length;
  const counts = new Array(optionCount).fill(0);

  const filtered = answers.filter(
    (a) => a.answer_type === question.answer_type,
  );

  filtered.forEach((a) => {
    if (a.answer_type === 'single_choice') {
      if (typeof a.answer === 'number') counts[a.answer]++;
    } else if (a.answer_type === 'multiple_choice') {
      a.answer.forEach((i) => counts[i]++);
    }
  });

  const total = filtered.length;

  const options = question.options.map((label, idx) => ({
    label,
    count: counts[idx],
    ratio: total > 0 ? (counts[idx] / total) * 100 : 0,
  }));

  return {
    totalParticipants: total,
    options,
  };
}

export default function ObjectiveResponse({
  question,
  answers,
}: {
  question: SingleChoiceQuestion | MultipleChoiceQuestion;
  answers: Answer[];
}) {
  const parsed = parseObjectiveAnswers(question, answers);

  return (
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="text-base font-semibold text-neutral-400">
        {question.title}
      </div>
      <div className="flex flex-col gap-3">
        <BarChart parsed={{ question, ...parsed }} />
      </div>
    </div>
  );
}
