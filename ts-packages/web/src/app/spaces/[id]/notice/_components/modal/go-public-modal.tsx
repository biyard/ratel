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
    <div className="w-[500px] flex flex-col">
      {/* Header */}
      <div className="text-center font-bold text-create-space-label text-[24px] mb-6 mt-2">
        {t('go_public_title')}
      </div>

      {/* Body */}
      <div className="text-center font-medium text-create-space-desc text-base mb-8">
        {t('go_public_desc')}
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        {/* Left button - transparent background like space selection form */}
        <button
          onClick={onCancel}
          className="flex-1 py-[14.5px] bg-transparent font-bold light:bg-neutral-300 text-white text-base rounded-[10px] hover:bg-neutral-800 transition-colors"
        >
          {t('cancel')}
        </button>

        {/* Right button - primary background */}
        <button
          onClick={onGoPublic}
          className="flex-1 py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          {t('go_public')}
        </button>
      </div>
    </div>
  );
}
