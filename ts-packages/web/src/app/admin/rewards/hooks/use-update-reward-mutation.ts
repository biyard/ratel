import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { rewardsKeys } from '@/constants';
import { RewardCondition, RewardPeriod } from '@/features/spaces/rewards/types';

export enum RewardAction {
  PollRespond = 'POLL_RESPOND',
}

export interface UpdateRewardRequest {
  action: RewardAction;
  point: number;
  period: RewardPeriod;
  condition: RewardCondition;
}

export interface RewardResponse {
  reward_action: RewardAction;
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
      queryClient.invalidateQueries({ queryKey: rewardsKeys.global_rewards() });
    },
  });
}
