import { Space } from '@/features/spaces/types/space';

import { SpaceRewardsI18n, useSpaceRewardsI18n } from '../../i18n';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useSpaceRewards } from '../../hooks';
import { SpaceRewardResponse } from '../../types';

export class RewardViewerController {
  constructor(
    public spacePk: string,
    public i18n: SpaceRewardsI18n,
    public space: Space,
    public spaceRewards: SpaceRewardResponse[],
  ) {}
}

export function useRewardViewerController(spacePk: string) {
  const i18n = useSpaceRewardsI18n();
  const { data: space } = useSpaceById(spacePk);
  const { data: spaceRewards = [] } = useSpaceRewards(spacePk);

  return new RewardViewerController(spacePk, i18n, space, spaceRewards);
}
