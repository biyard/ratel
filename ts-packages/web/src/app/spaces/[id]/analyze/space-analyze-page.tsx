import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePollAnalyzePage } from '@/features/spaces/polls/pages/analyze/space-poll-analyze-page';

export default function SpaceAnalyzePage() {
  const { spacePk, pollPk } = useParams<{ spacePk: string; pollPk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpacePollAnalyzePage spacePk={spacePk} pollPk={pollPk} />;
  }

  return <></>;
}
