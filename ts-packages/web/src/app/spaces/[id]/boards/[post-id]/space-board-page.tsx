import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { logger } from '@/lib/logger';
import { SpaceBoardsEditorDetailPage } from '@/features/spaces/boards/pages/creator/detail/space-boards-editor-detail-page';
import { SpaceBoardsViewerDetailPage } from '@/features/spaces/boards/pages/viewer/detail/space-boards-viewer-detail-page';

export default function SpaceBoardPage() {
  const { spacePk, postPk } = useParams<{ spacePk: string; postPk: string }>();
  const { data: space } = useSpaceById(spacePk);

  logger.debug('space pk, post pk: ', spacePk, postPk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceBoardsEditorDetailPage spacePk={spacePk} postPk={postPk} />;
  }

  return <SpaceBoardsViewerDetailPage spacePk={spacePk} postPk={postPk} />;
}
