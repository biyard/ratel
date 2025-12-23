import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { call } from '@/lib/api/ratel/call';
import { ListAvailableRewardsResponse } from '../types/reward-config';

export type RewardFeatureType = 'poll' | 'board';

export const availableRewardsKeys = {
  all: ['available-rewards'] as const,
  byFeature: (feature: RewardFeatureType) =>
    ['available-rewards', feature] as const,
};

export function getOption(feature?: RewardFeatureType) {
  const queryParams = feature ? `?feature=${feature}` : '';
  return {
    queryKey: feature
      ? availableRewardsKeys.byFeature(feature)
      : availableRewardsKeys.all,
    queryFn: async () => {
      const response = await call<void, ListAvailableRewardsResponse>(
        'GET',
        `/v3/rewards${queryParams}`,
      );
      return new ListAvailableRewardsResponse(response);
    },
    staleTime: Infinity, // These are static configs, no need to refetch
    refetchOnWindowFocus: false,
  };
}

export default function useAvailableRewards(
  feature?: RewardFeatureType,
): UseSuspenseQueryResult<ListAvailableRewardsResponse> {
  return useSuspenseQuery(getOption(feature));
}
