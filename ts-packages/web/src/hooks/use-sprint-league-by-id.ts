import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { useMutation } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import {
  SprintLeague,
  UpdateSprintLeaguePlayerRequest,
} from '@/lib/api/models/sprint_league';
import { getQueryKey as getSpaceByIdQk } from './use-space-by-id';
import { Space } from '@/lib/api/models/spaces';

export async function voteSprintLeague(
  spaceId: number,
  sprintLeagueId: number,
  playerId: number,
): Promise<FetchResponse<SprintLeague | null>> {
  return apiFetch<SprintLeague | null>(
    `${config.api_url}${ratelApi.sprint_league.voteSprintLeague(spaceId, sprintLeagueId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        vote: {
          player_id: playerId,
        },
      }),
    },
  );
}

export async function updateSprintLeaguePlayer(
  spaceId: number,
  sprintLeagueId: number,
  playerId: number,
  req: UpdateSprintLeaguePlayerRequest,
): Promise<FetchResponse<SprintLeague | null>> {
  return apiFetch<SprintLeague | null>(
    `${config.api_url}${ratelApi.sprint_league.updateSprintLeaguePlayer(spaceId, sprintLeagueId, playerId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(req),
    },
  );
}

export function useSprintLeagueSpaceByIdMutation(spaceId: number) {
  const queryClient = getQueryClient();
  const queryKey = getSpaceByIdQk(spaceId);

  const updatePlayerMutation = useMutation({
    mutationFn: async ({
      sprintLeagueId,
      playerId,
      req,
    }: {
      sprintLeagueId: number;
      playerId: number;
      req: UpdateSprintLeaguePlayerRequest;
    }) => {
      const { data } = await updateSprintLeaguePlayer(
        spaceId,
        sprintLeagueId,
        playerId,
        req,
      );
      console.log('Called');
      if (!data) {
        throw new Error('Update sprint league player failed.');
      }
      return { req, playerId };
    },
    onSuccess: (data) => {
      const { req, playerId } = data;
      queryClient.setQueryData(queryKey, (oldData: Space) => {
        if (!oldData) return oldData;

        const prevSprintLeague = oldData.sprint_leagues?.[0];
        if (!prevSprintLeague) return oldData;

        const updatedPlayers =
          prevSprintLeague.players?.map((player) => {
            if (player.id === playerId) {
              return {
                ...player,
                ...req,
              };
            }
            return player;
          }) || [];

        return {
          ...oldData,
          sprint_leagues: [
            {
              ...prevSprintLeague,
              players: updatedPlayers,
            },
          ],
        };
      });

      showSuccessToast('Update successful');
    },
    onError: (error) => {
      showErrorToast(error.message || 'Failed to update sprint league player');
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey,
        exact: true,
      });
    },
  });
  return { updatePlayer: updatePlayerMutation };
}
