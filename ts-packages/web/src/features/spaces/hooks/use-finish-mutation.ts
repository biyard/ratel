import { spaceIncentiveKeys, spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpaceCommon, SpaceStatus } from '@/features/spaces/types/space-common';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpaceIncentiveService } from '@/contracts/SpaceIncentiveService';
import { getKaiaSigner } from '@/lib/service/kaia-wallet-service';
import { config } from '@/config';

type SpaceIncentiveCandidateResponse = {
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
  score?: number;
};

type SpaceIncentiveCandidateListResponse = {
  incentive_address: string | null;
  candidates: SpaceIncentiveCandidateResponse[];
};

export function useFinishSpaceMutation<T extends SpaceCommon>() {
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationKey: ['end-space'],
    mutationFn: async ({
      spacePk,
      block,
    }: {
      spacePk: string;
      block?: boolean;
    }) => {
      const res = await call<void, SpaceIncentiveCandidateListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/candidates`,
      );
      const incentiveAddress = res?.incentive_address ?? null;
      const candidates = res?.candidates ?? [];

      if (incentiveAddress && candidates.length > 0) {
        const signer = await getKaiaSigner(
          config.env === 'prod' ? 'mainnet' : 'testnet',
        );
        const provider = signer.provider;
        const service = new SpaceIncentiveService(provider);
        await service.connectWallet();
        const scores = candidates.map((item) =>
          typeof item.score === 'number' ? item.score : 0,
        );
        await service.selectIncentiveRecipients(
          incentiveAddress,
          candidates.map((item) => item.evm_address),
          scores,
        );
        const selectedAddresses =
          await service.getIncentiveRecipients(incentiveAddress);
        if (selectedAddresses.length > 0) {
          await call(
            'POST',
            `/v3/spaces/${encodeURIComponent(spacePk)}/incentives/user`,
            { incentive_addresses: selectedAddresses },
          );
        }
      }

      await call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
        finished: true,
        block_participate: block ?? false,
      });
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.status = SpaceStatus.Finished;
        return space;
      });
      await queryClient.invalidateQueries({
        queryKey: spaceIncentiveKeys.candidates(spacePk),
      });
      await queryClient.invalidateQueries({
        queryKey: spaceIncentiveKeys.incentiveBase(spacePk),
      });
    },
  });

  return mutation;
}
