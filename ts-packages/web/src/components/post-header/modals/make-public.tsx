'use client';

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import { useTranslations } from 'next-intl';
import { LoadingIndicator } from '@/app/loading';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  makePublic: () => Promise<void>,
  title: string,
) => {
  popup
    .open(
      <MakePublicModal
        makePublic={makePublic}
        onCancel={() => popup.close()}
      />,
    )
    .withTitle(title)
    .withoutBackdropClose();
};

export default function MakePublicModal({
  makePublic,
  onCancel,
}: {
  makePublic: () => Promise<void>;
  onCancel: () => void;
}) {
  const t = useTranslations('SpaceMakePublicModal');
  const [loading, setLoading] = React.useState(false);
  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-medium text-desc-text text-base">
        {t.rich('description', {
          br: () => <br />,
          b: (chunks) => <span className="font-bold">{chunks}</span>,
        })}
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          className="flex-1/3 border-transparent bg-cancel-button-bg text-cancel-button-text hover:bg-hover"
          onClick={onCancel}
        >
          {t('button_cancel')}
        </Button>
        <Button
          variant="default"
          className="flex-2/3 bg-primary"
          disabled={loading}
          onClick={async () => {
            setLoading(true);
            try {
              await makePublic();
            } catch (error) {
              console.error('Failed to make space public:', error);

              setLoading(false);
            }
          }}
        >
          {!loading ? <>{t('button_make_public')}</> : <LoadingIndicator />}
        </Button>
      </div>
    </div>
  );
}
