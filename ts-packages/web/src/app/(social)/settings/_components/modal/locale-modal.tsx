'use client';

import React, { useState } from 'react';
import CheckCircle from '@/assets/icons/check-circle.svg';
import { useTranslations } from 'next-intl';

export interface LocaleModalProps {
  onCancel: () => void;
  onSave: (locale: 'en' | 'ko') => void;
  initialLocale?: 'en' | 'ko';
}

export default function LocaleModal({
  onCancel,
  onSave,
  initialLocale = 'en',
}: LocaleModalProps) {
  const t = useTranslations('Settings');
  const [selected, setSelected] = useState<'en' | 'ko'>(initialLocale);

  const options: { label: string; value: 'en' | 'ko' }[] = [
    { label: 'Korean (한국어)', value: 'ko' },
    { label: 'English (English)', value: 'en' },
  ];

  return (
    <div className="w-[420px]">
      <div className="flex flex-col gap-2 px-5">
        {options.map((opt) => {
          const isSelected = selected === opt.value;
          return (
            <button
              key={opt.value}
              onClick={() => setSelected(opt.value)}
              className={`flex items-center gap-3 w-full text-left px-3 py-3 rounded-[10px] transition-colors border ${isSelected ? ' border-neutral-400 light:border-primary light:bg-primary/10' : 'border-modal-card-border bg-modal-card-bg'}`}
            >
              <span className="w-5 flex items-center justify-center">
                {isSelected && (
                  <CheckCircle className="h-4.5 w-4.5 [&>circle]:hidden [&>path]:stroke-primary" />
                )}
              </span>
              <span className="text-text-primary font-medium text-sm/[16px]">
                {opt.label}
              </span>
            </button>
          );
        })}
      </div>

      <div className="flex items-center justify-end gap-4 mt-6 px-5 pb-2">
        <button
          onClick={onCancel}
          className="px-6 py-[12px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-cancel-button-text/80 transition-colors"
        >
          {t('cancel')}
        </button>
        <button
          onClick={() => onSave(selected)}
          className="px-8 py-[12px] font-bold text-base text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80"
        >
          {t('save')}
        </button>
      </div>
    </div>
  );
}
