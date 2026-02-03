import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type UpdateSpaceDaoRewardRequest = {
  reward_sks: string[];
  reward_distributed: boolean;
};

export function useUpdateSpaceDaoRewardMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-dao-reward'],
    mutationFn: async ({
      spacePk,
      rewardSks,
      rewardDistributed,
    }: {
      spacePk: string;
      rewardSks: string[];
      rewardDistributed: boolean;
    }) => {
      return call<UpdateSpaceDaoRewardRequest, void>(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/reward`,
        { reward_sks: rewardSks, reward_distributed: rewardDistributed },
      );
    },
    onSuccess: async (_, { spacePk }) => {
      await qc.invalidateQueries({
        queryKey: spaceDaoKeys.rewardBase(spacePk),
      });
    },
  });
}
