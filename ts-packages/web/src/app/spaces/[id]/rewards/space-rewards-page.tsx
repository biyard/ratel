import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { RewardSettings } from '@/features/spaces/rewards/components/reward-settings';

export function SpaceRewardsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (!space.isAdmin()) {
    return (
      <div className="flex items-center justify-center p-8">
        <p className="text-c-wg-60">
          You do not have permission to access this page.
        </p>
      </div>
    );
  }

  return <RewardSettings spacePk={spacePk!} />;
}
