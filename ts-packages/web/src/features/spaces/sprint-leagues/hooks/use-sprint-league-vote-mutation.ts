import { useMutation } from '@tanstack/react-query';
import { spaceKeys } from '@/constants';
import { showErrorToast } from '@/lib/toast';
import { optimisticUpdate } from '@/lib/hook-utils';

import SprintLeague from '../types/sprint-league';
import voteSprintLeague from '../api/vote-sprint-league';

export function useVoteSprintLeagueMutation() {
  return useMutation({
    mutationFn: async ({
      spacePk,
      playerSk,
      referralCode,
    }: {
      spacePk: string;
      playerSk: string;
      referralCode?: string;
    }) => {
      return await voteSprintLeague(spacePk, playerSk, referralCode);
    },

    onMutate: async ({ spacePk, playerSk }) => {
      const prevSprintLeague = await optimisticUpdate<SprintLeague>(
        { queryKey: spaceKeys.sprint_leagues(spacePk) },
        (sprintLeague) => {
          return {
            ...sprintLeague!,
            is_voted: true,
            players: sprintLeague!.players.map((player) =>
              player.sk === playerSk
                ? { ...player, voted: player.votes + 1 }
                : player,
            ),
            votes: sprintLeague.votes + 1,
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
