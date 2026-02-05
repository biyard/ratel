import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { rewardsKeys } from '@/constants';
import { UpdateRewardRequest } from './use-update-reward-mutation';
import { Reward } from '@/features/spaces/rewards/hooks/use-rewards';

export function useCreateGlobalRewardMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: UpdateRewardRequest): Promise<Reward> =>
      call('POST', '/m3/rewards', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: rewardsKeys.rewards() });
    },
  });
}
