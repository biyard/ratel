import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { DeleteSpaceRewardRequest } from '../types';

export function useDeleteSpaceRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-space-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: DeleteSpaceRewardRequest;
    }) => {
      await call<DeleteSpaceRewardRequest, void>(
        'DELETE',
        `/v3/spaces/${encodeURIComponent(spacePk)}/rewards`,
        req,
      );
      return { spacePk, req };
    },

    onSettled: async (_data, _error, { spacePk }) => {
      const qk = spaceKeys.rewards(spacePk);
      await qc.invalidateQueries({ queryKey: qk });
    },
  });
}
