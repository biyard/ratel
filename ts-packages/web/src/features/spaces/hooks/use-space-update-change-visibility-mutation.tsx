import { spaceKeys } from '@/constants';
import { useMutation, useQueryClient } from '@tanstack/react-query';

// FIXME: USE Space UPDATE API
export function useSpaceUpdateChangeVisibilityMutation() {
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
      throw new Error('Not implemented');
    },
    onSuccess: async (_, { spacePk, changeVisibility }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await qc.invalidateQueries({ queryKey: spaceQK });
    },
  });

  return mutation;
}
