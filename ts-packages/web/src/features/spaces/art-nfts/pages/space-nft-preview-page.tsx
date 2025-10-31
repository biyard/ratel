import { ArtworkPost } from '@/app/(social)/threads/[id]/_components/thread';
import Card from '@/components/card';
import useSuspensePostById from '@/features/posts/hooks/use-post';
import { spacePkToPostPk } from '@/features/spaces/utils/partition-key-utils';

export default function SpaceNftPreviewPage({ spacePk }: { spacePk: string }) {
  const {
    data: { post, artwork_metadata },
  } = useSuspensePostById(spacePkToPostPk(spacePk));
  return (
    <Card variant="secondary">
      <ArtworkPost post={post} artworkMetadata={artwork_metadata} />
    </Card>
  );
}
