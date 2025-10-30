import { useParams } from 'react-router';
import '@/features/spaces/art-nfts/nft-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';

export default function SpaceArtNftArtTwinPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  return <>Art Twin</>;
}
