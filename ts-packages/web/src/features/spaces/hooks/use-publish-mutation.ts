import { spaceKeys } from '@/constants';
import { publishSpace } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import {
  SpaceCommon,
  SpacePublishState,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import { useMutation } from '@tanstack/react-query';

export function usePublishSpaceMutation<T extends SpaceCommon>() {
  const mutation = useMutation({
    mutationKey: ['publish-space'],
    mutationFn: async ({
      spacePk,
      visibility,
    }: {
      spacePk: string;
      visibility: SpaceVisibility;
    }) => {
      await publishSpace(spacePk, visibility);
    },
    onSuccess: async (_, { spacePk, visibility }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.publish_state = SpacePublishState.Published;
        space.visibility = visibility;
        return space;
      });
    },
  });

  return mutation;
}
