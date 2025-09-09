'use client';

import React from 'react';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslations } from 'next-intl';

export interface QuizSubmitFormProps {
  onSubmit: () => void;
}

export default function QuizSubmitForm({ onSubmit }: QuizSubmitFormProps) {
  const popup = usePopup();
  const t = useTranslations('NoticeSpace');

  return (
    <div className="w-[500px] flex flex-col mt-2">
      <div className="text-center font-bold text-create-space-label text-[24px] mb-6">
        {t('quiz_submit_title')}
      </div>

      <div className="text-center font-medium text-create-space-desc text-base">
        {t.rich('quiz_submit_desc_line1', {
          b: (chunks) => (
            <span className="text-red-500 font-bold">{chunks}</span>
          ),
        })}
        <br />
        {t.rich('quiz_submit_desc_line2', {
          b: (chunks) => <span className="font-bold">{chunks}</span>,
        })}
        <br />
        <br />
        <span className="text-create-space-desc text-lg font-medium">
          {t('quiz_submit_cta_question')}
        </span>
      </div>

      <div className="flex flex-row justify-end gap-4 mt-8.75">
        <button
          onClick={() => popup.close()}
          className="px-10 py-[14.5px] bg-transparent font-bold text-base light:bg-neutral-300 light:rounded-lg text-neutral-400"
        >
          {t('cancel')}
        </button>
        <button
          onClick={onSubmit}
          className="w-full py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px]"
        >
          {t('submit')}
        </button>
      </div>
    </div>
  );
}
