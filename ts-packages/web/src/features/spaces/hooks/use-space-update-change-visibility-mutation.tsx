import { spaceKeys } from '@/constants';
import { updateSpaceChangeVisibility } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Space } from '../types/space';

export function useSpaceUpdateChangeVisibilityMutation<T extends Space>() {
  const qc = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['update-change-visibility-space'],
    mutationFn: async ({
      spacePk,
      changeVisibility,
    }: {
      spacePk: string;
      changeVisibility: boolean;
    }) => {
      await updateSpaceChangeVisibility(spacePk, changeVisibility);
    },
    onSuccess: async (_, { spacePk, changeVisibility }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.change_visibility = changeVisibility;
        return space;
      });
      await qc.invalidateQueries({ queryKey: spaceQK });
    },
  });

  return mutation;
}
