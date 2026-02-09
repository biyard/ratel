import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceIncentiveKeys } from '@/constants';

export type CreateSpaceIncentiveRequest = {
  contract_address: string;
  deploy_block: number;
};

export type SpaceIncentiveResponse = {
  contract_address: string;
  deploy_block: number;
};

export function useCreateSpaceIncentiveMutation() {
  const qc = useQueryClient();

  return useMutation({
    mutationKey: ['create-space-incentive'],
    mutationFn: async ({
      spacePk,
      req,
    }: {
      spacePk: string;
      req: CreateSpaceIncentiveRequest;
    }) => {
      return call<CreateSpaceIncentiveRequest, SpaceIncentiveResponse>(
        'POST',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives`,
        req,
      );
    },
    onSuccess: async (data, { spacePk }) => {
      qc.setQueryData(spaceIncentiveKeys.incentive(spacePk), data);
      await qc.invalidateQueries({
        queryKey: spaceIncentiveKeys.incentive(spacePk),
      });
    },
  });
}
