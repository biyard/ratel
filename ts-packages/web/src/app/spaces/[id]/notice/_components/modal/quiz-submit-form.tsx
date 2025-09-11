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
      <div className="text-center font-bold text-text-primary text-[24px] mb-6">
        {t('quiz_submit_title')}
      </div>

      <div className="text-center font-medium text-desc-text text-base">
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
        <span className="text-desc-text text-lg font-medium">
          {t('quiz_submit_cta_question')}
        </span>
      </div>

      <div className="flex flex-row justify-end gap-4 mt-8.75">
        <button
          onClick={() => popup.close()}
          className="min-w-[150px] px-10 py-[14.5px] font-bold text-base bg-cancel-button-bg text-cancel-button-text hover:bg-hover rounded-2.5"
        >
          {t('cancel')}
        </button>
        <button
          onClick={onSubmit}
          className="w-full py-[14.5px] bg-submit-button-bg font-bold text-submit-button-text text-base rounded-[10px] hover:bg-submit-button-bg/80"
        >
          {t('submit')}
        </button>
      </div>
    </div>
  );
}
