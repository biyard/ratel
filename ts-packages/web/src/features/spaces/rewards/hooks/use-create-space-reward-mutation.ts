import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { CreateSpaceRewardRequest, SpaceRewardResponse } from '../types';

export function useCreateSpaceRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-space-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: CreateSpaceRewardRequest;
    }) => {
      const response = await call<
        CreateSpaceRewardRequest,
        SpaceRewardResponse
      >('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/rewards`, req);
      return new SpaceRewardResponse(response);
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.rewards(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
