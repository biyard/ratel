import { spaceKeys } from '@/constants';
import { updateSpaceTitle } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { Space } from '../types/Space';

export function useSpaceUpdateTitleMutation<T extends Space>() {
  const mutation = useMutation({
    mutationKey: ['update-content-space'],
    mutationFn: async ({
      spacePk,
      title,
    }: {
      spacePk: string;
      title: string;
    }) => {
      await updateSpaceTitle(spacePk, title);
    },
    onSuccess: async (_, { spacePk, title }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.title = title;
        return space;
      });
    },
  });

  return mutation;
}
