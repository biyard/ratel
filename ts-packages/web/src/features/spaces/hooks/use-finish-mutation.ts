import { spaceDaoKeys, spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpaceCommon, SpaceStatus } from '@/features/spaces/types/space-common';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { getKaiaSigner } from '@/lib/service/kaia-wallet-service';
import { config } from '@/config';

type SpaceDaoCandidateResponse = {
  user_pk: string;
  username: string;
  display_name: string;
  profile_url: string;
  evm_address: string;
};

type SpaceDaoCandidateListResponse = {
  dao_address: string | null;
  candidates: SpaceDaoCandidateResponse[];
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
      const res = await call<void, SpaceDaoCandidateListResponse>(
        'GET',
        `/v3/spaces/${encodeURIComponent(spacePk)}/dao/candidates`,
      );
      const daoAddress = res?.dao_address ?? null;
      const candidates = res?.candidates ?? [];

      if (daoAddress && candidates.length > 0) {
        const signer = await getKaiaSigner(
          config.env === 'prod' ? 'mainnet' : 'testnet',
        );
        const provider = signer.provider;
        const service = new SpaceDaoService(provider);
        await service.connectWallet();
        await service.sampleCandidates(
          daoAddress,
          candidates.map((item) => item.evm_address),
        );
        const sampledAddresses = await service.getSampledAddresses(daoAddress);
        if (sampledAddresses.length > 0) {
          await call(
            'POST',
            `/v3/spaces/${encodeURIComponent(spacePk)}/dao/samples`,
            { sampled_addresses: sampledAddresses },
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
        queryKey: spaceDaoKeys.candidates(spacePk),
      });
      await queryClient.invalidateQueries({
        queryKey: spaceDaoKeys.samplesBase(spacePk),
      });
    },
  });

  return mutation;
}
