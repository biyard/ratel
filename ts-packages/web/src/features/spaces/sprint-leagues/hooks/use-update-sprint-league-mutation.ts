import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';

import updateSprintLeague from '../api/upsert-sprint-league';
import SprintLeague from '../types/sprint-league';
import CreateSprintLeaguePlayer from '../types/create-sprint-league-player';

export function useUpdateSprintLeagueMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      players,
    }: {
      spacePk: string;
      players: CreateSprintLeaguePlayer[];
    }) => {
      console.log('Updating sprint league with players:', players);
      return await updateSprintLeague(spacePk, players);
    },

    onMutate: async ({ spacePk, players }) => {
      const prevSprintLeague = await optimisticUpdate<SprintLeague>(
        { queryKey: spaceKeys.sprint_leagues(spacePk) },
        (sprintLeague) => {
          const nextPlayers = sprintLeague.players.map((p, index) => ({
            ...p,
            player_image: players[index]?.player_image || null,
            name: players[index]?.name || '',
            description: players[index]?.description || '',
          }));
          return {
            ...sprintLeague!,
            players: nextPlayers,
          };
        },
      );

      return { prevSprintLeague };
    },

    onError: (error: Error, _variables, context) => {
      context?.prevSprintLeague?.rollback();

      showErrorToast(error.message || 'Failed to update sprint league');
    },

    onSuccess: async (data, { spacePk }) => {
      await optimisticUpdate<SprintLeague>(
        { queryKey: spaceKeys.sprint_leagues(spacePk) },
        (_) => {
          return data;
        },
      );
    },
  });
}
