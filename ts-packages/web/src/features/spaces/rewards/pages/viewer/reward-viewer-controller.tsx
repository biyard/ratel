import { Space } from '@/features/spaces/types/space';
import { ListRewardsResponse } from '../../types/list-rewards-response';
import { RewardsI18n, useRewardsI18n } from '../../i18n';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import useSpaceRewards from '../../hooks/use-space-rewards';

export class RewardViewerController {
  constructor(
    public spacePk: string,
    public i18n: RewardsI18n,
    public space: Space,
    public rewards: ListRewardsResponse,
  ) {}
}

export function useRewardViewerController(spacePk: string) {
  const i18n = useRewardsI18n();
  const { data: space } = useSpaceById(spacePk);
  const { data: rewards } = useSpaceRewards(spacePk);

  return new RewardViewerController(spacePk, i18n, space, rewards);
}
