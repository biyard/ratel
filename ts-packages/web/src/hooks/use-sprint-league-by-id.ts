import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { useMutation } from '@tanstack/react-query';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { getQueryClient } from '@/providers/getQueryClient';
import { showErrorToast } from '@/lib/toast';
import {
  SprintLeague,
  UpdateSprintLeaguePlayerRequest,
} from '@/lib/api/models/sprint_league';
import { getQueryKey as getSpaceByIdQk } from './use-space-by-id';
import { Space } from '@/lib/api/models/spaces';
import { useReferralInfo } from '@/app/_providers/referral-handler';
import { useSprintLeagueStore } from '@/app/spaces/[id]/sprint-league/sprint-league-store';

export async function voteSprintLeague(
  spaceId: number,
  sprintLeagueId: number,
  playerId: number,
  referralCode: string | null = null,
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
          referral_code: referralCode,
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
  const { code } = useReferralInfo();
  const { updatePlayer } = useSprintLeagueStore();
  const queryClient = getQueryClient();
  const queryKey = getSpaceByIdQk(spaceId);

  const votePlayerMutation = useMutation({
    mutationFn: async ({
      sprintLeagueId,
      playerId,
    }: {
      sprintLeagueId: number;
      playerId: number;
    }) => {
      const { data } = await voteSprintLeague(
        spaceId,
        sprintLeagueId,
        playerId,
        code,
      );
      if (!data) {
        throw new Error('Vote failed.');
      }
      return { playerId };
    },
    onSuccess: (data) => {
      const { playerId } = data;
      queryClient.setQueryData(queryKey, (oldData: Space) => {
        if (!oldData) return oldData;

        const prevSprintLeague = oldData.sprint_leagues?.[0];
        if (!prevSprintLeague) return oldData;

        const updatedPlayers =
          prevSprintLeague.players?.map((player) => {
            if (player.id === playerId) {
              const newPlayer = {
                ...player,
                votes: (player.votes || 0) + 1,
              };
              updatePlayer(playerId, newPlayer);
              return newPlayer;
            }
            return player;
          }) || [];

        return {
          ...oldData,
          sprint_leagues: [
            {
              ...prevSprintLeague,
              is_voted: true,
              players: updatedPlayers,
            },
          ],
        };
      });
    },
    onError: (error) => {
      showErrorToast(error.message || 'Failed to vote sprint league');
    },
  });

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
      if (!data) {
        throw new Error('Update sprint league player failed.');
      }
      return { req, playerId };
    },
    onSuccess: (data) => {
      // queryClient.invalidateQueries({ queryKey });
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
    },
    onError: (error) => {
      showErrorToast(error.message || 'Failed to update sprint league player');
    },
  });
  return { updatePlayer: updatePlayerMutation, votePlayer: votePlayerMutation };
}
