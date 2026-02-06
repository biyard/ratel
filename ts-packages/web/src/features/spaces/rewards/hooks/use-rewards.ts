import { call } from '@/lib/api/ratel/call';
import { ListResponse } from '@/lib/api/ratel/common';
import { logger } from '@/lib/logger';
import { useQuery } from '@tanstack/react-query';
import { rewardsKeys } from '@/constants';
import { RewardAction } from '../types';
import { RewardUserBehavior, RewardCondition, RewardPeriod } from '../types';

export interface Reward {
  reward_behavior: RewardUserBehavior;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export function useRewards(action?: RewardAction) {
  return useQuery({
    queryKey: action
      ? rewardsKeys.rewards_by_action(action)
      : rewardsKeys.rewards(),
    queryFn: async (): Promise<Reward[]> => {
      try {
        const path = `/v3/rewards${action ? `?action=${action.toLowerCase()}` : ''}`;
        const ret: ListResponse<Reward> = await call('GET', path);
        return ret.items;
      } catch (e) {
        logger.error('Failed to fetch global rewards', e);
        throw new Error(e);
      }
    },
  });
}
