import { Lock2 } from '@/assets/icons/security';
import { Internet } from '@/assets/icons/internet-script';
import { Button } from '@/components/ui/button';
import { useMemo, useState } from 'react';
import SelectableCardList, {
  CardItemProps,
} from '@/components/selectable-card-list';
import { LoadingIndicator } from '@/app/loading';
import { useSpaceLayoutI18n } from '../space-layout-i18n';

export const PublishType = {
  Private: 'private',
  Public: 'public',
} as const;

export type PublishType = (typeof PublishType)[keyof typeof PublishType];

// export const openModal = (
//   popup: ReturnType<typeof usePopup>,
//   onPublish: (type: PublishType) => Promise<void>,
//   title: string,
// ) =>
//   popup
//     .open(
//       <PublishSpaceModal
//         onPublish={async (publishType) => {
//           try {
//             await onPublish(publishType);
//             popup.close();
//           } catch (error) {
//             console.error('Error publishing space:', error);
//           }
//         }}
//       />,
//     )
//     .withTitle(title)
//     .withoutBackdropClose();

export default function PublishSpaceModal({
  onPublish,
}: {
  onPublish: (type: PublishType) => Promise<void>;
}) {
  const i18n = useSpaceLayoutI18n();

  const items: CardItemProps[] = useMemo(
    () => [
      {
        value: PublishType.Private,
        Icon: <Lock2 className="[&>path]:stroke-neutral-500" />,
        label: i18n.publish_modal_private,
        description: i18n.publish_modal_private_desc,
      },
      {
        value: PublishType.Public,
        Icon: (
          <Internet className="[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500" />
        ),
        label: i18n.publish_modal_public,
        description: i18n.publish_modal_public_desc,
      },
    ],
    [i18n],
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
        data-testid="publish-space-modal-btn"
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
          i18n.publish_modal_button_publish
        ) : (
          <LoadingIndicator className="h-12" />
        )}
      </Button>
    </div>
  );
}
