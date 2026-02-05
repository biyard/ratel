import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { DeleteRewardRequest } from '../types';

export function useDeleteRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['delete-reward'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: DeleteRewardRequest;
    }) => {
      await call<DeleteRewardRequest, void>(
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
