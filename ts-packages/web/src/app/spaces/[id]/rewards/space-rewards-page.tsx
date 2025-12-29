import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { RewardEditorPage } from '@/features/spaces/rewards/pages/editor';
import { RewardViewerPage } from '@/features/spaces/rewards/pages/viewer';

export function SpaceRewardsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    return <RewardEditorPage spacePk={spacePk!} />;
  }

  return <RewardViewerPage spacePk={spacePk!} />;
}
