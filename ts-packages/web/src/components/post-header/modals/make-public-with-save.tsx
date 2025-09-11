'use client';

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import { useTranslations } from 'next-intl';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  makePublic: () => Promise<void>,
  saveSpace: () => Promise<void>,
) =>
  popup
    .open(
      <MakePublicWithSavingModal
        makePublic={() => {
          void (async () => {
            try {
              await makePublic();
              popup.close();
            } catch (error) {
              console.error('Error making public:', error);
            }
          })();
        }}
        makePublicWithSave={() => {
          void (async () => {
            try {
              await saveSpace();
              await makePublic();
              popup.close();
            } catch (error) {
              console.error('Error saving & publishing space:', error);
            }
          })();
        }}
      />,
    )
    .withoutBackdropClose();

export default function MakePublicWithSavingModal({
  makePublic,
  makePublicWithSave,
}: {
  makePublic: () => void | Promise<void>;
  makePublicWithSave: () => void | Promise<void>;
}) {
  const t = useTranslations('SprintSpace');

  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-bold text-modal-label-text text-[24px]">
        {t('make_public_save_title')}
      </div>

      <div className="text-center font-medium text-desc-text text-[16px]">
        {t('unsaved_notice_line1')}
        <br />
        {t('unsaved_notice_line2')}
        <br />
        {t('unsaved_notice_line3')}
        <br />
        <br />
        {t('make_public_desc_line1')} {t('make_public_desc_line2_prefix')}{' '}
        <span className="font-bold">{t('make_public_desc_line2_strong')}</span>
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          onClick={makePublic}
          className="flex-1/3 border-transparent bg-cancel-button-bg text-cancel-button-text hover:bg-hover"
        >
          {t('just_publish')}
        </Button>
        <Button
          variant="default"
          onClick={makePublicWithSave}
          className="flex-2/3 bg-primary"
        >
          {t('save_and_publish')}
        </Button>
      </div>
    </div>
  );
}
