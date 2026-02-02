import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePollsEditorPage } from '@/features/spaces/polls/pages/creator/list/space-polls-editor-page';
import { SpacePollsViewerPage } from '@/features/spaces/polls/pages/viewer/list/space-polls-viewer-page';

export default function SpacePollsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpacePollsEditorPage spacePk={spacePk} />;
  }

  return <SpacePollsViewerPage spacePk={spacePk} />;
}
