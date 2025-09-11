'use client';

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import { useTranslations } from 'next-intl';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  makePublic: () => void,
) => {
  popup
    .open(
      <MakePublicModal
        makePublic={makePublic}
        onCancel={() => popup.close()}
      />,
    )
    .withoutBackdropClose();
};

export default function MakePublicModal({
  makePublic,
  onCancel,
}: {
  makePublic: () => void;
  onCancel: () => void;
}) {
  const t = useTranslations('SprintSpace');

  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-bold text-modal-label-text text-[24px]">
        {t('make_public_title')}
      </div>

      <div className="text-center font-medium text-desc-text text-base">
        {t('make_public_desc_line1')}
        <br />
        {t('make_public_desc_line2_prefix')}{' '}
        <span className="font-bold">{t('make_public_desc_line2_strong')}</span>
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          className="flex-1/3 border-transparent bg-cancel-button-bg text-cancel-button-text hover:bg-hover"
          onClick={onCancel}
        >
          {t('cancel')}
        </Button>
        <Button
          variant="default"
          className="flex-2/3 bg-primary"
          onClick={makePublic}
        >
          {t('make_public')}
        </Button>
      </div>
    </div>
  );
}
