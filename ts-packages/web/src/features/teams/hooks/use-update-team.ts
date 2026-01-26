import { useMutation, useQueryClient } from '@tanstack/react-query';
import { teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';
import { Team } from '../types/team';

export interface UpdateTeamRequest {
  nickname?: string;
  description?: string;
  profile_url?: string;
  dao_address?: string;
}

async function updateTeam(
  teamPk: string,
  request: UpdateTeamRequest,
): Promise<Team> {
  return await call(
    'PATCH',
    `/v3/teams/${encodeURIComponent(teamPk)}`,
    request,
  );
}

export function useUpdateTeam() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      teamPk,
      request,
    }: {
      teamPk: string;
      request: UpdateTeamRequest;
    }): Promise<Team> => {
      return await updateTeam(teamPk, request);
    },
    onSuccess: (data, variables) => {
      queryClient.invalidateQueries({
        queryKey: teamKeys.detail(variables.teamPk),
      });
      queryClient.invalidateQueries({ queryKey: teamKeys.all });
    },
  });
}
