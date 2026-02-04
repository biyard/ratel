import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import SpaceNftPreviewPage from '@/features/spaces/art-nfts/pages/space-nft-preview-page';

export default function SpaceArtNftPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  // if (space.isAdmin()) {
  //   // Edit Mode
  //   return <SpaceNftEditorPage spacePk={spacePk} />;
  // }

  return <SpaceNftPreviewPage spacePk={spacePk} />;
}
