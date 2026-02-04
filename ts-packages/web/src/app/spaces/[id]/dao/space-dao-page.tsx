import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceDaoEditorPage } from '@/features/spaces/dao/pages/creator/space-dao-editor-page';
import { SpaceDaoViewerPage } from '@/features/spaces/dao/pages/viewer/space-dao-viewer-page';

export default function SpaceDaoPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!spacePk) {
    return null;
  }

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    return <SpaceDaoEditorPage spacePk={spacePk} />;
  }

  return <SpaceDaoViewerPage spacePk={spacePk} />;
}
