'use client';
import { Lock2 } from '@/assets/icons/security';
import { Internet } from '@/assets/icons/internet-script';
import { Button } from '@/components/ui/button';
import React, { useMemo, useState } from 'react';
import SelectableCardList, {
  CardItemProps,
} from '@/components/selectable-card-list';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslations } from 'next-intl';

export enum PublishType {
  Private = 'private',
  Public = 'public',
}

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  onPublish: (type: PublishType) => Promise<void>,
  title: string,
) =>
  popup
    .open(
      <PublishSpaceModal
        onPublish={async (t) => {
          try {
            await onPublish(t);
            popup.close();
          } catch (error) {
            console.error('Error publishing space:', error);
          }
        }}
      />,
    )
    .withoutBackdropClose()
    .withTitle(title);

export default function PublishSpaceModal({
  onPublish,
}: {
  onPublish: (type: PublishType) => Promise<void>;
}) {
  const t = useTranslations('SprintSpace');

  const items: CardItemProps[] = useMemo(
    () => [
      {
        value: PublishType.Private,
        Icon: <Lock2 className="[&>path]:stroke-neutral-500" />,
        label: `${t('private')} ${t('publish')}`,
        description: t('private_desc'),
      },
      {
        value: PublishType.Public,
        Icon: (
          <Internet className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500" />
        ),
        label: `${t('public')} ${t('publish')}`,
        description: t('public_desc'),
      },
    ],
    [t],
  );

  const [selectedType, setSelectedType] = useState<PublishType | null>(null);

  return (
    <div className="max-w-110 flex flex-col gap-6">
      <SelectableCardList
        items={items}
        value={selectedType as string}
        onSelect={(value) => setSelectedType(value as PublishType)}
      />
      <Button
        variant="default"
        className="h-12 bg-primary"
        disabled={!selectedType}
        onClick={() => selectedType && onPublish(selectedType)}
      >
        {t('publish')}
      </Button>
    </div>
  );
}
