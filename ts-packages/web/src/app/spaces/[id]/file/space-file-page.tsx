import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceFileEditorPage } from '@/features/spaces/files/pages/creator/space-file-editor-page';
import { SpaceFileViewerPage } from '@/features/spaces/files/pages/viewer/space-file-viewer-page';

export default function SpaceFilePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceFileEditorPage spacePk={spacePk} />;
  }

  return <SpaceFileViewerPage spacePk={spacePk} />;
}
