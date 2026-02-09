import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceIncentiveKeys } from '@/constants';

export type UpdateSpaceIncentiveRequest = {
  incentive_sk: string;
  incentive_distributed: boolean;
};

export function useUpdateSpaceIncentiveMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-incentive-incentive'],
    mutationFn: async ({
      spacePk,
      incentiveSk,
      incentiveDistributed,
    }: {
      spacePk: string;
      incentiveSk: string;
      incentiveDistributed: boolean;
    }) => {
      return call<UpdateSpaceIncentiveRequest, void>(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/user`,
        {
          incentive_sk: incentiveSk,
          incentive_distributed: incentiveDistributed,
        },
      );
    },
    onSuccess: async (_, { spacePk }) => {
      await qc.invalidateQueries({
        queryKey: spaceIncentiveKeys.incentiveBase(spacePk),
      });
    },
  });
}
