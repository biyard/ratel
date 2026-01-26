import { useMutation, useQueryClient } from '@tanstack/react-query';
import { QK_USERS_GET_INFO, teamKeys } from '@/constants';
import { call } from '@/lib/api/ratel/call';

export interface CreateTeamRequest {
  username: string;
  nickname: string;
  profile_url: string;
  description: string;
}

export interface CreateTeamResponse {
  team_pk: string;
}

async function createTeam(
  request: CreateTeamRequest,
): Promise<CreateTeamResponse> {
  return await call('POST', '/v3/teams', request);
}

export function useCreateTeam() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (
      request: CreateTeamRequest,
    ): Promise<CreateTeamResponse> => {
      return await createTeam(request);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [...teamKeys.all, QK_USERS_GET_INFO],
      });
    },
  });
}
