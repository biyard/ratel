import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';

import SprintLeaguePlayer from '../types/sprint-league-player';
import updateSprintLeague from '../api/upsert-sprint-league';
import SprintLeague from '../types/sprint-league';

export function useUpdateSprintLeagueMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      players,
    }: {
      spacePk: string;
      players: SprintLeaguePlayer[];
    }) => {
      return await updateSprintLeague(spacePk, players);
    },

    onMutate: async ({ spacePk, players }) => {
      const prevSprintLeague = await optimisticUpdate<SprintLeague>(
        { queryKey: spaceKeys.sprint_leagues(spacePk) },
        (sprintLeague) => {
          return {
            ...sprintLeague!,
            players,
          };
        },
      );

      return { prevSprintLeague };
    },

    onError: (error: Error, _variables, context) => {
      context?.prevSprintLeague?.rollback();

      showErrorToast(error.message || 'Failed to update sprint league');
    },

    onSettled: () => {},
  });
}
