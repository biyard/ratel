'use client';

import {
  CheckboxQuestion,
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';
import { Answer } from '@/lib/api/models/response';
import React from 'react';
import BarChartResponse from '../charts/bar_chart_response';
import PieChartResponse from '../charts/pie_chart_response';

function parseObjectiveAnswers(
  question: SingleChoiceQuestion | MultipleChoiceQuestion | CheckboxQuestion,
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
    } else if (
      a.answer_type === 'multiple_choice' ||
      a.answer_type === 'checkbox'
    ) {
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
  question: SingleChoiceQuestion | MultipleChoiceQuestion | CheckboxQuestion;
  answers: Answer[];
}) {
  const parsed = parseObjectiveAnswers(question, answers);
  const validAnswers = answers
    .filter((a) => a.answer_type === question.answer_type)
    .map((a) => a.answer);

  return (
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="flex items-center justify-between border-b border-neutral-500 pb-2">
        <div className="text-base font-semibold text-neutral-400">
          {question.title}
        </div>
        <div className="text-sm font-medium text-neutral-400">
          {validAnswers.length} Responses
        </div>
      </div>
      <div className="flex flex-col gap-3">
        <BarChartResponse parsed={{ question, ...parsed }} />
        <PieChartResponse parsed={{ question, ...parsed }} />
      </div>
    </div>
  );
}
