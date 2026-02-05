import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { UpdateRewardRequest } from '@/app/admin/rewards/hooks/use-update-reward-mutation';
import { SpaceRewardResponse } from '../types';

export function useUpdateRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: UpdateRewardRequest;
    }) => {
      const response = await call<UpdateRewardRequest, SpaceRewardResponse>(
        'PUT',
        `/v3/spaces/${encodeURIComponent(spacePk)}/rewards`,
        req,
      );
      return new SpaceRewardResponse(response);
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.rewards(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
