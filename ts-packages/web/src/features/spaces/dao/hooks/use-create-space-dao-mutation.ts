import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { spaceDaoKeys } from '@/constants';

export type CreateSpaceDaoRequest = {
  contract_address: string;
  deploy_block: number;
  require_pre_survey?: boolean;
  require_post_survey?: boolean;
};

export type SpaceDaoResponse = {
  contract_address: string;
  deploy_block: number;
  require_pre_survey?: boolean;
  require_post_survey?: boolean;
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
    onSuccess: async (data, { spacePk }) => {
      qc.setQueryData(spaceDaoKeys.dao(spacePk), data);
      await qc.invalidateQueries({ queryKey: spaceDaoKeys.dao(spacePk) });
    },
  });
}
