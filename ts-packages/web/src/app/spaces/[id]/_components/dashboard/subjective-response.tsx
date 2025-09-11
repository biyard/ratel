import { Answer } from '@/lib/api/models/response';
import {
  ShortAnswerQuestion,
  SubjectiveQuestion,
} from '@/lib/api/models/survey';
import { useTranslations } from 'next-intl';
import React from 'react';

export default function SubjectiveResponse({
  question,
  answers,
}: {
  question: ShortAnswerQuestion | SubjectiveQuestion;
  answers: Answer[];
}) {
  const t = useTranslations('PollSpace');
  const validAnswers = answers
    .filter(
      (a) =>
        a.answer_type === question.answer_type && a.answer !== '' && a.answer,
    )
    .map((a) => a.answer as string);

  return (
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="flex items-center justify-between border-b border-divider pb-2">
        <div className="text-base font-semibold text-neutral-400">
          {question.title}
        </div>
        <div className="text-sm font-medium text-neutral-400">
          {validAnswers.length} {t('responses')}
        </div>
      </div>

      <div className="flex flex-col gap-2">
        {validAnswers.map((answer, idx) => (
          <div
            key={idx}
            className="px-4 py-2 bg-input-box-bg border border-input-box-border rounded-md text-sm text-text-primary whitespace-pre-wrap"
          >
            {answer}
          </div>
        ))}
      </div>
    </div>
  );
}
