import { useParams } from 'react-router';
import '@/features/spaces/art-nfts/nft-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import SpaceNftEditorPage from '@/features/spaces/art-nfts/pages/editor/space-nft-editor-page';
import { SpacePanelViewerPage } from '@/features/spaces/panels/pages/viewer/space-panel-viewer-page';

export default function SpaceArtNftPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceNftEditorPage spacePk={spacePk} />;
  }

  return <SpacePanelViewerPage spacePk={spacePk} />;
}
