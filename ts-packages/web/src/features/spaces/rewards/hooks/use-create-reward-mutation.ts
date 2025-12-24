import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { CreateRewardRequest } from '../types/create-reward-request';
import { SpaceRewardResponse } from '../types/space-reward-response';

export function useCreateRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: CreateRewardRequest;
    }) => {
      const response = await call<CreateRewardRequest, SpaceRewardResponse>(
        'POST',
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
