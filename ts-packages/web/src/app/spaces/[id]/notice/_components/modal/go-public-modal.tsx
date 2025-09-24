'use client';

import { useTranslations } from 'next-intl';
import React from 'react';

export interface GoPublicModalProps {
  onCancel: () => void;
  onGoPublic: () => void;
}

export default function GoPublicModal({
  onCancel,
  onGoPublic,
}: GoPublicModalProps) {
  const t = useTranslations('NoticeSpace');
  return (
    <div className="w-[500px] max-tablet:w-full flex flex-col">
      {/* Header */}
      <div className="text-center font-bold text-text-primary text-[24px] mb-6 max-tablet:mt-6 mt-2">
        {t('go_public_title')}
      </div>

      {/* Body */}
      <div className="text-center font-medium text-desc-text text-base mb-8">
        {t('go_public_desc')}
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        {/* Left button - transparent background like space selection form */}
        <button
          onClick={onCancel}
          className="flex-1 py-[14.5px] bg-cancel-button-bg font-bold text-cancel-button-text text-base rounded-[10px] hover:bg-hover transition-colors"
        >
          {t('cancel')}
        </button>

        {/* Right button - primary background */}
        <button
          onClick={onGoPublic}
          className="flex-1 py-[14.5px] bg-submit-button-bg font-bold text-submit-button-text text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          {t('go_public')}
        </button>
      </div>
    </div>
  );
}
