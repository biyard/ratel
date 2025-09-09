import { Answer } from '@/lib/api/models/response';
import { useTranslations } from 'next-intl';
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
  const t = useTranslations('PollSpace');

  let choiceLabel = '';
  if (
    answerType === 'single_choice' ||
    (answerType === 'checkbox' && !isMulti)
  ) {
    choiceLabel = t('single_choice');
  } else if (answerType === 'checkbox' && isMulti) {
    choiceLabel = t('multiple_choice');
  }

  return (
    <div>
      <div className="flex flex-row w-full mt-1.75 mb-3.75 font-semibold text-base/[22.5px] text-foreground gap-1">
        <div className={isRequired ? 'text-[#ff6467]' : 'text-blue-500'}>
          [{isRequired ? t('required') : t('optional')}]
        </div>

        {choiceLabel && <div className="text-blue-500">[{choiceLabel}]</div>}

        <div>{title}</div>
      </div>
    </div>
  );
}
