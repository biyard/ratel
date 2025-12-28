import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { UpdateGlobalRewardRequest, GlobalRewardResponse } from '../types';
import { QK_GLOBAL_REWARDS } from './use-list-global-rewards';

export function useUpdateGlobalRewardMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (
      request: UpdateGlobalRewardRequest,
    ): Promise<GlobalRewardResponse> => call('PATCH', '/m3/rewards', request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [QK_GLOBAL_REWARDS] });
    },
  });
}
