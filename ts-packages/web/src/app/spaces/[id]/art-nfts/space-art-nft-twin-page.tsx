import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import SpaceArtNftArtTwinEditorPage from '@/features/spaces/art-nfts/pages/art-twin/editor';
import SpaceArtNftArtTwinViewerPage from '@/features/spaces/art-nfts/pages/art-twin/viewer';

export default function SpaceArtNftArtTwinPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space || !spacePk) {
    throw new Error('Space not found');
  }

  // Show editor if user has SpaceEdit permission, otherwise show viewer

  return (
    <div>
      {space.isAdmin() && <SpaceArtNftArtTwinEditorPage spacePk={spacePk} />}
      <SpaceArtNftArtTwinViewerPage spacePk={spacePk} />
    </div>
  );
}
