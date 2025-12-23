import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { RewardEditor } from '@/features/spaces/rewards/components/reward-settings';
import { RewardViewer } from '@/features/spaces/rewards/components/reward-viewer';
import useSpaceRewards from '@/features/spaces/rewards/hooks/use-space-rewards';

export function SpaceRewardsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);
  const { data: rewards } = useSpaceRewards(spacePk!);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    return <RewardEditor spacePk={spacePk!} />;
  }

  return <RewardViewer rewards={rewards} />;
}
