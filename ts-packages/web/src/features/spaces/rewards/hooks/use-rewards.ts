import { call } from '@/lib/api/ratel/call';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';
import { useInfiniteQuery } from '@tanstack/react-query';
import { rewardsKeys } from '@/constants';
import { FeatureType } from '../types/feature-type';
import { RewardAction, RewardCondition, RewardPeriod } from '../types';

export interface RewardResponse {
  reward_action: RewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export function useRewards(feature?: FeatureType) {
  return useInfiniteQuery({
    queryKey: feature
      ? rewardsKeys.rewards_by_feature(feature)
      : rewardsKeys.rewards(),
    queryFn: async ({ pageParam }): Promise<ListResponse<RewardResponse>> => {
      try {
        const params = new URLSearchParams();
        if (pageParam) {
          params.set('bookmark', pageParam);
        }
        params.set('limit', '20');

        const queryString = params.toString();
        const path = `/v3/rewards${queryString ? `?${queryString}` : ''}`;

        const ret: ListResponse<RewardResponse> = await call('GET', path);
        return ret;
      } catch (e) {
        logger.error('Failed to fetch global rewards', e);
        throw new Error(e);
      }
    },
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage) => lastPage.bookmark ?? undefined,
  });
}
