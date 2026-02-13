import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { rewardsKeys } from '@/constants';
import {
  RewardUserBehavior,
  RewardCondition,
  RewardPeriod,
} from '@/features/spaces/rewards/types';
import { Reward } from '@/features/spaces/rewards/hooks/use-rewards';

export interface UpdateRewardRequest {
  behavior: RewardUserBehavior;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export function useUpdateRewardMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: UpdateRewardRequest): Promise<Reward> =>
      call('PATCH', '/m3/rewards', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: rewardsKeys.rewards() });
    },
  });
}
