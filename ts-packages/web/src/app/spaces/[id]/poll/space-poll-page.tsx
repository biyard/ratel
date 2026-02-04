import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePollEditorPage } from '@/features/spaces/polls/pages/creator/space-poll-editor-page';
import { SpacePollViewerPage } from '@/features/spaces/polls/pages/viewer/space-poll-viewer-page';

export default function SpacePollPage() {
  const { spacePk, pollPk } = useParams<{ spacePk: string; pollPk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpacePollEditorPage spacePk={spacePk} pollPk={pollPk} />;
  }

  return <SpacePollViewerPage spacePk={spacePk} pollPk={pollPk} />;
}
