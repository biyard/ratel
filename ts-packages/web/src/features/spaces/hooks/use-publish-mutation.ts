import { spaceKeys } from '@/constants';
import { publishSpace } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import {
  SpacePublishState,
  SpaceVisibility,
} from '@/features/spaces/types/space-common';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Space } from '../types/space';

export function usePublishSpaceMutation() {
  const qc = useQueryClient();

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
    onMutate: async ({ spacePk, visibility }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      const rollback = await optimisticUpdate<Space>(
        { queryKey: spaceQK },
        (space) => {
          space.publishState = SpacePublishState.Published;
          space.visibility = visibility;
          return space;
        },
      );

      return { rollback };
    },
    onSuccess: async (_, { spacePk }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await qc.invalidateQueries({ queryKey: spaceQK });
    },
    onError: async (_, __, context) => {
      context.rollback?.rollback();
    },
  });

  return mutation;
}
