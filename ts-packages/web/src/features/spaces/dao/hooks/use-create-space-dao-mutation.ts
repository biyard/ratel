import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceKeys } from '@/constants';

export type CreateSpaceDaoRequest = {
  contract_address: string;
  sampling_count: number;
  reward_amount: number;
};

export type SpaceDaoResponse = {
  contract_address: string;
  sampling_count: number;
  reward_amount: number;
};

export function useCreateSpaceDaoMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-space-dao'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: CreateSpaceDaoRequest;
    }) => {
      return call<CreateSpaceDaoRequest, SpaceDaoResponse>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao`,
        req,
      );
    },
    onSettled: async (_data, _error, { spacePk }) => {
      await qc.invalidateQueries({ queryKey: spaceKeys.detail(spacePk) });
    },
  });
}
