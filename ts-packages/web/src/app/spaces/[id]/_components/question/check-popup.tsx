'use client';
import { useTranslations } from 'next-intl';
import React from 'react';

export default function CheckPopup({
  onContinue,
  onClose,
}: {
  onContinue: () => void;
  onClose: () => void;
}) {
  const t = useTranslations('PollSpace');
  return (
    <div className="w-full max-w-[450px] px-[20px]">
      <div className="flex flex-col gap-[40px]">
        <div className="font-semibold text-base text-desc-text">
          {t('check_desc')}
        </div>

        <div className="flex flex-row w-full justify-end items-center gap-[10px]">
          <div
            className="cursor-pointer flex flex-row w-fit h-fit px-[14px] py-[8px] rounded-lg bg-primary hover:opacity-50 font-semibold text-sm text-[#000203]"
            onClick={() => onContinue()}
          >
            {t('confirm')}
          </div>
          <div
            className="cursor-pointer flex flex-row w-fit h-fit px-[14px] py-[8px] rounded-lg bg-neutral-500 hover:opacity-50 font-semibold text-sm text-white"
            onClick={() => onClose()}
          >
            {t('cancel')}
          </div>
        </div>
      </div>
    </div>
  );
}
