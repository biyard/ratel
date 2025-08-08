'use client';
import { Lock2 } from '@/assets/icons/security';
import { Internet } from '@/assets/icons/internet-script';
import { Button } from '@/components/ui/button';

import React, { useState } from 'react';
import SelectableCardList, {
  CardItemProps,
} from '@/components/selectable-card-list';
import { usePopup } from '@/lib/contexts/popup-service';

// https://www.figma.com/design/YaLSz7dzRingD7CipyaC47/Ratel?node-id=3983-91206&t=riEhxEnpWA7Fr3v9-4

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  onPublish: (type: PublishType) => Promise<void>,
) =>
  popup
    .open(
      <PublishSpaceModal
        onPublish={async (t) => {
          try {
            await onPublish(t);
            popup.close();
          } catch (error) {
            console.log('Error publishing space:', error);
          }
        }}
      />,
    )
    .withoutBackdropClose()
    .withTitle('Publish this space');

const items: CardItemProps[] = [
  {
    value: 'private',
    Icon: <Lock2 className="[&>path]:stroke-neutral-500" />,
    label: 'Private Publish',
    description: 'Only your team members will be able to access this space.',
  },
  {
    value: 'public',
    Icon: (
      <Internet className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500" />
    ),
    label: 'Public Publish',
    description: 'Anyone can access and participate in this space.',
  },
];

export enum PublishType {
  Private = 'private',
  Public = 'public',
}

export default function PublishSpaceModal({
  onPublish,
}: {
  onPublish: (type: PublishType) => Promise<void>;
}) {
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
        Publish
      </Button>
    </div>
  );
}
