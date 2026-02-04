import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePollAnalyzesPage } from '@/features/spaces/polls/pages/analyze/list/space-poll-analyzes-page';

export default function SpaceAnalyzesPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpacePollAnalyzesPage spacePk={spacePk} />;
  }

  return <></>;
}
