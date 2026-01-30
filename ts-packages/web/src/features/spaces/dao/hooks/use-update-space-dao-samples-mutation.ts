import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type UpdateSpaceDaoSamplesRequest = {
  sample_sks: string[];
  reward_distributed: boolean;
};

export function useUpdateSpaceDaoSamplesMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-dao-samples'],
    mutationFn: async ({
      spacePk,
      sampleSks,
      rewardDistributed,
    }: {
      spacePk: string;
      sampleSks: string[];
      rewardDistributed: boolean;
    }) => {
      return call<UpdateSpaceDaoSamplesRequest, void>(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/samples`,
        { sample_sks: sampleSks, reward_distributed: rewardDistributed },
      );
    },
    onSuccess: async (_, { spacePk }) => {
      await qc.invalidateQueries({
        queryKey: spaceDaoKeys.samplesBase(spacePk),
      });
    },
  });
}
