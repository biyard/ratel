import { spaceKeys } from '@/constants';
import { updateSpaceContent } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { Space } from '../types/Space';

export function useSpaceUpdateContentMutation<T extends Space>() {
  const mutation = useMutation({
    mutationKey: ['update-content-space'],
    mutationFn: async ({
      spacePk,
      content,
    }: {
      spacePk: string;
      content: string;
    }) => {
      await updateSpaceContent(spacePk, content);
    },
    onSuccess: async (_, { spacePk, content }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.content = content;
        return space;
      });
    },
  });

  return mutation;
}
