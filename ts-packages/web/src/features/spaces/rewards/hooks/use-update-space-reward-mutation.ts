import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { UpdateSpaceRewardRequest } from '../types';
import { SpaceRewardResponse } from '../types';

export function useUpdateSpaceRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: UpdateSpaceRewardRequest;
    }) => {
      const response = await call<
        UpdateSpaceRewardRequest,
        SpaceRewardResponse
      >('PUT', `/v3/spaces/${encodeURIComponent(spacePk)}/rewards`, req);
      return new SpaceRewardResponse(response);
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.rewards(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
