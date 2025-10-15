import { spaceKeys } from '@/constants';
import { updateSpaceVisibility } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import {
  SpaceCommon,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import { useMutation } from '@tanstack/react-query';

export function useUpdateSpaceVisibilityMutation<T extends SpaceCommon>() {
  const mutation = useMutation({
    mutationKey: ['update-space-visibility'],
    mutationFn: async ({
      spacePk,
      visibility,
    }: {
      spacePk: string;
      visibility: SpaceVisibility;
    }) => {
      await updateSpaceVisibility(spacePk, visibility);
    },
    onSuccess: async (_, { spacePk, visibility }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.visibility = visibility;
        return space;
      });
    },
  });

  return mutation;
}
