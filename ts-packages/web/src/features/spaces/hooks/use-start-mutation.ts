import { spaceKeys } from '@/constants';
import { startSpace } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { SpaceCommon, SpaceStatus } from '@/features/spaces/types/space-common';
import { useMutation } from '@tanstack/react-query';

export function useStartSpaceMutation<T extends SpaceCommon>() {
  const mutation = useMutation({
    mutationKey: ['start-space'],
    mutationFn: async ({ spacePk }: { spacePk: string }) => {
      await startSpace(spacePk);
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
