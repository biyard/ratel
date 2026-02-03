import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type UpdateSpaceDaoSelectedRequest = {
  selected_sks: string[];
  reward_distributed: boolean;
};

export function useUpdateSpaceDaoSelectedMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['update-space-dao-selected'],
    mutationFn: async ({
      spacePk,
      selectedSks,
      rewardDistributed,
    }: {
      spacePk: string;
      selectedSks: string[];
      rewardDistributed: boolean;
    }) => {
      return call<UpdateSpaceDaoSelectedRequest, void>(
        'PATCH',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/selected`,
        { selected_sks: selectedSks, reward_distributed: rewardDistributed },
      );
    },
    onSuccess: async (_, { spacePk }) => {
      await qc.invalidateQueries({
        queryKey: spaceDaoKeys.selectedBase(spacePk),
      });
    },
  });
}
