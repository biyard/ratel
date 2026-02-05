import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { rewardsKeys } from '@/constants';
import {
  RewardAction,
  RewardCondition,
  RewardPeriod,
} from '@/features/spaces/rewards/types';
import { RewardResponse } from '@/features/spaces/rewards/hooks/use-rewards';

export interface UpdateRewardRequest {
  action: RewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export function useUpdateRewardMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: UpdateRewardRequest): Promise<RewardResponse> =>
      call('PATCH', '/m3/rewards', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: rewardsKeys.rewards() });
    },
  });
}
