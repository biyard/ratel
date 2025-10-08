'use client';

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation, Trans } from 'react-i18next';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  saveSpace: () => Promise<void>,
  handleNext: () => void,
  title: string,
) =>
  popup
    .open(
      <UnsaveAlertModal
        onSave={saveSpace}
        handleNext={() => {
          popup.close();
          handleNext();
        }}
      />,
    )
    .withTitle(title)
    .withoutBackdropClose();

export default function UnsaveAlertModal({
  handleNext,
  onSave,
}: {
  handleNext: () => void;
  onSave: () => Promise<void> | void;
}) {
  const { t } = useTranslation('SpaceUnsaveModal');
  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-medium text-desc-text text-[16px]">
        <Trans
          i18nKey="description"
          ns="SpaceUnsaveModal"
          components={{
            b: <span className="font-bold" />,
            br: <br />,
          }}
        />
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          onClick={() => {
            handleNext();
          }}
          className="flex-1/3 border-transparent bg-cancel-button-bg text-cancel-button-text hover:bg-hover"
        >
          {t('button_skip')}
        </Button>
        <Button
          variant="default"
          onClick={async () => {
            try {
              await onSave();
              handleNext();
            } catch (error) {
              console.error('Error saving space:', error);
            }
          }}
          className="flex-2/3 bg-primary"
        >
          {t('button_save')}
        </Button>
      </div>
    </div>
  );
}
