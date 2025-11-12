import { spaceKeys } from '@/constants';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpaceCommon, SpaceStatus } from '@/features/spaces/types/space-common';
import { useMutation } from '@tanstack/react-query';
import { call } from '@/lib/api/ratel/call';

export function useStartSpaceMutation<T extends SpaceCommon>() {
  const mutation = useMutation({
    mutationKey: ['start-space'],
    mutationFn: async ({
      spacePk,
      block,
    }: {
      spacePk: string;
      block?: boolean;
    }) => {
      call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
        start: true,
        block_participate: block ?? false,
      });
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.status = SpaceStatus.Started;
        return space;
      });
    },
  });

  return mutation;
}
