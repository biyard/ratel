'use client';
import { Lock2 } from '@/assets/icons/security';
import { Internet } from '@/assets/icons/internet-script';
import { Button } from '@/components/ui/button';
import { useMemo, useState } from 'react';
import SelectableCardList, {
  CardItemProps,
} from '@/components/selectable-card-list';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { LoadingIndicator } from '@/app/loading';

export const PublishType = {
  Private: 'private',
  Public: 'public',
} as const;

export type PublishType = (typeof PublishType)[keyof typeof PublishType];

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  onPublish: (type: PublishType) => Promise<void>,
  title: string,
) =>
  popup
    .open(
      <PublishSpaceModal
        onPublish={async (publishType) => {
          try {
            await onPublish(publishType);
            popup.close();
          } catch (error) {
            console.error('Error publishing space:', error);
          }
        }}
      />,
    )
    .withTitle(title)
    .withoutBackdropClose();

export default function PublishSpaceModal({
  onPublish,
}: {
  onPublish: (type: PublishType) => Promise<void>;
}) {
  const { t } = useTranslation('SpacePublishModal');

  const items: CardItemProps[] = useMemo(
    () => [
      {
        value: PublishType.Private,
        Icon: <Lock2 className="[&>path]:stroke-neutral-500" />,
        label: t('private_publish_label'),
        description: t('private_publish_description'),
      },
      {
        value: PublishType.Public,
        Icon: (
          <Internet className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500" />
        ),
        label: t('public_publish_label'),
        description: t('public_publish_description'),
      },
    ],
    [t],
  );

  const [selectedType, setSelectedType] = useState<PublishType | null>(null);
  const [isLoading, setLoading] = useState(false);
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
        disabled={!selectedType || isLoading}
        onClick={async () => {
          if (selectedType) {
            setLoading(true);
            try {
              await onPublish(selectedType);
            } catch (error) {
              console.error(error);
            } finally {
              setLoading(false);
            }
          }
        }}
      >
        {!isLoading ? (
          t('button_publish')
        ) : (
          <LoadingIndicator className="h-12" />
        )}
      </Button>
    </div>
  );
}
