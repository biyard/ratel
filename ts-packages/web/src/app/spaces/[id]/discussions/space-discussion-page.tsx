import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceDiscussionEditorPage } from '@/features/spaces/discussions/pages/creator/space-discussion-editor-page';
import { SpaceDiscussionViewerPage } from '@/features/spaces/discussions/pages/viewer/space-discussion-viewer-page';

export default function SpaceDiscussionPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceDiscussionEditorPage spacePk={spacePk} />;
  }

  return <SpaceDiscussionViewerPage spacePk={spacePk} />;
}
