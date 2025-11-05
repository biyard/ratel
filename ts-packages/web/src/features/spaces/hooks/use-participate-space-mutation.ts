import { spaceKeys } from '@/constants';
import { participateSpace } from '@/lib/api/ratel/spaces.v3';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export function useParticipateSpaceMutation() {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationKey: ['participate-space'],
    mutationFn: async ({
      spacePk,
      verifiablePresentation,
    }: {
      spacePk: string;
      verifiablePresentation: string;
    }) => {
      return await participateSpace(spacePk, verifiablePresentation);
    },
    onSuccess: async (_, { spacePk }) => {
      // Invalidate space details to refetch with updated participant status
      const spaceQK = spaceKeys.detail(spacePk);
      await queryClient.invalidateQueries({ queryKey: spaceQK });
    },
  });

  return mutation;
}
