import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceBoardsEditorPage } from '@/features/spaces/boards/pages/creator/space-boards-editor-page';
import { SpaceBoardsViewerPage } from '@/features/spaces/boards/pages/viewer/space-boards-viewer-page';

export default function SpaceBoardsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceBoardsEditorPage spacePk={spacePk} />;
  }

  return <SpaceBoardsViewerPage spacePk={spacePk} />;
}
