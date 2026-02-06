import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type UpdateSpaceDaoIncentiveRequest = {
  incentive_sk: string;
  incentive_distributed: boolean;
};

export function useUpdateSpaceDaoIncentiveMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-dao-incentive'],
    mutationFn: async ({
      spacePk,
      incentiveSk,
      incentiveDistributed,
    }: {
      spacePk: string;
      incentiveSk: string;
      incentiveDistributed: boolean;
    }) => {
      return call<UpdateSpaceDaoIncentiveRequest, void>(
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
        queryKey: spaceDaoKeys.incentiveBase(spacePk),
      });
    },
  });
}
