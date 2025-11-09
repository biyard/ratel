import { spaceKeys } from '@/constants';
import { updateSpaceAnonymousParticipation } from '@/lib/api/ratel/spaces.v3';
import { optimisticUpdate } from '@/lib/hook-utils';
import { useMutation } from '@tanstack/react-query';
import { Space } from '../types/space';

export function useSpaceUpdateAnonymousParticipationMutation<
  T extends Space,
>() {
  const mutation = useMutation({
    mutationKey: ['update-anonymous-participation-space'],
    mutationFn: async ({
      spacePk,
      anonymousParticipation,
    }: {
      spacePk: string;
      anonymousParticipation: boolean;
    }) => {
      await updateSpaceAnonymousParticipation(spacePk, anonymousParticipation);
    },
    onSuccess: async (_, { spacePk, anonymousParticipation }) => {
      const spaceQK = spaceKeys.detail(spacePk);
      await optimisticUpdate<T>({ queryKey: spaceQK }, (space) => {
        space.anonymous_participation = anonymousParticipation;
        return space;
      });
    },
  });

  return mutation;
}
