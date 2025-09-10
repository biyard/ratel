'use client';

import { useTranslations } from 'next-intl';
import React from 'react';

export default function GoPublicPopup({
  onpublic,
  onclose,
}: {
  onpublic: () => void;
  onclose: () => void;
}) {
  const t = useTranslations('Space');
  return (
    <div className="w-[500px] flex flex-col mt-6">
      <div className="text-center font-bold text-modal-label-text text-[24px] mb-6">
        {t('make_public_title')}
      </div>

      <div className="text-center font-medium text-desc-text text-base">
        {t.rich('make_public_desc_line1')}
        <br />
        {t.rich('make_public_desc_line2', {
          b: (chunks) => <span className="font-bold">{chunks}</span>,
        })}
      </div>

      <div className="flex flex-row justify-end gap-4 mt-8.75">
        <button
          onClick={onclose}
          className="min-w-35 px-10 py-[14.5px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:bg-cancel-button-bg/80 hover:text-white light:hover:text-hover"
        >
          {t('cancel')}
        </button>
        <button
          onClick={onpublic}
          className="w-full py-[14.5px] bg-enable-button-bg font-bold text-enable-button-text text-base rounded-[10px]"
        >
          {t('make_public')}
        </button>
      </div>
    </div>
  );
}
