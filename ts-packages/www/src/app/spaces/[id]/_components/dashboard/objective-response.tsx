'use client';

import {
  CheckboxQuestion,
  DropdownQuestion,
  LinearScaleQuestion,
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';
import { Answer } from '@/lib/api/models/response';
import BarChartResponse from '../charts/bar-chart-response';
import PieChartResponse from '../charts/pie-chart-response';
import { useTranslation } from 'react-i18next';

function parseObjectiveAnswers(
  question:
    | SingleChoiceQuestion
    | MultipleChoiceQuestion
    | CheckboxQuestion
    | LinearScaleQuestion
    | DropdownQuestion,
  answers: Answer[],
) {
  const optionCount =
    question.answer_type !== 'linear_scale'
      ? question.options.length
      : question.max_value;
  const counts = new Array(optionCount).fill(0);

  const filtered = answers.filter(
    (a) => a.answer_type === question.answer_type,
  );

  filtered.forEach((a) => {
    if (
      a.answer_type === 'single_choice' ||
      a.answer_type === 'dropdown' ||
      a.answer_type === 'linear_scale'
    ) {
      if (typeof a.answer === 'number') counts[a.answer]++;
    } else if (
      a.answer_type === 'multiple_choice' ||
      a.answer_type === 'checkbox'
    ) {
      if (a.answer) {
        a.answer.forEach((i) => counts[i]++);
      }
    }
  });

  const total = filtered.length;

  const options =
    question.answer_type !== 'linear_scale'
      ? question.options.map((label, idx) => ({
          label,
          count: counts[idx],
          ratio: total > 0 ? (counts[idx] / total) * 100 : 0,
        }))
      : Array.from(
          { length: (question.max_value ?? 0) - (question.min_value ?? 0) + 1 },
          (_, idx) => {
            const number = (question.min_value ?? 0) + idx;
            return {
              label: 'option ' + String(number),
              count: counts[idx],
              ratio: total > 0 ? (counts[idx] / total) * 100 : 0,
            };
          },
        );

  return {
    totalParticipants: total,
    options,
  };
}

export default function ObjectiveResponse({
  question,
  answers,
}: {
  question:
    | SingleChoiceQuestion
    | MultipleChoiceQuestion
    | CheckboxQuestion
    | DropdownQuestion
    | LinearScaleQuestion;
  answers: Answer[];
}) {
  const { t } = useTranslation('PollSpace');
  const parsed = parseObjectiveAnswers(question, answers);
  const validAnswers = answers
    .filter((a) => a.answer_type === question.answer_type && a.answer != null)
    .map((a) => a.answer);

  return (
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="flex items-center justify-between border-b border-input-box-border pb-2">
        <div className="text-base font-semibold text-neutral-400">
          {question.title}
        </div>
        <div className="text-sm font-medium text-neutral-400">
          {validAnswers.length} {t('responses')}
        </div>
      </div>
      {validAnswers.length != 0 && (
        <div className="flex flex-col gap-3">
          <BarChartResponse parsed={{ question, ...parsed }} />
          <PieChartResponse parsed={{ question, ...parsed }} />
        </div>
      )}
    </div>
  );
}
