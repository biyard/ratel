import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';

import { call } from '@/lib/api/ratel/call';
import { FeatureType } from '../types/feature-type';
import { RewardAction } from '../types/reward-type';
import { RewardPeriod } from '../types/reward-period';
import { RewardCondition } from '../types';

export const rewardConfigKeys = {
  all: ['reward-config'] as const,
  byFeature: (feature: FeatureType) => ['reward-config', feature] as const,
};

export interface RewardConfigItem {
  reward_action: RewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export interface ListRewardConfigResponse {
  items: RewardConfigItem[];
  bookmark: string | null;
}

export function getOption(feature?: FeatureType) {
  const queryParams = feature ? `?feature=${feature.toLowerCase()}` : '';
  return {
    queryKey: feature
      ? rewardConfigKeys.byFeature(feature)
      : rewardConfigKeys.all,
    queryFn: async () => {
      const response = await call<void, ListRewardConfigResponse>(
        'GET',
        `/v3/rewards${queryParams}`,
      );
      return response;
    },
    staleTime: Infinity,
    refetchOnWindowFocus: false,
  };
}

export default function useRewardConfig(
  feature?: FeatureType,
): UseSuspenseQueryResult<ListRewardConfigResponse> {
  return useSuspenseQuery(getOption(feature));
}
